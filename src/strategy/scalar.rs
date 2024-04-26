use json::JsonValue;

use crate::strategy::base::{SchemaStrategy, TypelessSchemaStrategy, ScalarType};

pub struct NullStrategy {
    extra_keywords: JsonValue,
}

impl NullStrategy {
    pub fn new() -> Self {
        NullStrategy {
            extra_keywords: json::object! {}
        }
    }
}

impl TypelessSchemaStrategy for NullStrategy {
    fn js_type(&self) -> &'static str {
        "null"
    }

    fn rs_type(&self) -> ScalarType {
        ScalarType::Null
    }
}

impl SchemaStrategy for NullStrategy {
    fn get_extra_keywords_mut(&mut self) -> &mut JsonValue {
        &mut self.extra_keywords
    }

    fn get_extra_keywords(&self) -> &JsonValue {
        &self.extra_keywords
    }

    fn match_schema(&self, schema: JsonValue) -> bool {
        TypelessSchemaStrategy::match_schema(self, schema)
    }

    fn match_object(&self, object: JsonValue) -> bool {
        TypelessSchemaStrategy::match_object(self, object)
    }
}

pub struct BooleanStrategy {
    extra_keywords: JsonValue,
}

impl BooleanStrategy {
    pub fn new() -> Self {
        BooleanStrategy {
            extra_keywords: json::object! {}
        }
    }
}

impl TypelessSchemaStrategy for BooleanStrategy {
    fn js_type(&self) -> &'static str {
        "boolean"
    }

    fn rs_type(&self) -> ScalarType {
        ScalarType::Boolean(false)
    }
}

impl SchemaStrategy for BooleanStrategy {
    fn get_extra_keywords_mut(&mut self) -> &mut JsonValue {
        &mut self.extra_keywords
    }

    fn get_extra_keywords(&self) -> &JsonValue {
        &self.extra_keywords
    }

    fn match_schema(&self, schema: JsonValue) -> bool {
        TypelessSchemaStrategy::match_schema(self, schema)
    }

    fn match_object(&self, object: JsonValue) -> bool {
        TypelessSchemaStrategy::match_object(self, object)
    }
}


pub struct StringStrategy {
    extra_keywords: JsonValue,
}

impl StringStrategy {
    pub fn new() -> Self {
        StringStrategy {
            extra_keywords: json::object! {}
        }
    }
}

impl SchemaStrategy for StringStrategy {
    fn get_extra_keywords_mut(&mut self) -> &mut JsonValue {
        &mut self.extra_keywords
    }

    fn get_extra_keywords(&self) -> &JsonValue {
        &self.extra_keywords
    }

    fn match_schema(&self, schema: JsonValue) -> bool {
        TypelessSchemaStrategy::match_schema(self, schema)
    }

    fn match_object(&self, object: JsonValue) -> bool {
        TypelessSchemaStrategy::match_object(self, object)
    }
}

impl TypelessSchemaStrategy for StringStrategy {
    fn js_type(&self) -> &'static str {
        "string"
    }

    fn rs_type(&self) -> ScalarType {
        ScalarType::String("")
    }
}

pub struct NumberStrategy {
    number_type: &'static str,
    extra_keywords: JsonValue,
}

impl NumberStrategy {
    pub fn new() -> Self {
        NumberStrategy {
            number_type: "integer",
            extra_keywords: json::object! {},
        }
    }
}

impl TypelessSchemaStrategy for NumberStrategy {
    fn js_type(&self) -> &'static str {
        "integer|number"
    }

    fn rs_type(&self) -> ScalarType {
        ScalarType::Number(0)
    }

    fn to_schema(&self) -> JsonValue {
        let mut schema = SchemaStrategy::to_schema(self);
        schema["type"] = JsonValue::String(self.number_type.to_string());
        schema
    
    }
}

impl SchemaStrategy for NumberStrategy {
    fn add_schema(&mut self, schema: JsonValue) {
        SchemaStrategy::add_schema(self, schema);
        if schema["type"] == "number" {
            self.number_type = "number";
        }
    }

    fn add_object(&mut self, object: JsonValue) {
        if object.is_number() {
            if let Some(_) = object.as_i64() {
                self.number_type = "number";
            }
        }
    }

    fn get_extra_keywords_mut(&mut self) -> &mut JsonValue {
        &mut self.extra_keywords
    }

    fn get_extra_keywords(&self) -> &JsonValue {
        &self.extra_keywords
    }

    fn match_schema(&self, schema: JsonValue) -> bool {
        TypelessSchemaStrategy::match_schema(self, schema)
    }

    fn match_object(&self, object: JsonValue) -> bool {
        TypelessSchemaStrategy::match_object(self, object)
    }
}

pub struct TypelessStrategy {
    extra_keywords: JsonValue,
}

impl TypelessStrategy {
    pub fn new() -> Self {
        TypelessStrategy {
            extra_keywords: json::object! {}
        }
    }
}

impl SchemaStrategy for TypelessStrategy {
    fn get_extra_keywords_mut(&mut self) -> &mut JsonValue {
        &mut self.extra_keywords
    }

    fn get_extra_keywords(&self) -> &JsonValue {
        &self.extra_keywords
    }

    fn match_schema(&self, schema: JsonValue) -> bool {
        !schema.has_key("type")
    }

    fn match_object(&self, object: JsonValue) -> bool {
        false
    }
}