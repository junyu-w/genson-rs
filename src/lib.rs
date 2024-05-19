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
pub fn build_single_json_object_schema(builder: &mut SchemaBuilder, object_slice: &mut [u8]) {
    let object = simd_json::to_borrowed_value(object_slice).unwrap();
    builder.add_object(&object);
}

/// Parse a JSON schema from a JSON object or an array of JSON objects and add it to the schema builder.
/// Passing delimiter will split the JSON objects more efficiently in parallel, and should result in a 2x speedup
/// for large JSON object files (either multiple JSON objects concatenated together or a large JSON array).
/// * `builder` - the schema builder object
/// * `json_slice` - the JSON object or array of JSON objects to parse
/// * `delimiter` - the delimiter to split the JSON objects
pub fn build_json_schema(builder: &mut SchemaBuilder, json_slice: &mut Vec<u8>, delimiter: Option<u8>) {
    let json_slice = trim_to_object(json_slice);
    if is_json_object_array(json_slice) {
        let array_elements = get_json_array_elements(json_slice);
        build_multi_json_objects_schema(builder, array_elements, None);
        // TODO: add the outer array schema back
    } else {
        build_multi_json_objects_schema(builder, json_slice, delimiter);
    }
}

/// Build a JSON schema from multiple JSON objects
fn build_multi_json_objects_schema(builder: &mut SchemaBuilder, json_slice: &mut [u8], delimiter: Option<u8>) {
    if let Some(delimiter) = delimiter {
        let object_iter = json_slice.par_split_mut(|byte| *byte == delimiter);
        add_schema_from_object_par_iter(object_iter, builder);
    } else {
        let mut structure_count = 0;
        let object_separator = move |byte: &u8| -> bool{
            if *byte == b'{' {
                structure_count += 1;
            } else if *byte == b'}' {
                structure_count -= 1;
            }
            if structure_count == 0 && *byte == b'}' {
                return true;
            } else {
                return false
            }
        };
        // an vector of pointers to each of the individual JSON object bytes in the data
        let mut object_slices: Vec<&mut [u8]> = vec![];
        json_slice
            .split_inclusive_mut(object_separator)
            .for_each(|slice| object_slices.push(slice));
        let object_iter = object_slices.into_par_iter();
        add_schema_from_object_par_iter(object_iter, builder);
    }
}

fn add_schema_from_object_par_iter<'a>(object_iter: impl ParallelIterator<Item = &'a mut [u8]>, builder: &mut SchemaBuilder) {
    let combined_builder = object_iter
    .fold(
        || SchemaBuilder::new(None),
        |mut chunk_builder, object| {
            let trimmed_object = trim_to_object(object);
            if trimmed_object.is_empty() {
                return chunk_builder;
            }
            build_single_json_object_schema(&mut chunk_builder, trimmed_object);
            chunk_builder
        },
    ).reduce_with(|mut builder1, builder2| {
        builder1.add_schema(builder2.to_schema());
        builder1
    }).unwrap_or(SchemaBuilder::new(None));

    builder.add_schema(combined_builder.to_schema());
}

/// trim the whitespace and non-JSON object characters from the start and end of the data
fn trim_to_object(data: &mut [u8]) -> &mut [u8] {
    let start = data.iter()
        .position(|&c| !c.is_ascii_whitespace() && (c == b'{' || c == b'['))
        .unwrap_or(data.len());
    // if the data is empty or only contains whitespace
    if start == data.len() {
        return &mut data[start..];
    }
    let end = data.iter()
        .rev()
        .position(|&c| !c.is_ascii_whitespace() && (c == b'}' || c == b']'))
        .map_or(0, |p| data.len() - p);
    &mut data[start..end]
}

/// Check if the data is a JSON object array, this function assumes that the data is trimmed
/// with `trim_to_object` before calling it
fn is_json_object_array(data: &[u8]) -> bool {
    return data[0] == b'[' && data[data.len() - 1] == b']';
}

/// Get only the JSON array elements from the JSON array data slice, excluding the square brackets.
/// This function assumes that the data is a JSON array (i.e. call `is_json_object_array` before calling it)
fn get_json_array_elements(data: &mut [u8]) -> &mut [u8] {
    let start = data.iter().position(|&c| c == b'[').unwrap_or(data.len());
    let end = data.iter().rposition(|&c| c == b']').map_or(0, |p| p);
    &mut data[start + 1..end]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_trim_whitespace_non_empty() {
        let mut data = br#"   {"name": "John", "age": 30}   "#.to_vec();
        let trimmed_data = trim_to_object(&mut data);
        let expected_data = br#"{"name": "John", "age": 30}"#.to_vec();
        assert_eq!(trimmed_data, expected_data);
    }

    #[test]
    fn test_trim_whitespace_empty() {
        let mut data = br#"   "#.to_vec();
        let trimmed_data = trim_to_object(&mut data);
        let expected_data = br#""#.to_vec();
        assert_eq!(trimmed_data, expected_data);
    }

    #[test]
    fn test_trim_non_json_array_characters() {
        let mut data = br#"\n  , [1, 2, 3]   \b\t"#.to_vec();
        let trimmed_data = trim_to_object(&mut data);
        let expected_data = br#"[1, 2, 3]"#.to_vec();
        assert_eq!(trimmed_data, expected_data);
    }

    #[test]
    fn test_trim_non_json_object_characters() {
        let mut data = br#"\n  , {"name": "John", "age": 30}   \b\t"#.to_vec();
        let trimmed_data = trim_to_object(&mut data);
        let expected_data = br#"{"name": "John", "age": 30}"#.to_vec();
        assert_eq!(trimmed_data, expected_data);
    }

    #[test]
    fn test_is_json_object_array() {
        let data = br#"[{"name": "John", "age": 30}]"#.to_vec();
        assert_eq!(is_json_object_array(&data), true);
    }

    #[test]
    fn test_get_json_array_elements_single_elements() {
        let mut data = br#"[{"name": "John", "age": 30}]"#.to_vec();
        let elements = get_json_array_elements(&mut data);
        let expected_elements = br#"{"name": "John", "age": 30}"#.to_vec();
        assert_eq!(elements, expected_elements);
    }

    #[test]
    fn test_get_json_array_elements_multiple_elements() {
        let mut data = br#"[{"name": "John", "age": 30}, {"name": "Joe", "age": 20}]"#.to_vec();
        let elements = get_json_array_elements(&mut data);
        let expected_elements = br#"{"name": "John", "age": 30}, {"name": "Joe", "age": 20}"#.to_vec();
        assert_eq!(elements, expected_elements);
    }


}