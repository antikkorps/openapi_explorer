pub mod layout;
pub mod fields;
pub mod schemas;
pub mod endpoints;
pub mod graph;

use crate::app::{App, Panel, View};
use crossterm::{
    event::{self, DisableMouseCapture, EnableMouseCapture, Event, KeyCode, KeyEventKind},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use ratatui::{
    backend::CrosstermBackend,
    layout::{Constraint, Direction, Layout},
    style::{Color, Modifier, Style},
    text::{Line, Span},
    widgets::{Block, Borders, Clear, List, ListItem, Paragraph, Wrap},
    Frame, Terminal,
};
use std::io;
use std::time::{Duration, Instant};

pub async fn run(app: &mut App) -> Result<(), Box<dyn std::error::Error>> {
    // Setup terminal
    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen, EnableMouseCapture)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    let mut last_tick = Instant::now();
    let tick_rate = Duration::from_millis(250);

    loop {
        // Handle reload request
        if app.should_reload {
            app.should_reload = false;
            match app.reload().await {
                Ok(_) => {
                    // Reload successful - the app state is updated
                }
                Err(_) => {
                    // Error is stored in app.reload_error
                }
            }
        }

        // Handle input
        if event::poll(tick_rate - last_tick.elapsed())? {
            if let Event::Key(key) = event::read()? {
                if key.kind == KeyEventKind::Press {
                    handle_key_events(key, app);
                }
            }
        }

        if last_tick.elapsed() >= tick_rate {
            last_tick = Instant::now();
        }

        // Render UI
        terminal.draw(|f| ui(f, app))?;

        if app.should_quit {
            break;
        }
    }

    // Restore terminal
    disable_raw_mode()?;
    execute!(
        terminal.backend_mut(),
        LeaveAlternateScreen,
        DisableMouseCapture
    )?;
    terminal.show_cursor()?;

    Ok(())
}

fn ui(f: &mut Frame, app: &mut App) {
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(3), // Search bar
            Constraint::Min(0),    // Main content
            Constraint::Length(3), // Status bar
        ])
        .split(f.area());

    // Search bar
    let search_text = Paragraph::new(format!("Search: {}", app.search_query))
        .style(Style::default().fg(Color::Yellow))
        .block(Block::default().borders(Borders::ALL));
    f.render_widget(search_text, chunks[0]);

    // Main content area
    let main_chunks = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([Constraint::Percentage(30), Constraint::Percentage(40), Constraint::Percentage(30)])
        .split(chunks[1]);

    match app.current_view {
        View::Fields => fields::render_fields_view(f, app, main_chunks.to_vec()),
        View::Schemas => schemas::render_schemas_view(f, app, main_chunks.to_vec()),
        View::Endpoints => endpoints::render_endpoints_view(f, app, main_chunks.to_vec()),
        View::Graph => graph::render_graph_view(f, app, main_chunks.to_vec()),
        View::Stats => render_stats_view(f, app, main_chunks.to_vec()),
    }

    // Status bar
    let mut status_text = vec![
        Span::styled("h:Help", Style::default().fg(Color::Cyan)),
        Span::raw("  "),
        Span::styled("r:Reload", Style::default().fg(Color::Cyan)),
        Span::raw("  "),
        Span::styled("q:Quit", Style::default().fg(Color::Red)),
        Span::raw("  "),
        Span::styled(format!("View: {:?}", app.current_view), Style::default().fg(Color::Green)),
        Span::raw("  "),
        Span::styled(format!("Panel: {:?}", app.current_panel), Style::default().fg(Color::Green)),
    ];

    // Add loading, reload status or error message
    if app.is_loading && !app.loading_message.is_empty() {
        status_text.push(Span::raw("  "));
        status_text.push(Span::styled(
            format!("⟳ {}", app.loading_message),
            Style::default().fg(Color::Yellow).add_modifier(Modifier::BOLD)
        ));
    } else if app.should_reload {
        status_text.push(Span::raw("  "));
        status_text.push(Span::styled("⟳ Reloading...", Style::default().fg(Color::Yellow)));
    } else if let Some(error) = &app.reload_error {
        status_text.push(Span::raw("  "));
        status_text.push(Span::styled(format!("✗ {}", error), Style::default().fg(Color::Red)));
    }

    let status_bar = Paragraph::new(Line::from(status_text))
        .style(Style::default().bg(Color::DarkGray))
        .block(Block::default().borders(Borders::ALL));
    f.render_widget(status_bar, chunks[2]);

    // Help popup
    if app.show_help {
        render_help_popup(f);
    }

    // Endpoint details popup
    if app.show_endpoint_details && app.selected_endpoint_for_details.is_some() {
        render_endpoint_details_popup(f, app);
    }
}

fn render_stats_view(f: &mut Frame, app: &App, chunks: Vec<ratatui::layout::Rect>) {
    // Calculate statistics
    let total_schemas = app.field_index.schemas.len();
    let total_fields = app.field_index.fields.len();
    let total_endpoints = app.openapi_spec.paths.len();

    // Count field types
    let mut type_counts: std::collections::HashMap<String, usize> = std::collections::HashMap::new();
    for field_data in app.field_index.fields.values() {
        *type_counts.entry(field_data.field_type.clone()).or_insert(0) += 1;
    }

    // Count HTTP methods
    let mut method_counts: std::collections::HashMap<String, usize> = std::collections::HashMap::new();
    for path_item in app.openapi_spec.paths.values() {
        for method in path_item.operations.keys() {
            *method_counts.entry(method.to_uppercase()).or_insert(0) += 1;
        }
    }

    // Count critical fields
    let critical_fields = app.field_index.fields.values()
        .filter(|f| !f.endpoints.is_empty() &&
                f.endpoints.iter().any(|e| e.to_lowercase().contains("post") || e.to_lowercase().contains("put")))
        .count();

    // Find most used fields
    let mut field_usage: Vec<(&String, usize)> = app.field_index.fields.iter()
        .map(|(name, data)| (name, data.endpoints.len()))
        .collect();
    field_usage.sort_by(|a, b| b.1.cmp(&a.1));

    // Build stats text
    let mut stats_text = vec![
        Line::from(vec![
            Span::styled("📊 OpenAPI Statistics", Style::default().fg(Color::Cyan).add_modifier(Modifier::BOLD)),
        ]),
        Line::from(""),
        Line::from(vec![
            Span::styled("Overview", Style::default().fg(Color::Yellow).add_modifier(Modifier::UNDERLINED)),
        ]),
        Line::from(format!("  • Schemas: {}", total_schemas)),
        Line::from(format!("  • Fields: {}", total_fields)),
        Line::from(format!("  • Endpoints: {}", total_endpoints)),
        Line::from(format!("  • Critical Fields: {} ({:.1}%)",
            critical_fields,
            (critical_fields as f64 / total_fields.max(1) as f64) * 100.0)),
        Line::from(""),
    ];

    // Field types distribution
    if !type_counts.is_empty() {
        stats_text.push(Line::from(vec![
            Span::styled("Field Types", Style::default().fg(Color::Yellow).add_modifier(Modifier::UNDERLINED)),
        ]));
        let mut types: Vec<_> = type_counts.iter().collect();
        types.sort_by(|a, b| b.1.cmp(a.1));
        for (field_type, count) in types.iter().take(5) {
            let percentage = (*count as f64 / total_fields as f64) * 100.0;
            stats_text.push(Line::from(format!("  • {}: {} ({:.1}%)", field_type, count, percentage)));
        }
        stats_text.push(Line::from(""));
    }

    // HTTP methods distribution
    if !method_counts.is_empty() {
        stats_text.push(Line::from(vec![
            Span::styled("HTTP Methods", Style::default().fg(Color::Yellow).add_modifier(Modifier::UNDERLINED)),
        ]));
        let mut methods: Vec<_> = method_counts.iter().collect();
        methods.sort_by(|a, b| b.1.cmp(a.1));
        for (method, count) in methods.iter() {
            let color = match method.as_str() {
                "GET" => Color::Green,
                "POST" => Color::Blue,
                "PUT" => Color::Yellow,
                "DELETE" => Color::Red,
                _ => Color::White,
            };
            stats_text.push(Line::from(vec![
                Span::raw("  • "),
                Span::styled(format!("{}: {}", method, count), Style::default().fg(color)),
            ]));
        }
        stats_text.push(Line::from(""));
    }

    // Most used fields
    if !field_usage.is_empty() {
        stats_text.push(Line::from(vec![
            Span::styled("Top Fields (by endpoint usage)", Style::default().fg(Color::Yellow).add_modifier(Modifier::UNDERLINED)),
        ]));
        for (field_name, usage_count) in field_usage.iter().take(5) {
            if *usage_count > 0 {
                stats_text.push(Line::from(format!("  • {}: {} endpoint(s)", field_name, usage_count)));
            }
        }
        stats_text.push(Line::from(""));
    }

    // Validation warnings
    if !app.validation_warnings.is_empty() {
        stats_text.push(Line::from(vec![
            Span::styled("⚠ Validation Warnings", Style::default().fg(Color::Red).add_modifier(Modifier::UNDERLINED)),
        ]));
        for (i, warning) in app.validation_warnings.iter().enumerate().take(10) {
            stats_text.push(Line::from(vec![
                Span::styled(format!("  {}. ", i + 1), Style::default().fg(Color::Red)),
                Span::raw(warning),
            ]));
        }
        if app.validation_warnings.len() > 10 {
            stats_text.push(Line::from(vec![
                Span::styled(
                    format!("  ... and {} more warnings", app.validation_warnings.len() - 10),
                    Style::default().fg(Color::DarkGray).add_modifier(Modifier::ITALIC)
                ),
            ]));
        }
    } else {
        stats_text.push(Line::from(vec![
            Span::styled("✓ No validation warnings", Style::default().fg(Color::Green)),
        ]));
    }

    let stats_widget = Paragraph::new(stats_text)
        .block(Block::default().borders(Borders::ALL).title("Statistics Dashboard"))
        .wrap(Wrap { trim: true });
    f.render_widget(stats_widget, chunks[1]);
}

fn render_help_popup(f: &mut Frame) {
    let help_text = vec![
        Line::from(vec![
            Span::styled("OpenAPI Field Explorer - Help", Style::default().fg(Color::Cyan).add_modifier(Modifier::BOLD)),
        ]),
        Line::from(""),
        Line::from(vec![
            Span::styled("⌨  Keyboard Shortcuts", Style::default().fg(Color::Yellow).add_modifier(Modifier::UNDERLINED)),
        ]),
        Line::from(""),
        Line::from(vec![
            Span::styled("  Navigation", Style::default().fg(Color::Green)),
        ]),
        Line::from("    ↑/↓         Navigate items in current panel"),
        Line::from("    Tab         Switch between panels (Left/Center/Right)"),
        Line::from("    Enter       Select item / Show details"),
        Line::from("    Esc         Go back / Clear errors / Close help"),
        Line::from(""),
        Line::from(vec![
            Span::styled("  Views", Style::default().fg(Color::Green)),
        ]),
        Line::from("    1           Fields View (search by field name)"),
        Line::from("    2           Schemas View (browse by schema)"),
        Line::from("    3           Endpoints View (navigate endpoints)"),
        Line::from("    4           Graph View (visualize relationships)"),
        Line::from("    5           Stats View (dashboard & metrics)"),
        Line::from(""),
        Line::from(vec![
            Span::styled("  Search & Actions", Style::default().fg(Color::Green)),
        ]),
        Line::from("    /           Start typing to search (fuzzy match)"),
        Line::from("    Backspace   Delete search character"),
        Line::from("    r           Reload OpenAPI file"),
        Line::from("    h           Toggle this help screen"),
        Line::from("    q / Ctrl+C  Quit application"),
        Line::from(""),
        Line::from(vec![
            Span::styled("💡 Tips", Style::default().fg(Color::Yellow).add_modifier(Modifier::UNDERLINED)),
        ]),
        Line::from(""),
        Line::from("  • Fuzzy search: Type 'usid' to find 'USER_ID'"),
        Line::from("  • Yellow = Selected, Cyan = Cursor position"),
        Line::from("  • Critical fields (POST/PUT) shown in red"),
        Line::from("  • Press 'r' after editing OpenAPI file to reload"),
        Line::from("  • Use Tab to navigate between panels efficiently"),
        Line::from(""),
        Line::from(vec![
            Span::styled("Press 'h' or 'Esc' to close", Style::default().fg(Color::DarkGray).add_modifier(Modifier::ITALIC)),
        ]),
    ];

    let help_widget = Paragraph::new(help_text)
        .block(Block::default()
            .borders(Borders::ALL)
            .border_style(Style::default().fg(Color::Cyan))
            .title(" Help "))
        .style(Style::default().bg(Color::Black).fg(Color::White))
        .wrap(Wrap { trim: true });

    // Make the help window larger to fit all content
    let area = ratatui::layout::Rect {
        x: f.area().x + f.area().width / 6,
        y: f.area().y + 1,
        width: (f.area().width * 2) / 3,
        height: f.area().height.saturating_sub(2),
    };

    f.render_widget(Clear, area);
    f.render_widget(help_widget, area);
}

fn render_endpoint_details_popup(f: &mut Frame, app: &App) {
    if let Some(endpoint_str) = &app.selected_endpoint_for_details {
        // Parse endpoint string (format: "METHOD /path")
        let parts: Vec<&str> = endpoint_str.splitn(2, ' ').collect();
        if parts.len() != 2 {
            return;
        }

        let method = parts[0];
        let path = parts[1];

        // Find the operation in the spec
        if let Some(path_item) = app.openapi_spec.paths.get(path) {
            if let Some(operation) = path_item.operations.get(&method.to_lowercase()) {
                let mut details_text = vec![
                    Line::from(vec![
                        Span::styled(format!("{} {}", method, path),
                            Style::default().fg(Color::Cyan).add_modifier(Modifier::BOLD)),
                    ]),
                    Line::from(""),
                ];

                // Summary
                if let Some(summary) = &operation.summary {
                    details_text.push(Line::from(vec![
                        Span::styled("Summary: ", Style::default().fg(Color::Yellow)),
                        Span::raw(summary),
                    ]));
                    details_text.push(Line::from(""));
                }

                // Description
                if let Some(description) = &operation.description {
                    details_text.push(Line::from(vec![
                        Span::styled("Description: ", Style::default().fg(Color::Yellow)),
                    ]));
                    details_text.push(Line::from(format!("  {}", description)));
                    details_text.push(Line::from(""));
                }

                // Tags
                if let Some(tags) = &operation.tags {
                    if !tags.is_empty() {
                        details_text.push(Line::from(vec![
                            Span::styled("Tags: ", Style::default().fg(Color::Yellow)),
                            Span::raw(tags.join(", ")),
                        ]));
                        details_text.push(Line::from(""));
                    }
                }

                // Parameters
                if let Some(parameters) = &operation.parameters {
                    if !parameters.is_empty() {
                        details_text.push(Line::from(vec![
                            Span::styled("Parameters:", Style::default().fg(Color::Yellow).add_modifier(Modifier::UNDERLINED)),
                        ]));
                        for param in parameters {
                            let required = if param.required.unwrap_or(false) { " *" } else { "" };
                            details_text.push(Line::from(format!(
                                "  • {} ({}){} - {}",
                                param.name,
                                param.in_,
                                required,
                                param.description.as_deref().unwrap_or("No description")
                            )));
                        }
                        details_text.push(Line::from(""));
                    }
                }

                // Request body
                if let Some(request_body) = &operation.request_body {
                    details_text.push(Line::from(vec![
                        Span::styled("Request Body:", Style::default().fg(Color::Yellow).add_modifier(Modifier::UNDERLINED)),
                    ]));
                    if let Some(desc) = &request_body.description {
                        details_text.push(Line::from(format!("  {}", desc)));
                    }
                    details_text.push(Line::from(format!("  Content types: {}",
                        request_body.content.keys().map(|k| k.as_str()).collect::<Vec<_>>().join(", "))));
                    details_text.push(Line::from(""));
                }

                // Responses
                if !operation.responses.is_empty() {
                    details_text.push(Line::from(vec![
                        Span::styled("Responses:", Style::default().fg(Color::Yellow).add_modifier(Modifier::UNDERLINED)),
                    ]));
                    for (status_code, response) in &operation.responses {
                        let color = if status_code.starts_with('2') {
                            Color::Green
                        } else if status_code.starts_with('4') || status_code.starts_with('5') {
                            Color::Red
                        } else {
                            Color::Yellow
                        };
                        details_text.push(Line::from(vec![
                            Span::styled(format!("  • {}: ", status_code), Style::default().fg(color)),
                            Span::raw(&response.description),
                        ]));
                    }
                }

                details_text.push(Line::from(""));
                details_text.push(Line::from(vec![
                    Span::styled("Press 'Esc' to close", Style::default().fg(Color::DarkGray).add_modifier(Modifier::ITALIC)),
                ]));

                let details_widget = Paragraph::new(details_text)
                    .block(Block::default()
                        .borders(Borders::ALL)
                        .border_style(Style::default().fg(Color::Cyan))
                        .title(" Endpoint Details "))
                    .style(Style::default().bg(Color::Black).fg(Color::White))
                    .wrap(Wrap { trim: true });

                // Large popup to fit all details
                let area = ratatui::layout::Rect {
                    x: f.area().x + f.area().width / 8,
                    y: f.area().y + 1,
                    width: (f.area().width * 3) / 4,
                    height: f.area().height.saturating_sub(2),
                };

                f.render_widget(Clear, area);
                f.render_widget(details_widget, area);
            }
        }
    }
}

fn handle_key_events(key: crossterm::event::KeyEvent, app: &mut App) {
    match key.code {
        KeyCode::Char('q') | KeyCode::Char('c') if key.modifiers.contains(crossterm::event::KeyModifiers::CONTROL) => {
            app.should_quit = true;
        }
        KeyCode::Char('q') => {
            app.should_quit = true;
        }
        KeyCode::Tab => {
            app.next_panel();
        }
        KeyCode::Char('/') => {
            app.search_query.clear();
            app.update_filters();
        }
        KeyCode::Char('h') => {
            app.show_help = !app.show_help;
        }
        KeyCode::Char('1') => {
            app.set_view(View::Fields);
        }
        KeyCode::Char('2') => {
            app.set_view(View::Schemas);
        }
        KeyCode::Char('3') => {
            app.set_view(View::Endpoints);
        }
        KeyCode::Char('4') => {
            app.set_view(View::Graph);
        }
        KeyCode::Char('5') => {
            app.set_view(View::Stats);
        }
        KeyCode::Char('r') => {
            app.request_reload();
        }
        KeyCode::Esc => {
            app.show_help = false;
            app.show_endpoint_details = false;
            app.selected_endpoint_for_details = None;
            app.reload_error = None; // Clear reload error on Esc
        }
        KeyCode::Char(ch) => {
            if !app.search_query.is_empty() || ch == '/' {
                if ch != '/' {
                    app.search_query.push(ch);
                    app.update_filters();
                }
            }
        }
        KeyCode::Backspace => {
            if !app.search_query.is_empty() {
                app.search_query.pop();
                app.update_filters();
            }
        }
        KeyCode::Up => {
            if !app.show_help {
                app.navigate_up();
            }
        }
        KeyCode::Down => {
            if !app.show_help {
                app.navigate_down();
            }
        }
        KeyCode::Enter => {
            if !app.show_help {
                app.select_current_item();
            }
        }
        _ => {}
    }
}