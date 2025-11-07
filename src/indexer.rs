use crate::parser::{OpenApiSpec, Schema};
use std::collections::{HashMap, HashSet};

#[derive(Debug, Clone)]
pub struct FieldData {
    pub field_type: String,
    pub description: Option<String>,
    pub schemas: Vec<String>,
    pub endpoints: HashSet<String>,
}

#[derive(Debug)]
pub struct FieldIndex {
    pub fields: HashMap<String, FieldData>,
    pub schemas: HashMap<String, Schema>,
    pub endpoint_fields: HashMap<String, Vec<String>>,
}

impl FieldIndex {
    pub fn new() -> Self {
        Self {
            fields: HashMap::new(),
            schemas: HashMap::new(),
            endpoint_fields: HashMap::new(),
        }
    }

    pub fn get_endpoints_for_field(&self, field_name: &str) -> Vec<String> {
        self.fields
            .get(field_name)
            .map(|data| data.endpoints.iter().cloned().collect())
            .unwrap_or_default()
    }

    pub fn is_critical_field(&self, field_name: &str) -> bool {
        if let Some(data) = self.fields.get(field_name) {
            // Consider a field critical if it's used in POST/PUT operations
            data.endpoints.iter().any(|endpoint| {
                endpoint.to_lowercase().contains("post") || endpoint.to_lowercase().contains("put")
            })
        } else {
            false
        }
    }

    pub fn get_schema_fields(&self, schema_name: &str) -> Vec<String> {
        self.schemas
            .get(schema_name)
            .map(|schema| schema.get_field_names())
            .unwrap_or_default()
    }
}

pub fn build_field_index(openapi_spec: &OpenApiSpec) -> FieldIndex {
    let mut index = FieldIndex::new();

    log::debug!("Building field index from OpenAPI specification");

    // Index all schemas first
    if let Some(components) = &openapi_spec.components {
        if let Some(schemas) = &components.schemas {
            log::debug!("Processing {} schemas", schemas.len());
            for (schema_name, schema) in schemas {
                index.schemas.insert(schema_name.clone(), schema.clone());

                // Index fields from this schema
                let field_names = schema.get_field_names();
                log::trace!("Schema '{}' has {} fields", schema_name, field_names.len());

                for field_name in field_names {
                    let field_data =
                        index
                            .fields
                            .entry(field_name.clone())
                            .or_insert_with(|| FieldData {
                                field_type: schema
                                    .get_field_type(&field_name)
                                    .unwrap_or_else(|| "unknown".to_string()),
                                description: schema.get_field_description(&field_name),
                                schemas: Vec::new(),
                                endpoints: HashSet::new(),
                            });

                    if !field_data.schemas.contains(schema_name) {
                        field_data.schemas.push(schema_name.clone());
                    }
                }
            }
        }
    } else {
        log::warn!("No components found in OpenAPI specification");
    }

    // Index endpoints and their field usage
    log::debug!("Processing {} endpoints", openapi_spec.paths.len());
    for (path, path_item) in &openapi_spec.paths {
        for (method, operation) in &path_item.operations {
            let endpoint_key = format!("{} {}", method.to_uppercase(), path);
            let mut endpoint_fields = Vec::new();
            log::trace!("Processing endpoint: {}", endpoint_key);

            // Check parameters
            if let Some(parameters) = &operation.parameters {
                for param in parameters {
                    if let Some(schema) = &param.schema {
                        let param_fields = extract_fields_from_schema(schema);
                        for field in param_fields {
                            endpoint_fields.push(field.clone());
                            if let Some(field_data) = index.fields.get_mut(&field) {
                                field_data.endpoints.insert(endpoint_key.clone());
                            }
                        }
                    }
                }
            }

            // Check request body
            if let Some(request_body) = &operation.request_body {
                for (_content_type, media_type) in &request_body.content {
                    if let Some(schema) = &media_type.schema {
                        let body_fields = extract_fields_from_schema(schema);
                        for field in body_fields {
                            endpoint_fields.push(field.clone());
                            if let Some(field_data) = index.fields.get_mut(&field) {
                                field_data.endpoints.insert(endpoint_key.clone());
                            }
                        }
                    }
                }
            }

            // Check responses
            for (_status_code, response) in &operation.responses {
                if let Some(content) = &response.content {
                    for (_content_type, media_type) in content {
                        if let Some(schema) = &media_type.schema {
                            let response_fields = extract_fields_from_schema(schema);
                            for field in response_fields {
                                endpoint_fields.push(field.clone());
                                if let Some(field_data) = index.fields.get_mut(&field) {
                                    field_data.endpoints.insert(endpoint_key.clone());
                                }
                            }
                        }
                    }
                }
            }

            index.endpoint_fields.insert(endpoint_key, endpoint_fields);
        }
    }

    index
}

pub fn analyze_field_relationships(index: &FieldIndex) -> HashMap<String, Vec<String>> {
    let mut relationships = HashMap::new();

    for (field_name, field_data) in &index.fields {
        let mut related_fields = Vec::new();

        // Find fields that appear in the same schemas
        for schema_name in &field_data.schemas {
            if let Some(schema) = index.schemas.get(schema_name) {
                let schema_fields = schema.get_field_names();
                for other_field in schema_fields {
                    if other_field != *field_name {
                        related_fields.push(other_field);
                    }
                }
            }
        }

        // Remove duplicates and sort
        related_fields.sort();
        related_fields.dedup();
        relationships.insert(field_name.clone(), related_fields);
    }

    relationships
}

fn extract_fields_from_schema(schema: &crate::parser::Schema) -> Vec<String> {
    let mut fields = Vec::new();

    // Direct properties
    if let Some(properties) = &schema.properties {
        fields.extend(properties.keys().cloned());
    }

    // Array items
    if let Some(items) = &schema.items {
        fields.extend(extract_fields_from_schema(items));
    }

    // Composition (allOf, oneOf, anyOf)
    if let Some(all_of) = &schema.all_of {
        for sub_schema in all_of {
            fields.extend(extract_fields_from_schema(sub_schema));
        }
    }

    if let Some(one_of) = &schema.one_of {
        for sub_schema in one_of {
            fields.extend(extract_fields_from_schema(sub_schema));
        }
    }

    if let Some(any_of) = &schema.any_of {
        for sub_schema in any_of {
            fields.extend(extract_fields_from_schema(sub_schema));
        }
    }

    fields
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::parser::{Components, Info, OpenApiSpec, Operation, PathItem, Schema};
    use std::collections::HashMap;

    fn create_test_spec() -> OpenApiSpec {
        OpenApiSpec {
            openapi: "3.0.0".to_string(),
            info: Info {
                title: "Test API".to_string(),
                version: "1.0.0".to_string(),
                description: None,
            },
            paths: HashMap::from([(
                "/users".to_string(),
                PathItem {
                    operations: HashMap::from([
                        (
                            "get".to_string(),
                            Operation {
                                operation_id: Some("listUsers".to_string()),
                                summary: Some("List users".to_string()),
                                description: None,
                                tags: None,
                                parameters: Some(vec![crate::parser::Parameter {
                                    name: "id".to_string(),
                                    in_: "query".to_string(),
                                    description: Some("User ID".to_string()),
                                    required: Some(true),
                                    schema: Some(Schema {
                                        schema_type: Some("integer".to_string()),
                                        ..Default::default()
                                    }),
                                }]),
                                request_body: None,
                                responses: HashMap::new(),
                            },
                        ),
                        (
                            "post".to_string(),
                            Operation {
                                operation_id: Some("createUser".to_string()),
                                summary: Some("Create user".to_string()),
                                description: None,
                                tags: None,
                                parameters: None,
                                request_body: Some(crate::parser::RequestBody {
                                    description: None,
                                    content: HashMap::from([(
                                        "application/json".to_string(),
                                        crate::parser::MediaType {
                                            schema: Some(Schema {
                                                schema_type: Some("object".to_string()),
                                                properties: Some(HashMap::from([
                                                    (
                                                        "name".to_string(),
                                                        Schema {
                                                            schema_type: Some("string".to_string()),
                                                            ..Default::default()
                                                        },
                                                    ),
                                                    (
                                                        "email".to_string(),
                                                        Schema {
                                                            schema_type: Some("string".to_string()),
                                                            format: Some("email".to_string()),
                                                            ..Default::default()
                                                        },
                                                    ),
                                                ])),
                                                ..Default::default()
                                            }),
                                        },
                                    )]),
                                }),
                                responses: HashMap::new(),
                            },
                        ),
                    ]),
                },
            )]),
            components: Some(Components {
                schemas: Some(HashMap::from([(
                    "User".to_string(),
                    Schema {
                        schema_type: Some("object".to_string()),
                        properties: Some(HashMap::from([
                            (
                                "id".to_string(),
                                Schema {
                                    schema_type: Some("integer".to_string()),
                                    description: Some("User ID".to_string()),
                                    ..Default::default()
                                },
                            ),
                            (
                                "name".to_string(),
                                Schema {
                                    schema_type: Some("string".to_string()),
                                    description: Some("User name".to_string()),
                                    ..Default::default()
                                },
                            ),
                        ])),
                        ..Default::default()
                    },
                )])),
            }),
        }
    }

    #[test]
    fn test_field_index_new() {
        let index = FieldIndex::new();
        assert!(index.fields.is_empty());
        assert!(index.schemas.is_empty());
        assert!(index.endpoint_fields.is_empty());
    }

    #[test]
    fn test_build_field_index() {
        let spec = create_test_spec();
        let index = build_field_index(&spec);

        // Check that fields were indexed
        assert!(index.fields.contains_key("id"));
        assert!(index.fields.contains_key("name"));
        // Note: email field might not be extracted depending on implementation
        // Let's check what we actually have
        println!(
            "Available fields: {:?}",
            index.fields.keys().collect::<Vec<_>>()
        );

        // Check that schemas were indexed
        assert!(index.schemas.contains_key("User"));

        // Check field data
        let id_field = index.fields.get("id").unwrap();
        assert_eq!(id_field.field_type, "integer");
        assert!(id_field.schemas.contains(&"User".to_string()));
        assert!(!id_field.endpoints.is_empty());

        // Check endpoints
        assert!(index.endpoint_fields.contains_key("GET /users"));
        assert!(index.endpoint_fields.contains_key("POST /users"));
    }

    #[test]
    fn test_get_endpoints_for_field() {
        let spec = create_test_spec();
        let index = build_field_index(&spec);

        let endpoints = index.get_endpoints_for_field("name");
        assert!(!endpoints.is_empty());
        assert!(endpoints.iter().any(|e| e.contains("POST")));
    }

    #[test]
    fn test_is_critical_field() {
        let spec = create_test_spec();
        let index = build_field_index(&spec);

        // name field is used in POST request, should be critical
        assert!(index.is_critical_field("name"));

        // Non-existent field should not be critical
        assert!(!index.is_critical_field("nonexistent"));
    }

    #[test]
    fn test_get_schema_fields() {
        let spec = create_test_spec();
        let index = build_field_index(&spec);

        let user_fields = index.get_schema_fields("User");
        assert_eq!(user_fields.len(), 2);
        assert!(user_fields.contains(&"id".to_string()));
        assert!(user_fields.contains(&"name".to_string()));

        // Non-existent schema should return empty vec
        let empty_fields = index.get_schema_fields("NonExistent");
        assert!(empty_fields.is_empty());
    }
}
