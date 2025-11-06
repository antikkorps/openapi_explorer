use crate::app::{App, Panel};
use ratatui::{
    layout::Rect,
    style::{Color, Modifier, Style},
    text::{Line, Span},
    widgets::{Block, Borders, List, ListItem, Paragraph, Wrap},
    Frame,
};

pub fn render_fields_view(f: &mut Frame, app: &mut App, chunks: Vec<Rect>) {
    // Left panel - Fields list
    let field_items: Vec<ListItem> = app.filtered_fields
        .iter()
        .map(|field| {
            let style = if Some(field.as_str()) == app.selected_field.as_deref() {
                Style::default().fg(Color::Yellow).add_modifier(Modifier::BOLD)
            } else {
                Style::default()
            };
            ListItem::new(field.as_str()).style(style)
        })
        .collect();

    let fields_list = List::new(field_items)
        .block(crate::ui::layout::panel_block("Fields", app.current_panel == Panel::Left))
        .highlight_style(Style::default().add_modifier(Modifier::REVERSED));

    f.render_widget(fields_list, chunks[0]);

    // Center panel - Field details
    if let Some(selected_field) = &app.selected_field {
        if let Some(field_info) = app.get_field_info(selected_field) {
            let details_text = vec![
                Line::from(vec![
                    Span::styled("Field: ", Style::default().fg(Color::Cyan)),
                    Span::styled(&field_info.name, Style::default().add_modifier(Modifier::BOLD)),
                ]),
                Line::from(""),
                Line::from(vec![
                    Span::styled("Type: ", Style::default().fg(Color::Cyan)),
                    Span::styled(&field_info.field_type, Style::default()),
                ]),
                Line::from(""),
                Line::from(vec![
                    Span::styled("Description: ", Style::default().fg(Color::Cyan)),
                    Span::styled(
                        field_info.description.as_deref().unwrap_or("No description"),
                        Style::default(),
                    ),
                ]),
                Line::from(""),
                Line::from(vec![
                    Span::styled("Used in schemas: ", Style::default().fg(Color::Cyan)),
                    Span::styled(
                        format!("{} schemas", field_info.schemas.len()),
                        Style::default(),
                    ),
                ]),
                Line::from(""),
                Line::from(vec![
                    Span::styled("Critical: ", Style::default().fg(Color::Cyan)),
                    Span::styled(
                        if field_info.is_critical { "Yes" } else { "No" },
                        Style::default().fg(if field_info.is_critical { Color::Red } else { Color::Green }),
                    ),
                ]),
            ];

            let details_widget = Paragraph::new(details_text)
                .wrap(Wrap { trim: true })
                .block(crate::ui::layout::panel_block("Field Details", app.current_panel == Panel::Center));
            f.render_widget(details_widget, chunks[1]);
        } else {
            let no_details = Paragraph::new("No field selected")
                .style(Style::default().fg(Color::DarkGray))
                .block(crate::ui::layout::panel_block("Field Details", app.current_panel == Panel::Center));
            f.render_widget(no_details, chunks[1]);
        }
    } else {
        let no_selection = Paragraph::new("Select a field to view details")
            .style(Style::default().fg(Color::DarkGray))
            .block(crate::ui::layout::panel_block("Field Details", app.current_panel == Panel::Center));
        f.render_widget(no_selection, chunks[1]);
    }

    // Right panel - Endpoints using this field
    if let Some(selected_field) = &app.selected_field {
        let endpoints = app.field_index.get_endpoints_for_field(selected_field);
        let endpoint_items: Vec<ListItem> = endpoints
            .iter()
            .map(|endpoint| {
                let is_critical = endpoint.to_lowercase().contains("post") || 
                                 endpoint.to_lowercase().contains("put");
                let style = if is_critical {
                    Style::default().fg(Color::Red)
                } else {
                    Style::default()
                };
                ListItem::new(endpoint.as_str()).style(style)
            })
            .collect();

        let title = format!("Endpoints ({})", endpoints.len());
        let endpoints_list = List::new(endpoint_items)
            .block(crate::ui::layout::panel_block(
                &title,
                app.current_panel == Panel::Right,
            ))
            .highlight_style(Style::default().add_modifier(Modifier::REVERSED));

        f.render_widget(endpoints_list, chunks[2]);
    } else {
        let no_endpoints = Paragraph::new("Select a field to see related endpoints")
            .style(Style::default().fg(Color::DarkGray))
            .block(crate::ui::layout::panel_block("Endpoints", app.current_panel == Panel::Right));
        f.render_widget(no_endpoints, chunks[2]);
    }
}