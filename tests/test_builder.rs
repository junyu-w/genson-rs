use genson_rs::{build_json_schema, get_builder, BuildConfig};
use serde_json::json;

#[test]
fn test_anyof_should_include_all_scalar_field_types() {
    let config = BuildConfig {
        delimiter: None,
        ignore_outer_array: false,
    };
    let mut builder = get_builder(None);
    let mut test_object = json!(
      [
        {"multi": 1},
        {"multi": 2.5},
        {"multi": null},
        {"multi": "string"},
      ]
    ).to_string().into_bytes();
    let schema = build_json_schema(&mut builder, &mut test_object, &config);

    let expected_schema = json!({
      "type": "array",
      "items": {
        "type": "object",
        "properties": {
          "multi": {
            "type": [
              "null",
              "number",
              "string"
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

#[test]
fn test_field_should_be_optional_when_not_present_in_all_objects() {
  let config = BuildConfig {
    delimiter: None,
    ignore_outer_array: false,
};
  let mut builder = get_builder(None);
    let mut test_object = json!(
      [
        {"field_A": 1},
        {"field_B": 2.5},
      ]
    ).to_string().into_bytes();
    let schema = build_json_schema(&mut builder, &mut test_object, &config);

    let expected_schema = json!({
      "type": "array",
      "items": {
        "type": "object",
        "properties": {
          "field_A": {
            "type": "integer",
          },
          "field_B": {
            "type": "number",
          }
        }
      }
    });
    assert_eq!(schema, expected_schema);
}

#[test]
fn test_field_should_be_required_when_present_in_all_objects() {
  let config = BuildConfig {
    delimiter: None,
    ignore_outer_array: false,
};
  let mut builder = get_builder(None);
    let mut test_object = json!(
      [
        {"field_A": "test_string"},
        {"field_A": "test_string"},
      ]
    ).to_string().into_bytes();
    let schema = build_json_schema(&mut builder, &mut test_object, &config);

    let expected_schema = json!({
      "type": "array",
      "items": {
        "type": "object",
        "properties": {
          "field_A": {
            "type": "string",
          }
        },
        "required": [
          "field_A"
        ]
      }
    });
    assert_eq!(schema, expected_schema);
}

#[test]
fn test_field_type_should_be_integer_if_all_values_are_ints() {
  let config = BuildConfig {
    delimiter: None,
    ignore_outer_array: false,
};
  let mut builder = get_builder(None);
    let mut test_object = json!(
      [
        {"field_A": 1},
        {"field_A": 2},
        {"field_A": 3},
      ]
    ).to_string().into_bytes();
    let schema = build_json_schema(&mut builder, &mut test_object, &config);

    let expected_schema = json!({
      "type": "array",
      "items": {
        "type": "object",
        "properties": {
          "field_A": {
            "type": "integer",
          }
        },
        "required": [
          "field_A"
        ]
      }
    });
    assert_eq!(schema, expected_schema);
}


#[test]
fn test_field_type_should_be_number_if_values_include_float() {
  let config = BuildConfig {
    delimiter: None,
    ignore_outer_array: false,
};
  let mut builder = get_builder(None);
    let mut test_object = json!(
      [
        {"field_A": 1},
        {"field_A": 2.5},
        {"field_A": 3},
      ]
    ).to_string().into_bytes();
    let schema = build_json_schema(&mut builder, &mut test_object, &config);

    let expected_schema = json!({
      "type": "array",
      "items": {
        "type": "object",
        "properties": {
          "field_A": {
            "type": "number",
          }
        },
        "required": [
          "field_A"
        ]
      }
    });
    assert_eq!(schema, expected_schema);
}

#[test]
fn test_schema_should_include_all_fields_that_are_present() {
  let config = BuildConfig {
    delimiter: None,
    ignore_outer_array: false,
};
  let mut builder = get_builder(None);
    let mut test_object = json!(
      [
        {"field_A": 1, "field_B": "test_string"},
        {"field_A": 2, "field_B": "test_string"},
        {"field_A": 3, "field_B": "test_string"},
      ]
    ).to_string().into_bytes();
    let schema = build_json_schema(&mut builder, &mut test_object, &config);

    let expected_schema = json!({
      "type": "array",
      "items": {
        "type": "object",
        "properties": {
          "field_A": {
            "type": "integer",
          },
          "field_B": {
            "type": "string",
          }
        },
        "required": [
          "field_A",
          "field_B"
        ]
      }
    });
    assert_eq!(schema, expected_schema);
}

#[test]
fn test_field_should_be_correct_object_type_when_its_nested_json() {
  let config = BuildConfig {
    delimiter: None,
    ignore_outer_array: false,
};
  let mut builder = get_builder(None);
    let mut test_object = json!(
      [
        {"field_A": {"nested_field": 1}},
        {"field_A": {"nested_field": 2}},
        {"field_A": {"nested_field": 3}},
      ]
    ).to_string().into_bytes();
    let schema = build_json_schema(&mut builder, &mut test_object, &config);

    let expected_schema = json!({
      "type": "array",
      "items": {
        "type": "object",
        "properties": {
          "field_A": {
            "type": "object",
            "properties": {
              "nested_field": {
                "type": "integer",
              }
            },
            "required": [
              "nested_field"
            ]
          }
        },
        "required": [
          "field_A"
        ]
      }
    });
    assert_eq!(schema, expected_schema);
}

#[test]
fn test_field_should_be_correct_array_type_when_its_an_array() {
  let config = BuildConfig {
    delimiter: None,
    ignore_outer_array: false,
};
  let mut builder = get_builder(None);
    let mut test_object = json!(
      [
        {"field_A": [1, 2, 5.5]},
        {"field_A": [1, 2, 3]},
        {"field_A": [1, 2, 3]},
      ]
    ).to_string().into_bytes();
    let schema = build_json_schema(&mut builder, &mut test_object, &config);

    let expected_schema = json!({
      "type": "array",
      "items": {
        "type": "object",
        "properties": {
          "field_A": {
            "type": "array",
            "items": {
              "type": "number",
            }
          }
        },
        "required": [
          "field_A"
        ]
      }
    });
    assert_eq!(schema, expected_schema);
}

#[test]
fn test_field_should_be_correct_tuple_type_when_its_an_array_of_different_value_types() {
  let config = BuildConfig {
    delimiter: None,
    ignore_outer_array: false,
};
  let mut builder = get_builder(None);
    let mut test_object = json!(
      [
        {"field_A": [1, "string", 5.5]},
        {"field_A": [1, "string", 3]},
        {"field_A": [1, "string", 3]},
      ]
    ).to_string().into_bytes();
    let schema = build_json_schema(&mut builder, &mut test_object, &config);

    let expected_schema = json!({
      "type": "array",
      "items": {
        "type": "object",
        "properties": {
          "field_A": {
            "type": "array",
            "items": {
              "type": [
                "number",
                "string",
              ]
            }
          }
        },
        "required": [
          "field_A"
        ]
      }
    });
    assert_eq!(schema, expected_schema);
}

#[test]
fn test_schema_should_be_correct_when_building_from_multiple_objects() {
  let config = BuildConfig {
    delimiter: Some("\n".as_bytes()[0]),
    ignore_outer_array: false,
};
  let mut builder = get_builder(None);
    let mut test_object = r#"
      {"field_A": 1, "field_B": "test_string"}
      {"field_A": 2, "field_B": "test_string"}
      {"field_A": 3, "field_B": "test_string"}
      {"field_A": 4, "field_B": "test_string"}
      {"field_A": 5, "field_B": "test_string"}
      {"field_A": 6, "field_B": "test_string"}
      {"field_A": 7, "field_B": "test_string"}
      {"field_A": 8, "field_B": "test_string"}
      {"field_A": 9, "field_B": "test_string"}
      {"field_A": 10, "field_B": "test_string"}
    "#.to_string().into_bytes();
    let schema = build_json_schema(&mut builder, &mut test_object, &config);

    let expected_schema = json!({
      "type": "object",
      "properties": {
        "field_A": {
          "type": "integer",
        },
        "field_B": {
          "type": "string",
        }
      },
      "required": [
        "field_A",
        "field_B"
      ]
    });
    assert_eq!(schema, expected_schema);
}

#[test]
fn test_json_schema_should_not_contain_outer_array_when_ignore_outer_array_config_is_true() {
  let config = BuildConfig {
    delimiter: None,
    ignore_outer_array: true,
};
  let mut builder = get_builder(None);
    let mut test_object = r#"
    [
      {"field_A": 1, "field_B": "test_string"}
      {"field_A": 2, "field_B": "test_string"}
    ]"#.to_string().into_bytes();
    let schema = build_json_schema(&mut builder, &mut test_object, &config);

    let expected_schema = json!({
      "type": "object",
      "properties": {
        "field_A": {
          "type": "integer",
        },
        "field_B": {
          "type": "string",
        }
      },
      "required": [
        "field_A",
        "field_B"
      ]
    });
    assert_eq!(schema, expected_schema);
}