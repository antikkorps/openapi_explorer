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

    // Add reload status or error message
    if app.should_reload {
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
}

fn render_stats_view(f: &mut Frame, app: &App, chunks: Vec<ratatui::layout::Rect>) {
    let stats_text = vec![
        Line::from("OpenAPI Statistics"),
        Line::from(""),
        Line::from(format!("Total Schemas: {}", app.field_index.schemas.len())),
        Line::from(format!("Total Fields: {}", app.field_index.fields.len())),
        Line::from(format!("Total Endpoints: {}", app.openapi_spec.paths.len())),
        Line::from(""),
        Line::from("Field Distribution:"),
        // Add more statistics here
    ];

    let stats_widget = Paragraph::new(stats_text)
        .block(Block::default().borders(Borders::ALL).title("Statistics"));
    f.render_widget(stats_widget, chunks[1]);
}

fn render_help_popup(f: &mut Frame) {
    let help_text = vec![
        Line::from("Keyboard Shortcuts:"),
        Line::from(""),
        Line::from("q/Ctrl+C  : Quit"),
        Line::from("Tab        : Change panel"),
        Line::from("/          : Search mode"),
        Line::from("Enter      : Select/View details"),
        Line::from("Esc        : Back/Cancel"),
        Line::from("1-5        : Change view"),
        Line::from("r          : Reload file"),
        Line::from("h          : Toggle help"),
        Line::from("↑↓         : Navigate"),
    ];

    let help_widget = Paragraph::new(help_text)
        .block(Block::default().borders(Borders::ALL).title("Help"))
        .style(Style::default().bg(Color::Black).fg(Color::White));

    let area = ratatui::layout::Rect {
        x: f.area().x + f.area().width / 4,
        y: f.area().y + f.area().height / 4,
        width: f.area().width / 2,
        height: f.area().height / 2,
    };

    f.render_widget(Clear, area);
    f.render_widget(help_widget, area);
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