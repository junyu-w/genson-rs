use genson_rs::get_builder;
use serde_json::json;

#[test]
fn test_add_object_file() {
    let mut builder = get_builder(None);
    let test_object = json!([
        {"multi": 1},
        {"multi": null},
        {"multi": "string"},
        {"multi": {"key": "value"}}
    ]);
    builder.add_object(test_object);

    let schema = builder.to_schema();
    let expected_schema = json!({
        "$schema": "http://json-schema.org/schema#",
        "type": "array",
        "items": {
          "type": "object",
          "properties": {
            "multi": {
              "anyOf": [
                {"type": ["integer", "null", "string"]},
                {
                  "type": "object",
                  "properties": {"key": {"type": "string"}},
                  "required": ["key"]
                }
              ]
            }
          },
          "required": [
            "multi"
          ]
        }
    });
    assert_eq!(schema, expected_schema);
}