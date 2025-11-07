use crate::app::{App, Panel};
use ratatui::{
    layout::Rect,
    style::{Color, Modifier, Style},
    text::{Line, Span},
    widgets::{Block, Borders, List, ListItem, Paragraph, Wrap},
    Frame,
};

pub fn render_schemas_view(f: &mut Frame, app: &mut App, chunks: Vec<Rect>) {
    // Left panel - Schemas list
    let schema_items: Vec<ListItem> = app
        .filtered_schemas
        .iter()
        .map(|schema| {
            let style = if Some(schema.as_str()) == app.selected_schema.as_deref() {
                Style::default()
                    .fg(Color::Yellow)
                    .add_modifier(Modifier::BOLD)
            } else {
                Style::default()
            };
            ListItem::new(schema.as_str()).style(style)
        })
        .collect();

    let schemas_list = List::new(schema_items)
        .block(crate::ui::layout::panel_block(
            "Schemas",
            app.current_panel == Panel::Left,
        ))
        .highlight_style(Style::default().add_modifier(Modifier::REVERSED));

    f.render_widget(schemas_list, chunks[0]);

    // Center panel - Schema details
    if let Some(selected_schema) = &app.selected_schema {
        if let Some(schema) = app.field_index.schemas.get(selected_schema) {
            let fields = schema.get_field_names();
            let mut details_text = vec![
                Line::from(vec![
                    Span::styled("Schema: ", Style::default().fg(Color::Cyan)),
                    Span::styled(
                        selected_schema,
                        Style::default().add_modifier(Modifier::BOLD),
                    ),
                ]),
                Line::from(""),
                Line::from(vec![
                    Span::styled("Type: ", Style::default().fg(Color::Cyan)),
                    Span::styled(
                        schema.schema_type.as_deref().unwrap_or("object"),
                        Style::default(),
                    ),
                ]),
                Line::from(""),
                Line::from(vec![
                    Span::styled("Fields: ", Style::default().fg(Color::Cyan)),
                    Span::styled(format!("{} fields", fields.len()), Style::default()),
                ]),
                Line::from(""),
            ];

            if let Some(description) = &schema.description {
                details_text.push(Line::from(vec![
                    Span::styled("Description: ", Style::default().fg(Color::Cyan)),
                    Span::styled(description, Style::default()),
                ]));
                details_text.push(Line::from(""));
            }

            details_text.push(Line::from("Field List:"));
            for (i, field) in fields.iter().enumerate() {
                let field_type = schema
                    .get_field_type(field)
                    .unwrap_or_else(|| "unknown".to_string());
                details_text.push(Line::from(vec![
                    Span::styled(
                        format!("  {}. ", i + 1),
                        Style::default().fg(Color::DarkGray),
                    ),
                    Span::styled(field, Style::default().add_modifier(Modifier::BOLD)),
                    Span::styled(
                        format!(" ({})", field_type),
                        Style::default().fg(Color::Green),
                    ),
                ]));
            }

            let details_widget = Paragraph::new(details_text)
                .wrap(Wrap { trim: true })
                .block(crate::ui::layout::panel_block(
                    "Schema Details",
                    app.current_panel == Panel::Center,
                ));
            f.render_widget(details_widget, chunks[1]);
        } else {
            let no_details = Paragraph::new("Schema not found")
                .style(Style::default().fg(Color::Red))
                .block(crate::ui::layout::panel_block(
                    "Schema Details",
                    app.current_panel == Panel::Center,
                ));
            f.render_widget(no_details, chunks[1]);
        }
    } else {
        let no_selection = Paragraph::new("Select a schema to view details")
            .style(Style::default().fg(Color::DarkGray))
            .block(crate::ui::layout::panel_block(
                "Schema Details",
                app.current_panel == Panel::Center,
            ));
        f.render_widget(no_selection, chunks[1]);
    }

    // Right panel - Related endpoints
    if let Some(selected_schema) = &app.selected_schema {
        let schema_fields = app.field_index.get_schema_fields(selected_schema);
        let mut related_endpoints = std::collections::HashSet::new();

        for field in &schema_fields {
            let endpoints = app.field_index.get_endpoints_for_field(field);
            for endpoint in endpoints {
                related_endpoints.insert(endpoint);
            }
        }

        let endpoint_items: Vec<ListItem> = related_endpoints
            .iter()
            .map(|endpoint| {
                let is_critical = endpoint.to_lowercase().contains("post")
                    || endpoint.to_lowercase().contains("put");
                let style = if is_critical {
                    Style::default().fg(Color::Red)
                } else {
                    Style::default()
                };
                ListItem::new(endpoint.as_str()).style(style)
            })
            .collect();

        let title = format!("Related Endpoints ({})", related_endpoints.len());
        let endpoints_list = List::new(endpoint_items)
            .block(crate::ui::layout::panel_block(
                &title,
                app.current_panel == Panel::Right,
            ))
            .highlight_style(Style::default().add_modifier(Modifier::REVERSED));

        f.render_widget(endpoints_list, chunks[2]);
    } else {
        let no_endpoints = Paragraph::new("Select a schema to see related endpoints")
            .style(Style::default().fg(Color::DarkGray))
            .block(crate::ui::layout::panel_block(
                "Related Endpoints",
                app.current_panel == Panel::Right,
            ));
        f.render_widget(no_endpoints, chunks[2]);
    }
}
