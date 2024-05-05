mod node;
mod strategy;
mod builder;

pub use builder::SchemaBuilder;

pub fn get_builder(schema_uri: Option<&str>) -> SchemaBuilder {
    SchemaBuilder::new(schema_uri)
}

pub fn add_schema_file(builder: &mut SchemaBuilder, file: &str) {
    let schema = std::fs::read_to_string(file).unwrap();
    let schema: serde_json::Value = serde_json::from_str(&schema).unwrap();
    builder.add_schema(schema);
}

pub fn add_object_file(builder: &mut SchemaBuilder, file: &str) {
    let object = std::fs::read_to_string(file).unwrap();
    let object: serde_json::Value = serde_json::from_str(&object).unwrap();
    builder.add_object(object);
}