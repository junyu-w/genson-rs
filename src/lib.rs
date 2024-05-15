mod node;
mod strategy;
mod builder;

use std::time::Instant;
use mimalloc::MiMalloc;

// Setting the global allocator to mimalloc for more efficient memory allocation
#[global_allocator]
static GLOBAL: MiMalloc = MiMalloc;

pub use builder::SchemaBuilder;

pub fn get_builder(schema_uri: Option<&str>) -> SchemaBuilder {
    SchemaBuilder::new(schema_uri)
}

/// Parse a JSON object and add it to the schema builder
/// * `builder` - the schema builder object
/// * `object_slice` - the JSON object to parse
/// * `verbose_mode` - whether to print verbose output (to stderr)
pub fn parse_json_schema(builder: &mut SchemaBuilder, object_slice: &mut Vec<u8>, verbose_mode: bool) {
    let now = Instant::now();

    let object = simd_json::to_borrowed_value(object_slice).unwrap();
    let json_parsing_duration_ms = now.elapsed().as_millis();

    builder.add_object(&object);
    let schema_parsing_duration = now.elapsed().as_millis() - json_parsing_duration_ms;

    if verbose_mode {
        eprintln!("JSON parsing duration: {}ms", json_parsing_duration_ms);
        eprintln!("Schema parsing duration: {}ms", schema_parsing_duration);
    }
}