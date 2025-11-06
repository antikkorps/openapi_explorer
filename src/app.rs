use crate::parser::OpenApiSpec;
use crate::indexer::FieldIndex;
use std::collections::HashMap;
use fuzzy_matcher::FuzzyMatcher;
use fuzzy_matcher::skim::SkimMatcherV2;

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
    // Selection indices for navigation
    pub field_list_state: usize,
    pub schema_list_state: usize,
    pub endpoint_list_state: usize,
    // File path for reloading
    pub file_path: Option<std::path::PathBuf>,
    pub should_reload: bool,
    pub reload_error: Option<String>,
}

impl App {
    pub fn new(openapi_spec: OpenApiSpec, field_index: FieldIndex, file_path: Option<std::path::PathBuf>) -> Self {
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
            field_list_state: 0,
            schema_list_state: 0,
            endpoint_list_state: 0,
            file_path,
            should_reload: false,
            reload_error: None,
        };

        app.update_filters();
        app
    }

    pub fn update_filters(&mut self) {
        // Pre-allocate vectors with estimated capacity for better performance
        let estimated_size = if self.search_query.is_empty() {
            self.field_index.fields.len()
        } else {
            self.field_index.fields.len() / 4 // Assume ~25% match rate
        };

        if self.search_query.is_empty() {
            // Fast path: no filtering needed
            self.filtered_fields = self.field_index.fields.keys().cloned().collect();
            self.filtered_schemas = self.field_index.schemas.keys().cloned().collect();
            self.filtered_endpoints = self.openapi_spec.paths.keys().cloned().collect();

            // Sort alphabetically
            self.filtered_fields.sort_unstable();
            self.filtered_schemas.sort_unstable();
            self.filtered_endpoints.sort_unstable();
        } else {
            // Fuzzy search implementation with pre-allocated vectors
            let matcher = SkimMatcherV2::default();
            let query = &self.search_query;

            // Filter and score fields with capacity hint
            let mut field_matches: Vec<(String, i64)> = Vec::with_capacity(estimated_size);
            field_matches.extend(
                self.field_index.fields
                    .keys()
                    .filter_map(|field| {
                        matcher.fuzzy_match(field, query)
                            .map(|score| (field.clone(), score))
                    })
            );
            field_matches.sort_unstable_by(|a, b| b.1.cmp(&a.1)); // Sort by score descending
            self.filtered_fields = field_matches.into_iter().map(|(field, _)| field).collect();

            // Filter and score schemas
            let mut schema_matches: Vec<(String, i64)> = Vec::with_capacity(estimated_size);
            schema_matches.extend(
                self.field_index.schemas
                    .keys()
                    .filter_map(|schema| {
                        matcher.fuzzy_match(schema, query)
                            .map(|score| (schema.clone(), score))
                    })
            );
            schema_matches.sort_unstable_by(|a, b| b.1.cmp(&a.1));
            self.filtered_schemas = schema_matches.into_iter().map(|(schema, _)| schema).collect();

            // Filter and score endpoints
            let mut endpoint_matches: Vec<(String, i64)> = Vec::with_capacity(estimated_size);
            endpoint_matches.extend(
                self.openapi_spec.paths
                    .keys()
                    .filter_map(|endpoint| {
                        matcher.fuzzy_match(endpoint, query)
                            .map(|score| (endpoint.clone(), score))
                    })
            );
            endpoint_matches.sort_unstable_by(|a, b| b.1.cmp(&a.1));
            self.filtered_endpoints = endpoint_matches.into_iter().map(|(endpoint, _)| endpoint).collect();
        }

        // Reset selection indices to stay within bounds
        // Use saturating_sub to avoid underflow on empty lists
        if !self.filtered_fields.is_empty() {
            self.field_list_state = self.field_list_state.min(self.filtered_fields.len() - 1);
        }
        if !self.filtered_schemas.is_empty() {
            self.schema_list_state = self.schema_list_state.min(self.filtered_schemas.len() - 1);
        }
        if !self.filtered_endpoints.is_empty() {
            self.endpoint_list_state = self.endpoint_list_state.min(self.filtered_endpoints.len() - 1);
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

    pub fn navigate_up(&mut self) {
        match self.current_panel {
            Panel::Left => {
                match self.current_view {
                    View::Fields => {
                        if self.field_list_state > 0 {
                            self.field_list_state -= 1;
                        }
                    }
                    View::Schemas => {
                        if self.schema_list_state > 0 {
                            self.schema_list_state -= 1;
                        }
                    }
                    View::Endpoints => {
                        if self.endpoint_list_state > 0 {
                            self.endpoint_list_state -= 1;
                        }
                    }
                    _ => {}
                }
            }
            Panel::Right => {
                // Navigation in right panel (endpoints list)
                if self.endpoint_list_state > 0 {
                    self.endpoint_list_state -= 1;
                }
            }
            _ => {}
        }
    }

    pub fn navigate_down(&mut self) {
        match self.current_panel {
            Panel::Left => {
                match self.current_view {
                    View::Fields => {
                        if self.field_list_state < self.filtered_fields.len().saturating_sub(1) {
                            self.field_list_state += 1;
                        }
                    }
                    View::Schemas => {
                        if self.schema_list_state < self.filtered_schemas.len().saturating_sub(1) {
                            self.schema_list_state += 1;
                        }
                    }
                    View::Endpoints => {
                        if self.endpoint_list_state < self.filtered_endpoints.len().saturating_sub(1) {
                            self.endpoint_list_state += 1;
                        }
                    }
                    _ => {}
                }
            }
            Panel::Right => {
                // Navigation in right panel (endpoints list)
                if let Some(selected_field) = &self.selected_field {
                    let endpoints = self.field_index.get_endpoints_for_field(selected_field);
                    if self.endpoint_list_state < endpoints.len().saturating_sub(1) {
                        self.endpoint_list_state += 1;
                    }
                }
            }
            _ => {}
        }
    }

    pub fn select_current_item(&mut self) {
        match self.current_panel {
            Panel::Left => {
                match self.current_view {
                    View::Fields => {
                        if self.field_list_state < self.filtered_fields.len() {
                            self.selected_field = Some(self.filtered_fields[self.field_list_state].clone());
                            self.endpoint_list_state = 0; // Reset endpoint selection
                        }
                    }
                    View::Schemas => {
                        if self.schema_list_state < self.filtered_schemas.len() {
                            self.selected_schema = Some(self.filtered_schemas[self.schema_list_state].clone());
                        }
                    }
                    View::Endpoints => {
                        if self.endpoint_list_state < self.filtered_endpoints.len() {
                            self.selected_endpoint = Some(self.filtered_endpoints[self.endpoint_list_state].clone());
                        }
                    }
                    _ => {}
                }
            }
            Panel::Right => {
                // Could be used to show endpoint details popup
                self.show_endpoint_details = true;
            }
            _ => {}
        }
    }

    pub fn request_reload(&mut self) {
        self.should_reload = true;
    }

    pub async fn reload(&mut self) -> Result<(), String> {
        if let Some(file_path) = &self.file_path {
            match crate::parser::parse_openapi(file_path).await {
                Ok(spec) => {
                    let new_index = crate::indexer::build_field_index(&spec);
                    self.openapi_spec = spec;
                    self.field_index = new_index;
                    self.update_filters();
                    self.reload_error = None;
                    Ok(())
                }
                Err(e) => {
                    let error_msg = format!("Failed to reload: {}", e);
                    self.reload_error = Some(error_msg.clone());
                    Err(error_msg)
                }
            }
        } else {
            let error_msg = "No file path available for reload".to_string();
            self.reload_error = Some(error_msg.clone());
            Err(error_msg)
        }
    }
}