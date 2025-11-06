use crate::app::{App, Panel};
use ratatui::{
    layout::Rect,
    style::{Color, Modifier, Style},
    text::{Line, Span},
    widgets::{Block, Borders, Paragraph, Wrap},
    Frame,
};

pub fn render_graph_view(f: &mut Frame, app: &mut App, chunks: Vec<Rect>) {
    // Left panel - Graph options
    let options_text = vec![
        Line::from("Graph Options"),
        Line::from(""),
        Line::from("• Field relationships"),
        Line::from("• Schema dependencies"),
        Line::from("• Endpoint connections"),
        Line::from("• Critical path analysis"),
        Line::from(""),
        Line::from("Press 'g' to generate"),
        Line::from("Press 's' to save"),
    ];

    let options_widget = Paragraph::new(options_text)
        .wrap(Wrap { trim: true })
        .block(crate::ui::layout::panel_block("Options", app.current_panel == Panel::Left));
    f.render_widget(options_widget, chunks[0]);

    // Center panel - ASCII graph visualization
    let graph_text = generate_ascii_graph(app);
    
    let graph_widget = Paragraph::new(graph_text)
        .wrap(Wrap { trim: true })
        .block(crate::ui::layout::panel_block("Field Relationship Graph", app.current_panel == Panel::Center));
    f.render_widget(graph_widget, chunks[1]);

    // Right panel - Graph statistics
    let stats_text = vec![
        Line::from("Graph Statistics"),
        Line::from(""),
        Line::from(format!("Total Nodes: {}", app.field_index.fields.len())),
        Line::from(format!("Total Schemas: {}", app.field_index.schemas.len())),
        Line::from(format!("Total Endpoints: {}", app.openapi_spec.paths.len())),
        Line::from(""),
        Line::from("Critical Fields:"),
        Line::from(format!("  • {} high-impact", count_critical_fields(app))),
        Line::from(""),
        Line::from("Most Connected:"),
        Line::from(format!("  • {}", get_most_connected_field(app))),
        Line::from(""),
        Line::from("Graph Density:"),
        Line::from(format!("  • {:.2}%", calculate_graph_density(app))),
    ];

    let stats_widget = Paragraph::new(stats_text)
        .wrap(Wrap { trim: true })
        .block(crate::ui::layout::panel_block("Statistics", app.current_panel == Panel::Right));
    f.render_widget(stats_widget, chunks[2]);
}

fn generate_ascii_graph(app: &App) -> Vec<Line> {
    let mut lines = vec![
        Line::from("Field Relationship Graph"),
        Line::from(""),
    ];

    if app.field_index.fields.is_empty() {
        lines.push(Line::from("No fields to display"));
        return lines;
    }

    // Simple ASCII graph showing field relationships
    let mut field_count = 0;
    let max_fields = std::cmp::min(10, app.field_index.fields.len()); // Limit display
    
    lines.push(Line::from("┌─ Field Dependencies ──────────────────┐"));
    
    for (field_name, field_data) in &app.field_index.fields {
        if field_count >= max_fields {
            break;
        }
        
        let is_critical = app.field_index.is_critical_field(field_name);
        let marker = if is_critical { "🔴" } else { "⚪" };
        
        lines.push(Line::from(format!("│ {} {} ({} schemas)", 
            marker, 
            field_name, 
            field_data.schemas.len()
        )));
        
        // Show connections to other fields (simplified)
        if field_data.schemas.len() > 1 {
            lines.push(Line::from("│    ├─ Shared schemas"));
            for schema in &field_data.schemas {
                if field_data.schemas.len() > 2 {
                    lines.push(Line::from(format!("│    │  • {}", schema)));
                }
            }
        }
        
        field_count += 1;
    }
    
    lines.push(Line::from("└──────────────────────────────────────┘"));
    lines.push(Line::from(""));
    
    if app.field_index.fields.len() > max_fields {
        lines.push(Line::from(format!("... and {} more fields", 
            app.field_index.fields.len() - max_fields
        )));
    }
    
    lines.push(Line::from(""));
    lines.push(Line::from("Legend: 🔴 Critical field  ⚪ Regular field"));
    
    lines
}

fn count_critical_fields(app: &App) -> usize {
    app.field_index.fields
        .keys()
        .filter(|field| app.field_index.is_critical_field(field))
        .count()
}

fn get_most_connected_field(app: &App) -> String {
    app.field_index.fields
        .iter()
        .max_by_key(|(_, field_data)| field_data.schemas.len())
        .map(|(name, _)| name.clone())
        .unwrap_or_else(|| "None".to_string())
}

fn calculate_graph_density(app: &App) -> f64 {
    let total_fields = app.field_index.fields.len();
    if total_fields == 0 {
        return 0.0;
    }
    
    let total_connections: usize = app.field_index.fields
        .values()
        .map(|field_data| field_data.schemas.len())
        .sum();
    
    let max_possible_connections = total_fields * (total_fields - 1) / 2;
    
    if max_possible_connections == 0 {
        0.0
    } else {
        (total_connections as f64 / max_possible_connections as f64) * 100.0
    }
}