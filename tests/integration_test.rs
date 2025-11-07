use openapi_explorer::indexer;
use openapi_explorer::parser;
use std::path::Path;

#[tokio::test]
async fn test_parse_and_index_petstore() {
    // Test with the example petstore file
    let path = Path::new("examples/petstore.json");

    if !path.exists() {
        println!("Skipping test: petstore.json not found");
        return;
    }

    // Parse the OpenAPI spec
    let spec_result = parser::parse_openapi(path).await;
    assert!(spec_result.is_ok(), "Failed to parse petstore.json");

    let spec = spec_result.unwrap();

    // Verify basic spec properties
    assert_eq!(spec.openapi, "3.0.0");
    assert!(!spec.paths.is_empty(), "Petstore should have paths");

    // Build index
    let index = indexer::build_field_index(&spec);

    // Verify index was built
    assert!(!index.fields.is_empty(), "Index should have fields");
    assert!(!index.schemas.is_empty(), "Index should have schemas");

    println!(
        "Successfully parsed and indexed {} schemas with {} fields",
        index.schemas.len(),
        index.fields.len()
    );
}

#[tokio::test]
async fn test_parse_invalid_file() {
    let path = Path::new("nonexistent/file.json");
    let result = parser::parse_openapi(path).await;
    assert!(result.is_err(), "Should fail on nonexistent file");
}

#[test]
fn test_field_relationships() {
    use openapi_explorer::parser::{Components, Info, OpenApiSpec, Schema};
    use std::collections::HashMap;

    let spec = OpenApiSpec {
        openapi: "3.0.0".to_string(),
        info: Info {
            title: "Test".to_string(),
            version: "1.0.0".to_string(),
            description: None,
        },
        paths: HashMap::new(),
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
                                ..Default::default()
                            },
                        ),
                        (
                            "name".to_string(),
                            Schema {
                                schema_type: Some("string".to_string()),
                                ..Default::default()
                            },
                        ),
                    ])),
                    ..Default::default()
                },
            )])),
        }),
    };

    let index = indexer::build_field_index(&spec);
    let relationships = indexer::analyze_field_relationships(&index);

    // id and name should be related (in same schema)
    assert!(relationships.contains_key("id"));
    assert!(relationships.contains_key("name"));

    // id should be related to name and vice versa
    let id_related = &relationships["id"];
    assert!(id_related.contains(&"name".to_string()));

    let name_related = &relationships["name"];
    assert!(name_related.contains(&"id".to_string()));
}
