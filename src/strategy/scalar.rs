use serde_json::{Value, json};

use crate::strategy::base::{SchemaStrategy, ScalarSchemaStrategy};

#[derive(Debug)]
pub struct NullStrategy {
    extra_keywords: Value,
}

impl NullStrategy {
    pub fn new() -> Self {
        NullStrategy {
            extra_keywords: json!({})
        }
    }
}

impl ScalarSchemaStrategy for NullStrategy {
    fn js_type() -> &'static str {
        "null"
    }
}

impl SchemaStrategy for NullStrategy {
    fn get_extra_keywords_mut(&mut self) -> &mut Value {
        &mut self.extra_keywords
    }

    fn get_extra_keywords(&self) -> &Value {
        &self.extra_keywords
    }

    fn match_schema(schema: &Value) -> bool {
        schema["type"] == "null"
    }

    fn match_object(object: &Value) -> bool {
        object.is_null()
    }

    fn add_object(&mut self, _object: &Value) {
        ()
    }
}

#[derive(Debug)]
pub struct BooleanStrategy {
    extra_keywords: Value,
}

impl BooleanStrategy {
    pub fn new() -> Self {
        BooleanStrategy {
            extra_keywords: json!({})
        }
    }
}

impl ScalarSchemaStrategy for BooleanStrategy {
    fn js_type() -> &'static str {
        "boolean"
    }
}

impl SchemaStrategy for BooleanStrategy {
    fn get_extra_keywords_mut(&mut self) -> &mut Value {
        &mut self.extra_keywords
    }

    fn get_extra_keywords(&self) -> &Value {
        &self.extra_keywords
    }

    fn match_schema(schema: &Value) -> bool {
        schema["type"] == "boolean"
    }

    fn match_object(object: &Value) -> bool {
        object.is_boolean()
    }

    fn add_object(&mut self, _object: &Value) {
        ()
    }
}

#[derive(Debug)]
pub struct StringStrategy {
    extra_keywords: Value,
}

impl StringStrategy {
    pub fn new() -> Self {
        StringStrategy {
            extra_keywords: json!({})
        }
    }
}

impl SchemaStrategy for StringStrategy {
    fn get_extra_keywords_mut(&mut self) -> &mut Value {
        &mut self.extra_keywords
    }

    fn get_extra_keywords(&self) -> &Value {
        &self.extra_keywords
    }

    fn match_schema(schema: &Value) -> bool {
        schema["type"] == "string"
    }

    fn match_object(object: &Value) -> bool {
        object.is_string()
    }

    fn add_object(&mut self, _object: &Value) {
        ()
    }
}

impl ScalarSchemaStrategy for StringStrategy {
    fn js_type() -> &'static str {
        "string"
    }
}

#[derive(Debug)]
pub struct NumberStrategy {
    number_type: &'static str,
    extra_keywords: Value,
}

impl NumberStrategy {
    pub fn new() -> Self {
        NumberStrategy {
            number_type: "integer",
            extra_keywords: json!({}),
        }
    }
}

impl ScalarSchemaStrategy for NumberStrategy {
    fn js_type() -> &'static str {
        "integer|number"
    }

    fn to_schema(&self) -> Value {
        let mut schema = SchemaStrategy::to_schema(self);
        schema["type"] = Value::String(self.number_type.to_string());
        schema
    }
}

impl SchemaStrategy for NumberStrategy {
    fn add_schema(&mut self, schema: &Value) {
        self.add_extra_keywords(schema);
        // "number" takes precedence over "integer" if both are present
        if schema["type"] == "number" {
            self.number_type = "number";
        }
    }

    fn add_object(&mut self, object: &Value) {
        if object.is_f64() {
            self.number_type = "number";
        }
    }

    fn get_extra_keywords_mut(&mut self) -> &mut Value {
        &mut self.extra_keywords
    }

    fn get_extra_keywords(&self) -> &Value {
        &self.extra_keywords
    }

    fn match_schema(schema: &Value) -> bool {
        schema["type"] == "number" || schema["type"] == "integer"
    }

    fn match_object(object: &Value) -> bool {
        object.is_number()
    }
}

/// schema strategy for schemas with no type. This is only used when
/// there is no other active strategy, and it will be merged into the
/// first typed strategy that gets added.
#[derive(Debug)]
pub struct TypelessStrategy {
    extra_keywords: Value,
}

impl TypelessStrategy {
    pub fn new() -> Self {
        TypelessStrategy {
            extra_keywords: json!({})
        }
    }
}

impl SchemaStrategy for TypelessStrategy {
    fn get_extra_keywords_mut(&mut self) -> &mut Value {
        &mut self.extra_keywords
    }

    fn get_extra_keywords(&self) -> &Value {
        &self.extra_keywords
    }

    fn match_schema(schema: &Value) -> bool {
        if let Value::Object(obj) = schema {
            return !obj.contains_key("type");
        }
        return true;
    }

    fn match_object(_: &Value) -> bool {
        false
    }

    fn add_object(&mut self, _object: &Value) {
        ()
    }
}