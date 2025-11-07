use crate::parser::OpenApiSpec;
use crate::indexer::FieldIndex;
use std::collections::HashMap;
use fuzzy_matcher::FuzzyMatcher;
use fuzzy_matcher::skim::SkimMatcherV2;

// Heuristic for pre-allocating vectors during fuzzy search
// Assumes approximately 25% of items will match a typical search query
const FUZZY_SEARCH_MATCH_RATE: usize = 4; // 1/4 = 25%

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
    pub selected_endpoint_for_details: Option<String>,
    // Selection indices for navigation
    pub field_list_state: usize,
    pub schema_list_state: usize,
    pub endpoint_list_state: usize,
    // File path for reloading
    pub file_path: Option<std::path::PathBuf>,
    pub should_reload: bool,
    pub reload_error: Option<String>,
    // Loading state
    pub is_loading: bool,
    pub loading_message: String,
    // Validation warnings
    pub validation_warnings: Vec<String>,
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
            selected_endpoint_for_details: None,
            field_list_state: 0,
            schema_list_state: 0,
            endpoint_list_state: 0,
            file_path,
            should_reload: false,
            reload_error: None,
            is_loading: false,
            loading_message: String::new(),
            validation_warnings: Vec::new(),
        };

        app.update_filters();
        app.validate_spec();
        app
    }

    pub fn update_filters(&mut self) {
        // Pre-allocate vectors with estimated capacity for better performance
        let estimated_size = if self.search_query.is_empty() {
            self.field_index.fields.len()
        } else {
            // Use heuristic: assume ~25% of items match a typical search query
            self.field_index.fields.len() / FUZZY_SEARCH_MATCH_RATE
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
        // Reset to 0 when lists are empty to prevent index out of bounds
        if !self.filtered_fields.is_empty() {
            self.field_list_state = self.field_list_state.min(self.filtered_fields.len() - 1);
        } else {
            self.field_list_state = 0;
        }

        if !self.filtered_schemas.is_empty() {
            self.schema_list_state = self.schema_list_state.min(self.filtered_schemas.len() - 1);
        } else {
            self.schema_list_state = 0;
        }

        if !self.filtered_endpoints.is_empty() {
            self.endpoint_list_state = self.endpoint_list_state.min(self.filtered_endpoints.len() - 1);
        } else {
            self.endpoint_list_state = 0;
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
                // Only navigate if a field is selected (consistent with navigate_down)
                if let Some(selected_field) = &self.selected_field {
                    let endpoints = self.field_index.get_endpoints_for_field(selected_field);
                    if self.endpoint_list_state > 0 && !endpoints.is_empty() {
                        self.endpoint_list_state -= 1;
                    }
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
                    if !endpoints.is_empty() && self.endpoint_list_state < endpoints.len() - 1 {
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
                        // Use get() for safe bounds-checked access
                        if let Some(field) = self.filtered_fields.get(self.field_list_state) {
                            self.selected_field = Some(field.clone());
                            self.endpoint_list_state = 0; // Reset endpoint selection
                        }
                    }
                    View::Schemas => {
                        if let Some(schema) = self.filtered_schemas.get(self.schema_list_state) {
                            self.selected_schema = Some(schema.clone());
                        }
                    }
                    View::Endpoints => {
                        if let Some(endpoint) = self.filtered_endpoints.get(self.endpoint_list_state) {
                            self.selected_endpoint = Some(endpoint.clone());
                        }
                    }
                    _ => {}
                }
            }
            Panel::Right => {
                // Show endpoint details popup when selecting in Right panel
                if let Some(selected_field) = &self.selected_field {
                    let endpoints = self.field_index.get_endpoints_for_field(selected_field);
                    if let Some(endpoint) = endpoints.get(self.endpoint_list_state) {
                        self.selected_endpoint_for_details = Some(endpoint.clone());
                        self.show_endpoint_details = true;
                    }
                }
            }
            _ => {}
        }
    }

    pub fn request_reload(&mut self) {
        self.should_reload = true;
        self.is_loading = true;
        self.loading_message = "Reloading OpenAPI specification...".to_string();
    }

    pub async fn reload(&mut self) -> Result<(), String> {
        if let Some(file_path) = &self.file_path {
            self.is_loading = true;
            self.loading_message = format!("Parsing {}...",
                file_path.file_name().unwrap_or_default().to_string_lossy());

            match crate::parser::parse_openapi(file_path).await {
                Ok(spec) => {
                    self.loading_message = "Building field index...".to_string();
                    let new_index = crate::indexer::build_field_index(&spec);
                    self.openapi_spec = spec;
                    self.field_index = new_index;
                    self.update_filters();
                    self.validate_spec(); // Validate after reload
                    self.reload_error = None;
                    self.is_loading = false;
                    self.loading_message.clear();
                    Ok(())
                }
                Err(e) => {
                    let error_msg = format!("Failed to reload: {}", e);
                    self.reload_error = Some(error_msg.clone());
                    self.is_loading = false;
                    self.loading_message.clear();
                    Err(error_msg)
                }
            }
        } else {
            let error_msg = "No file path available for reload".to_string();
            self.reload_error = Some(error_msg.clone());
            self.is_loading = false;
            self.loading_message.clear();
            Err(error_msg)
        }
    }

    pub fn validate_spec(&mut self) {
        self.validation_warnings.clear();

        // Check for empty or missing components
        if self.openapi_spec.components.is_none() {
            self.validation_warnings.push("No components section found in OpenAPI spec".to_string());
        } else if let Some(components) = &self.openapi_spec.components {
            if components.schemas.is_none() || components.schemas.as_ref().unwrap().is_empty() {
                self.validation_warnings.push("No schemas defined in components".to_string());
            }
        }

        // Check for paths
        if self.openapi_spec.paths.is_empty() {
            self.validation_warnings.push("No paths/endpoints defined in spec".to_string());
        }

        // Check for fields without types
        for (field_name, field_data) in &self.field_index.fields {
            if field_data.field_type == "unknown" {
                self.validation_warnings.push(
                    format!("Field '{}' has unknown type", field_name)
                );
            }
        }

        // Check for endpoints without operations
        for (path, path_item) in &self.openapi_spec.paths {
            if path_item.operations.is_empty() {
                self.validation_warnings.push(
                    format!("Path '{}' has no operations defined", path)
                );
            }
        }

        // Check for missing descriptions
        let mut missing_descriptions = 0;
        for (_, operation) in self.openapi_spec.paths.values()
            .flat_map(|pi| pi.operations.iter()) {
            if operation.description.is_none() && operation.summary.is_none() {
                missing_descriptions += 1;
            }
        }
        if missing_descriptions > 0 {
            self.validation_warnings.push(
                format!("{} endpoint(s) missing description/summary", missing_descriptions)
            );
        }

        // Check for schemas not used in any endpoint
        let mut unused_schemas = 0;
        for schema_name in self.field_index.schemas.keys() {
            let is_used = self.field_index.fields.values()
                .any(|field_data| field_data.schemas.contains(schema_name) && !field_data.endpoints.is_empty());
            if !is_used {
                unused_schemas += 1;
            }
        }
        if unused_schemas > 0 {
            self.validation_warnings.push(
                format!("{} schema(s) not used in any endpoint", unused_schemas)
            );
        }

        log::debug!("Spec validation complete: {} warning(s) found", self.validation_warnings.len());
    }
}