use crate::app::{App, Panel};
use ratatui::{
    layout::Rect,
    style::{Color, Modifier, Style},
    text::{Line, Span},
    widgets::{Block, Borders, List, ListItem, Paragraph, Wrap},
    Frame,
};

pub fn render_endpoints_view(f: &mut Frame, app: &mut App, chunks: Vec<Rect>) {
    // Left panel - Endpoints list
    let endpoint_items: Vec<ListItem> = app.filtered_endpoints
        .iter()
        .map(|endpoint| {
            let style = if Some(endpoint.as_str()) == app.selected_endpoint.as_deref() {
                Style::default().fg(Color::Yellow).add_modifier(Modifier::BOLD)
            } else {
                let is_critical = endpoint.to_lowercase().contains("post") || 
                                 endpoint.to_lowercase().contains("put");
                if is_critical {
                    Style::default().fg(Color::Red)
                } else {
                    Style::default()
                }
            };
            ListItem::new(endpoint.as_str()).style(style)
        })
        .collect();

    let endpoints_list = List::new(endpoint_items)
        .block(crate::ui::layout::panel_block("Endpoints", app.current_panel == Panel::Left))
        .highlight_style(Style::default().add_modifier(Modifier::REVERSED));

    f.render_widget(endpoints_list, chunks[0]);

    // Center panel - Endpoint details
    if let Some(selected_endpoint) = &app.selected_endpoint {
        let parts: Vec<&str> = selected_endpoint.splitn(2, ' ').collect();
        if parts.len() == 2 {
            let method = parts[0];
            let path = parts[1];
            
            if let Some(path_item) = app.openapi_spec.paths.get(path) {
                if let Some(operation) = path_item.operations.get(method.to_lowercase().as_str()) {
                    let mut details_text = vec![
                        Line::from(vec![
                            Span::styled("Endpoint: ", Style::default().fg(Color::Cyan)),
                            Span::styled(selected_endpoint, Style::default().add_modifier(Modifier::BOLD)),
                        ]),
                        Line::from(""),
                        Line::from(vec![
                            Span::styled("Method: ", Style::default().fg(Color::Cyan)),
                            Span::styled(method.to_uppercase(), Style::default().fg(Color::Yellow)),
                        ]),
                        Line::from(""),
                        Line::from(vec![
                            Span::styled("Path: ", Style::default().fg(Color::Cyan)),
                            Span::styled(path, Style::default()),
                        ]),
                        Line::from(""),
                    ];

                    if let Some(summary) = &operation.summary {
                        details_text.push(Line::from(vec![
                            Span::styled("Summary: ", Style::default().fg(Color::Cyan)),
                            Span::styled(summary, Style::default()),
                        ]));
                        details_text.push(Line::from(""));
                    }

                    if let Some(description) = &operation.description {
                        details_text.push(Line::from(vec![
                            Span::styled("Description: ", Style::default().fg(Color::Cyan)),
                            Span::styled(description, Style::default()),
                        ]));
                        details_text.push(Line::from(""));
                    }

                    if let Some(tags) = &operation.tags {
                        if !tags.is_empty() {
                            details_text.push(Line::from(vec![
                                Span::styled("Tags: ", Style::default().fg(Color::Cyan)),
                                Span::styled(tags.join(", "), Style::default().fg(Color::Green)),
                            ]));
                            details_text.push(Line::from(""));
                        }
                    }

                    if let Some(parameters) = &operation.parameters {
                        details_text.push(Line::from(vec![
                            Span::styled("Parameters: ", Style::default().fg(Color::Cyan)),
                            Span::styled(format!("{} parameters", parameters.len()), Style::default()),
                        ]));
                        for param in parameters {
                            let required = param.required.unwrap_or(false);
                            details_text.push(Line::from(vec![
                                Span::styled("  • ", Style::default().fg(Color::DarkGray)),
                                Span::styled(&param.name, Style::default().add_modifier(Modifier::BOLD)),
                                Span::styled(format!(" ({})", param.in_), Style::default().fg(Color::Green)),
                                Span::styled(
                                    if required { " (required)" } else { " (optional)" },
                                    Style::default().fg(if required { Color::Red } else { Color::DarkGray }),
                                ),
                            ]));
                        }
                        details_text.push(Line::from(""));
                    }

                    let details_widget = Paragraph::new(details_text)
                        .wrap(Wrap { trim: true })
                        .block(crate::ui::layout::panel_block("Endpoint Details", app.current_panel == Panel::Center));
                    f.render_widget(details_widget, chunks[1]);
                } else {
                    let no_operation = Paragraph::new("Operation not found")
                        .style(Style::default().fg(Color::Red))
                        .block(crate::ui::layout::panel_block("Endpoint Details", app.current_panel == Panel::Center));
                    f.render_widget(no_operation, chunks[1]);
                }
            } else {
                let no_path = Paragraph::new("Path not found")
                    .style(Style::default().fg(Color::Red))
                    .block(crate::ui::layout::panel_block("Endpoint Details", app.current_panel == Panel::Center));
                f.render_widget(no_path, chunks[1]);
            }
        } else {
            let invalid_format = Paragraph::new("Invalid endpoint format")
                .style(Style::default().fg(Color::Red))
                .block(crate::ui::layout::panel_block("Endpoint Details", app.current_panel == Panel::Center));
            f.render_widget(invalid_format, chunks[1]);
        }
    } else {
        let no_selection = Paragraph::new("Select an endpoint to view details")
            .style(Style::default().fg(Color::DarkGray))
            .block(crate::ui::layout::panel_block("Endpoint Details", app.current_panel == Panel::Center));
        f.render_widget(no_selection, chunks[1]);
    }

    // Right panel - Fields used by this endpoint
    if let Some(selected_endpoint) = &app.selected_endpoint {
        if let Some(fields) = app.field_index.endpoint_fields.get(selected_endpoint) {
            let field_items: Vec<ListItem> = fields
                .iter()
                .map(|field| {
                    let is_critical = app.field_index.is_critical_field(field);
                    let style = if is_critical {
                        Style::default().fg(Color::Red)
                    } else {
                        Style::default()
                    };
                    ListItem::new(field.as_str()).style(style)
                })
                .collect();

            let title = format!("Fields ({})", fields.len());
            let fields_list = List::new(field_items)
                .block(crate::ui::layout::panel_block(
                    &title,
                    app.current_panel == Panel::Right,
                ))
                .highlight_style(Style::default().add_modifier(Modifier::REVERSED));

            f.render_widget(fields_list, chunks[2]);
        } else {
            let no_fields = Paragraph::new("No fields found for this endpoint")
                .style(Style::default().fg(Color::DarkGray))
                .block(crate::ui::layout::panel_block("Fields", app.current_panel == Panel::Right));
            f.render_widget(no_fields, chunks[2]);
        }
    } else {
        let no_endpoint = Paragraph::new("Select an endpoint to see related fields")
            .style(Style::default().fg(Color::DarkGray))
            .block(crate::ui::layout::panel_block("Fields", app.current_panel == Panel::Right));
        f.render_widget(no_endpoint, chunks[2]);
    }
}