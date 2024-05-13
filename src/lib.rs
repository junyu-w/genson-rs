mod node;
mod strategy;
mod builder;

pub use builder::SchemaBuilder;
use serde_json::Value;

pub fn get_builder(schema_uri: Option<&str>) -> SchemaBuilder {
    SchemaBuilder::new(schema_uri)
}

pub fn parse_json_object(json_file: &str) -> Value {
    let object_str = std::fs::read_to_string(json_file).unwrap();
    let object: Value = serde_json::from_str(&object_str).unwrap();
    return object;
}
