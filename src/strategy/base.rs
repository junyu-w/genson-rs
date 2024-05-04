use serde_json::Value;

use crate::strategy::{
    array::ListStrategy, object::ObjectStrategy, 
    scalar::{BooleanStrategy, NullStrategy, NumberStrategy, StringStrategy, TypelessStrategy}
};


/// base schema strategy trait
pub trait SchemaStrategy {

    fn match_schema(schema: &Value) -> bool;
    fn match_object(object: &Value) -> bool;

    fn add_schema(&mut self, schema: &Value) {
        self.add_extra_keywords(schema)
    }

    fn add_object(&mut self, _object: &Value) {
        ()
    }

    fn to_schema(&self) -> Value {
        self.get_extra_keywords().clone()
    }

    fn add_extra_keywords(&mut self, schema: &Value) {
        if let Value::Object(schema) = schema {
            schema.iter().for_each(|(key, value)| {
                let keywords = self.get_extra_keywords_mut();
                match keywords {
                    Value::Object(keywords) => {
                        if key == "type" {
                            return;
            
                        } else if let None = keywords.get(key) {
                            keywords.insert(key.to_string(), value.clone());
            
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
    }

    fn get_extra_keywords_mut(&mut self) -> &mut Value;

    fn get_extra_keywords(&self) -> &Value;
}


pub enum ScalarType {
    Null,
    String,
    Number,
    Boolean,
}

/// base schema strategy trait for scalar types
pub trait TypelessSchemaStrategy: SchemaStrategy {
    fn js_type() -> &'static str;
    fn rs_type() -> ScalarType;
    fn to_schema(&self) -> Value {
        let mut schema = SchemaStrategy::to_schema(self);
        schema["type"] = Value::String(Self::js_type().to_string());
        schema
    }

    fn match_schema(schema: &Value) -> bool {
        Self::js_type().split("|").any(|t| schema["type"] == t)
    }

    fn match_object(object: &Value) -> bool {
        match Self::rs_type() {
            ScalarType::Null => object.is_null(),
            ScalarType::String => object.is_string(),
            ScalarType::Number => object.is_number(),
            ScalarType::Boolean => object.is_boolean(),
        }
    }
}
