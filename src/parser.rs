use anyhow::{anyhow, Result};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use tokio::fs;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OpenApiSpec {
    pub openapi: String,
    pub info: Info,
    pub paths: HashMap<String, PathItem>,
    pub components: Option<Components>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Info {
    pub title: String,
    pub version: String,
    pub description: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PathItem {
    #[serde(flatten)]
    pub operations: HashMap<String, Operation>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Operation {
    pub operation_id: Option<String>,
    pub summary: Option<String>,
    pub description: Option<String>,
    pub tags: Option<Vec<String>>,
    pub parameters: Option<Vec<Parameter>>,
    pub request_body: Option<RequestBody>,
    pub responses: HashMap<String, Response>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Parameter {
    pub name: String,
    #[serde(rename = "in")]
    pub in_: String,
    pub description: Option<String>,
    pub required: Option<bool>,
    pub schema: Option<Schema>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RequestBody {
    pub description: Option<String>,
    pub content: HashMap<String, MediaType>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Response {
    pub description: String,
    pub content: Option<HashMap<String, MediaType>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MediaType {
    pub schema: Option<Schema>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Components {
    pub schemas: Option<HashMap<String, Schema>>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct Schema {
    #[serde(rename = "type")]
    pub schema_type: Option<String>,
    pub format: Option<String>,
    pub description: Option<String>,
    pub properties: Option<HashMap<String, Schema>>,
    pub items: Option<Box<Schema>>,
    pub required: Option<Vec<String>>,
    pub all_of: Option<Vec<Schema>>,
    pub one_of: Option<Vec<Schema>>,
    pub any_of: Option<Vec<Schema>>,
    pub not: Option<Box<Schema>>,
    pub additional_properties: Option<Box<Schema>>,
    pub nullable: Option<bool>,
    pub read_only: Option<bool>,
    pub write_only: Option<bool>,
    pub example: Option<serde_json::Value>,
    pub enum_: Option<Vec<serde_json::Value>>,
    pub default: Option<serde_json::Value>,
    #[serde(rename = "$ref")]
    pub reference: Option<String>,
}

pub async fn parse_openapi(file_path: &std::path::Path) -> Result<OpenApiSpec> {
    if !file_path.exists() {
        return Err(anyhow!("OpenAPI file not found: {}", file_path.display()));
    }

    let content = fs::read_to_string(file_path).await?;

    // Try to parse as JSON first
    if file_path.extension().and_then(|s| s.to_str()) == Some("json") {
        let spec: OpenApiSpec = serde_json::from_str(&content)
            .map_err(|e| anyhow!("Failed to parse OpenAPI JSON: {}", e))?;
        return Ok(spec);
    }

    // Try YAML (for now, just attempt JSON parsing - YAML support can be added later)
    let spec: OpenApiSpec = serde_json::from_str(&content)
        .map_err(|e| anyhow!("Failed to parse OpenAPI file: {}", e))?;

    Ok(spec)
}

pub async fn parse_openapi_or_default(
    file_path: &Option<std::path::PathBuf>,
) -> Result<OpenApiSpec> {
    match file_path {
        Some(path) => parse_openapi(path).await,
        None => {
            // Try to find a default OpenAPI file in examples/
            let default_path = std::path::Path::new("examples/petstore.json");
            if default_path.exists() {
                parse_openapi(default_path).await
            } else {
                Err(anyhow!(
                    "No OpenAPI file specified and no default file found"
                ))
            }
        }
    }
}

pub fn resolve_references(spec: &mut OpenApiSpec) -> Result<()> {
    if let Some(components) = &mut spec.components {
        if let Some(schemas) = &mut components.schemas {
            resolve_schema_references(schemas)?;
        }
    }

    // Resolve references in paths
    let spec_clone = spec.clone();
    for (_path, path_item) in &mut spec.paths {
        for (_method, operation) in &mut path_item.operations {
            resolve_operation_references(operation, &spec_clone)?;
        }
    }

    Ok(())
}

fn resolve_schema_references(schemas: &mut HashMap<String, Schema>) -> Result<()> {
    let schema_names: Vec<String> = schemas.keys().cloned().collect();

    for schema_name in schema_names {
        let schemas_clone = schemas.clone();
        if let Some(schema) = schemas.get_mut(&schema_name) {
            resolve_schema_refs_recursive(schema, &schemas_clone)?;
        }
    }

    Ok(())
}

fn resolve_schema_refs_recursive(
    schema: &mut Schema,
    all_schemas: &HashMap<String, Schema>,
) -> Result<()> {
    // Resolve reference if present
    if let Some(ref_path) = &schema.reference {
        if let Some(target_name) = extract_schema_name_from_ref(ref_path) {
            if let Some(target_schema) = all_schemas.get(target_name) {
                // Copy properties from referenced schema
                if schema.properties.is_none() {
                    schema.properties = target_schema.properties.clone();
                }
                if schema.schema_type.is_none() {
                    schema.schema_type = target_schema.schema_type.clone();
                }
                if schema.description.is_none() {
                    schema.description = target_schema.description.clone();
                }
                if schema.required.is_none() {
                    schema.required = target_schema.required.clone();
                }
            }
        }
        schema.reference = None;
    }

    // Resolve nested references
    if let Some(properties) = &mut schema.properties {
        for (_field_name, field_schema) in properties {
            resolve_schema_refs_recursive(field_schema, all_schemas)?;
        }
    }

    if let Some(items) = &mut schema.items {
        resolve_schema_refs_recursive(items, all_schemas)?;
    }

    if let Some(all_of) = &mut schema.all_of {
        for sub_schema in all_of {
            resolve_schema_refs_recursive(sub_schema, all_schemas)?;
        }
    }

    if let Some(one_of) = &mut schema.one_of {
        for sub_schema in one_of {
            resolve_schema_refs_recursive(sub_schema, all_schemas)?;
        }
    }

    if let Some(any_of) = &mut schema.any_of {
        for sub_schema in any_of {
            resolve_schema_refs_recursive(sub_schema, all_schemas)?;
        }
    }

    Ok(())
}

fn resolve_operation_references(operation: &mut Operation, spec: &OpenApiSpec) -> Result<()> {
    // Resolve parameter references
    if let Some(parameters) = &mut operation.parameters {
        for parameter in parameters {
            if let Some(schema) = &mut parameter.schema {
                resolve_parameter_schema_refs(schema, spec)?;
            }
        }
    }

    // Resolve request body references
    if let Some(request_body) = &mut operation.request_body {
        for (_content_type, media_type) in &mut request_body.content {
            if let Some(schema) = &mut media_type.schema {
                resolve_parameter_schema_refs(schema, spec)?;
            }
        }
    }

    // Resolve response references
    for (_status_code, response) in &mut operation.responses {
        if let Some(content) = &mut response.content {
            for (_content_type, media_type) in content {
                if let Some(schema) = &mut media_type.schema {
                    resolve_parameter_schema_refs(schema, spec)?;
                }
            }
        }
    }

    Ok(())
}

fn resolve_parameter_schema_refs(schema: &mut Schema, spec: &OpenApiSpec) -> Result<()> {
    if let Some(ref_path) = &schema.reference {
        if let Some(target_name) = extract_schema_name_from_ref(ref_path) {
            if let Some(components) = &spec.components {
                if let Some(schemas) = &components.schemas {
                    if let Some(target_schema) = schemas.get(target_name) {
                        // Create a copy of the target schema
                        let mut resolved_schema = target_schema.clone();
                        resolved_schema.reference = None;
                        *schema = resolved_schema;
                    }
                }
            }
        }
    }

    // Recursively resolve nested references
    if let Some(properties) = &mut schema.properties {
        for (_field_name, field_schema) in properties {
            resolve_parameter_schema_refs(field_schema, spec)?;
        }
    }

    if let Some(items) = &mut schema.items {
        resolve_parameter_schema_refs(items, spec)?;
    }

    Ok(())
}

fn extract_schema_name_from_ref(ref_path: &str) -> Option<&str> {
    if ref_path.starts_with("#/components/schemas/") {
        Some(&ref_path[21..])
    } else {
        None
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_extract_schema_name_from_ref_valid() {
        let result = extract_schema_name_from_ref("#/components/schemas/Pet");
        assert_eq!(result, Some("Pet"));
    }

    #[test]
    fn test_extract_schema_name_from_ref_invalid() {
        let result = extract_schema_name_from_ref("invalid/ref");
        assert_eq!(result, None);
    }

    #[test]
    fn test_extract_schema_name_from_ref_empty() {
        let result = extract_schema_name_from_ref("");
        assert_eq!(result, None);
    }

    #[test]
    fn test_schema_get_field_names_simple() {
        let schema = Schema {
            schema_type: Some("object".to_string()),
            format: None,
            description: None,
            properties: Some(std::collections::HashMap::from([
                (
                    "name".to_string(),
                    Schema {
                        schema_type: Some("string".to_string()),
                        ..Default::default()
                    },
                ),
                (
                    "age".to_string(),
                    Schema {
                        schema_type: Some("integer".to_string()),
                        ..Default::default()
                    },
                ),
            ])),
            items: None,
            required: None,
            all_of: None,
            one_of: None,
            any_of: None,
            not: None,
            additional_properties: None,
            nullable: None,
            read_only: None,
            write_only: None,
            example: None,
            enum_: None,
            default: None,
            reference: None,
        };

        let field_names = schema.get_field_names();
        assert_eq!(field_names.len(), 2);
        assert!(field_names.contains(&"name".to_string()));
        assert!(field_names.contains(&"age".to_string()));
    }

    #[tokio::test]
    async fn test_parse_openapi_valid_json() {
        use std::io::Write;
        use tempfile::NamedTempFile;

        let mut temp_file = NamedTempFile::new().unwrap();
        let openapi_content = r#"{
            "openapi": "3.0.0",
            "info": {
                "title": "Test API",
                "version": "1.0.0"
            },
            "paths": {}
        }"#;

        temp_file.write_all(openapi_content.as_bytes()).unwrap();
        temp_file.flush().unwrap();

        let result = parse_openapi(temp_file.path()).await;
        assert!(result.is_ok());

        let spec = result.unwrap();
        assert_eq!(spec.openapi, "3.0.0");
        assert_eq!(spec.info.title, "Test API");
        assert_eq!(spec.info.version, "1.0.0");
    }

    #[tokio::test]
    async fn test_parse_openapi_file_not_found() {
        use std::path::Path;

        let result = parse_openapi(Path::new("/nonexistent/file.json")).await;
        assert!(result.is_err());

        let error_msg = result.unwrap_err().to_string();
        assert!(error_msg.contains("OpenAPI file not found"));
    }
}

impl Schema {
    pub fn get_field_names(&self) -> Vec<String> {
        let mut fields = Vec::new();

        if let Some(properties) = &self.properties {
            fields.extend(properties.keys().cloned());
        }

        // Handle nested schemas
        if let Some(items) = &self.items {
            fields.extend(items.get_field_names());
        }

        // Handle composition
        if let Some(all_of) = &self.all_of {
            for schema in all_of {
                fields.extend(schema.get_field_names());
            }
        }

        if let Some(one_of) = &self.one_of {
            for schema in one_of {
                fields.extend(schema.get_field_names());
            }
        }

        if let Some(any_of) = &self.any_of {
            for schema in any_of {
                fields.extend(schema.get_field_names());
            }
        }

        fields
    }

    pub fn get_field_type(&self, field_name: &str) -> Option<String> {
        if let Some(properties) = &self.properties {
            if let Some(schema) = properties.get(field_name) {
                return schema.schema_type.clone();
            }
        }
        None
    }

    pub fn get_field_description(&self, field_name: &str) -> Option<String> {
        if let Some(properties) = &self.properties {
            if let Some(schema) = properties.get(field_name) {
                return schema.description.clone();
            }
        }
        None
    }

    pub fn get_field_format(&self, field_name: &str) -> Option<String> {
        if let Some(properties) = &self.properties {
            if let Some(schema) = properties.get(field_name) {
                return schema.format.clone();
            }
        }
        None
    }

    pub fn is_field_required(&self, field_name: &str) -> bool {
        if let Some(required) = &self.required {
            required.contains(&field_name.to_string())
        } else {
            false
        }
    }

    pub fn get_field_enum_values(&self, field_name: &str) -> Option<Vec<serde_json::Value>> {
        if let Some(properties) = &self.properties {
            if let Some(schema) = properties.get(field_name) {
                return schema.enum_.clone();
            }
        }
        None
    }
}
