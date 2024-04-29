use json::JsonValue;

use crate::strategy::{
    array::ListStrategy, object::ObjectStrategy, 
    scalar::{BooleanStrategy, NullStrategy, NumberStrategy, StringStrategy}
};

pub enum BasicSchemaStrategy {
    Object(ObjectStrategy),
    List(ListStrategy),
    Null(NullStrategy),
    Boolean(BooleanStrategy),
    Number(NumberStrategy),
    String(StringStrategy),
}

/// base schema strategy trait
pub trait SchemaStrategy {

    fn match_schema(&self, schema: &JsonValue) -> bool;
    fn match_object(&self, object: &JsonValue) -> bool;

    fn add_schema(&mut self, schema: &JsonValue) {
        self.add_extra_keywords(schema)
    }

    fn add_object(&mut self, object: &JsonValue) {
        ()
    }

    fn to_schema(&self) -> JsonValue {
        self.get_extra_keywords().clone()
    }

    fn add_extra_keywords(&mut self, schema: &JsonValue) {
        schema.entries().for_each(|(key, value)| {
            let keywords = self.get_extra_keywords_mut();
            match keywords {
                JsonValue::Object(keywords) => {
                    if key == "type" {
                        return;
        
                    } else if let None = keywords.get(key) {
                        keywords.insert(key, value.clone());
        
                    } else if let Some(current_value) = keywords.get(key) {
                        if current_value != value {
                            eprintln!("Warning: Schema incompatible. Keyword {key} has conflicting \
                                values {current_value} and {value}. Using {current_value}.")
                        }
                    }
                },
                _ => ()
            }
        });
    }

    fn get_extra_keywords_mut(&mut self) -> &mut JsonValue;

    fn get_extra_keywords(&self) -> &JsonValue;
}


pub enum ScalarType {
    Null,
    String,
    Number,
    Boolean,
}

/// base schema strategy trait for scalar types
pub trait TypelessSchemaStrategy: SchemaStrategy {
    fn js_type(&self) -> &'static str;
    fn rs_type(&self) -> ScalarType;
    fn to_schema(&self) -> JsonValue {
        let mut schema = SchemaStrategy::to_schema(self);
        schema["type"] = JsonValue::String(self.js_type().to_string());
        schema
    }

    fn match_schema(&self, schema: &JsonValue) -> bool {
        self.js_type().split("|").any(|t| schema["type"] == t)
    }

    fn match_object(&self, object: &JsonValue) -> bool {
        match self.rs_type() {
            ScalarType::Null => object.is_null(),
            ScalarType::String => object.is_string(),
            ScalarType::Number => object.is_number(),
            ScalarType::Boolean => object.is_boolean(),
        }
    }
}
