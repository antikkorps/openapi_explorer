use crate::parser::OpenApiSpec;
use crate::indexer::FieldIndex;
use std::collections::HashMap;

#[derive(Debug, Clone, PartialEq)]
pub enum View {
    Fields,
    Schemas,
    Endpoints,
    Graph,
    Stats,
}

#[derive(Debug, Clone, PartialEq)]
pub enum Panel {
    Left,
    Center,
    Right,
}

#[derive(Debug, Clone)]
pub struct FieldInfo {
    pub name: String,
    pub field_type: String,
    pub description: Option<String>,
    pub schemas: Vec<String>,
    pub endpoints: Vec<String>,
    pub is_critical: bool,
}

#[derive(Debug)]
pub struct App {
    pub openapi_spec: OpenApiSpec,
    pub field_index: FieldIndex,
    pub current_view: View,
    pub current_panel: Panel,
    pub selected_field: Option<String>,
    pub selected_schema: Option<String>,
    pub selected_endpoint: Option<String>,
    pub search_query: String,
    pub filtered_fields: Vec<String>,
    pub filtered_schemas: Vec<String>,
    pub filtered_endpoints: Vec<String>,
    pub should_quit: bool,
    pub show_help: bool,
    pub show_endpoint_details: bool,
}

impl App {
    pub fn new(openapi_spec: OpenApiSpec, field_index: FieldIndex) -> Self {
        let mut app = Self {
            openapi_spec,
            field_index,
            current_view: View::Fields,
            current_panel: Panel::Left,
            selected_field: None,
            selected_schema: None,
            selected_endpoint: None,
            search_query: String::new(),
            filtered_fields: Vec::new(),
            filtered_schemas: Vec::new(),
            filtered_endpoints: Vec::new(),
            should_quit: false,
            show_help: false,
            show_endpoint_details: false,
        };

        app.update_filters();
        app
    }

    pub fn update_filters(&mut self) {
        if self.search_query.is_empty() {
            self.filtered_fields = self.field_index.fields.keys().cloned().collect();
            self.filtered_schemas = self.field_index.schemas.keys().cloned().collect();
            self.filtered_endpoints = self.openapi_spec.paths.keys().cloned().collect();
        } else {
            // Simple filtering for now - will be enhanced with fuzzy matching
            let query = self.search_query.to_lowercase();
            
            self.filtered_fields = self.field_index.fields
                .keys()
                .filter(|field| field.to_lowercase().contains(&query))
                .cloned()
                .collect();
            
            self.filtered_schemas = self.field_index.schemas
                .keys()
                .filter(|schema| schema.to_lowercase().contains(&query))
                .cloned()
                .collect();
            
            self.filtered_endpoints = self.openapi_spec.paths
                .keys()
                .filter(|endpoint| endpoint.to_lowercase().contains(&query))
                .cloned()
                .collect();
        }
    }

    pub fn get_field_info(&self, field_name: &str) -> Option<FieldInfo> {
        self.field_index.fields.get(field_name).map(|field_data| {
            let endpoints = self.field_index.get_endpoints_for_field(field_name);
            FieldInfo {
                name: field_name.to_string(),
                field_type: field_data.field_type.clone(),
                description: field_data.description.clone(),
                schemas: field_data.schemas.clone(),
                endpoints,
                is_critical: self.field_index.is_critical_field(field_name),
            }
        })
    }

    pub fn next_panel(&mut self) {
        self.current_panel = match self.current_panel {
            Panel::Left => Panel::Center,
            Panel::Center => Panel::Right,
            Panel::Right => Panel::Left,
        };
    }

    pub fn set_view(&mut self, view: View) {
        self.current_view = view;
        self.selected_field = None;
        self.selected_schema = None;
        self.selected_endpoint = None;
    }
}