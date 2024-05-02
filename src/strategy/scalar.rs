use serde_json::{Value, json};

use crate::strategy::base::{SchemaStrategy, TypelessSchemaStrategy, ScalarType};

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

impl TypelessSchemaStrategy for NullStrategy {
    fn js_type() -> &'static str {
        "null"
    }

    fn rs_type() -> ScalarType {
        ScalarType::Null
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
        <Self as TypelessSchemaStrategy>::match_schema(schema)
    }

    fn match_object(object: &Value) -> bool {
        <Self as TypelessSchemaStrategy>::match_object(object)
    }
}

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

impl TypelessSchemaStrategy for BooleanStrategy {
    fn js_type() -> &'static str {
        "boolean"
    }

    fn rs_type() -> ScalarType {
        ScalarType::Boolean
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
        <Self as TypelessSchemaStrategy>::match_schema(schema)
    }

    fn match_object(object: &Value) -> bool {
        <Self as TypelessSchemaStrategy>::match_object(object)
    }
}


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
        <Self as TypelessSchemaStrategy>::match_schema(schema)
    }

    fn match_object(object: &Value) -> bool {
        <Self as TypelessSchemaStrategy>::match_object(object)
    }
}

impl TypelessSchemaStrategy for StringStrategy {
    fn js_type() -> &'static str {
        "string"
    }

    fn rs_type() -> ScalarType {
        ScalarType::String
    }
}

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

impl TypelessSchemaStrategy for NumberStrategy {
    fn js_type() -> &'static str {
        "integer|number"
    }

    fn rs_type() -> ScalarType {
        ScalarType::Number
    }

    fn to_schema(&self) -> Value {
        let mut schema = SchemaStrategy::to_schema(self);
        schema["type"] = Value::String(self.number_type.to_string());
        schema
    
    }
}

impl SchemaStrategy for NumberStrategy {
    fn add_schema(&mut self, schema: &Value) {
        if schema["type"] == "number" {
            self.number_type = "number";
        }
        SchemaStrategy::add_schema(self, schema);
    }

    fn add_object(&mut self, object: &Value) {
        if object.is_number() {
            if let Some(_) = object.as_i64() {
                self.number_type = "number";
            }
        }
    }

    fn get_extra_keywords_mut(&mut self) -> &mut Value {
        &mut self.extra_keywords
    }

    fn get_extra_keywords(&self) -> &Value {
        &self.extra_keywords
    }

    fn match_schema(schema: &Value) -> bool {
        <Self as TypelessSchemaStrategy>::match_schema(schema)
    }

    fn match_object(object: &Value) -> bool {
        <Self as TypelessSchemaStrategy>::match_object(object)
    }
}

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
}