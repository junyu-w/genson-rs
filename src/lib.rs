mod node;
mod strategy;
mod builder;

use rayon::prelude::*;
use mimalloc::MiMalloc;

// Setting the global allocator to mimalloc for more efficient memory allocation
#[global_allocator]
static GLOBAL: MiMalloc = MiMalloc;

pub use builder::SchemaBuilder;

pub fn get_builder(schema_uri: Option<&str>) -> SchemaBuilder {
    SchemaBuilder::new(schema_uri)
}

/// Parse a single JSON object and add it to the schema builder
/// * `builder` - the schema builder object
/// * `object_slice` - the JSON object to parse
/// * `should_drop` - whether to drop the parsed object after parsing. Setting to false will 
/// increase the performance of the function when the object is large, but the caller will be 
/// responsible for dropping the object
pub fn build_single_json_object_schema(builder: &mut SchemaBuilder, object_slice: &mut [u8], should_drop: bool) {
    let object = simd_json::to_borrowed_value(object_slice).unwrap();
    builder.add_object(&object);

    if !should_drop {
        std::mem::forget(object);
    }
}

fn trim_whitespace(data: &mut [u8]) -> &mut [u8] {
    let start = data.iter().position(|&c| !c.is_ascii_whitespace()).unwrap_or(data.len());
    // if the data is empty or only contains whitespace
    if start == data.len() {
        return &mut data[start..];
    }
    let end = data.iter().rev().position(|&c| !c.is_ascii_whitespace()).map_or(0, |p| data.len() - p);
    &mut data[start..end]
}


/// Build a JSON schema from multiple JSON objects
/// * `builder` - the schema builder object
/// * `json_slice` - the JSON object to parse
/// * `delimiter` - the delimiter to split the JSON objects
pub fn build_multi_json_objects_schema(builder: &mut SchemaBuilder, json_slice: &mut Vec<u8>, delimiter: u8) {
    let combined_builder = json_slice
        .par_split_mut(|byte| *byte == delimiter)
        .fold(
            || SchemaBuilder::new(None),
            |mut chunk_builder, object| {
                let trimmed_object = trim_whitespace(object);
                if trimmed_object.is_empty() {
                    return chunk_builder;
                }
                build_single_json_object_schema(&mut chunk_builder, trimmed_object, true);
                chunk_builder
            },
        ).reduce_with(|mut builder1, builder2| {
            builder1.add_schema(builder2.to_schema());
            builder1
        }).unwrap_or(SchemaBuilder::new(None));

    builder.add_schema(combined_builder.to_schema());
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_trim_whitespace_non_empty() {
        let mut data = br#"   {"name": "John", "age": 30}   "#.to_vec();
        let trimmed_data = trim_whitespace(&mut data);
        let expected_data = br#"{"name": "John", "age": 30}"#.to_vec();
        assert_eq!(trimmed_data, expected_data);
    }

    #[test]
    fn test_trim_whitespace_empty() {
        let mut data = br#"   "#.to_vec();
        let trimmed_data = trim_whitespace(&mut data);
        let expected_data = br#""#.to_vec();
        assert_eq!(trimmed_data, expected_data);
    }
}