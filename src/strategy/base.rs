use serde_json::Value;

/// base schema strategy trait
pub trait SchemaStrategy {

    fn match_schema(schema: &Value) -> bool;
    fn match_object(object: &Value) -> bool;

    fn add_schema(&mut self, schema: &Value) {
        self.add_extra_keywords(schema)
    }

    fn add_object(&mut self, _object: &Value);

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
                        } else if !keywords.contains_key(key) {
                            // add the property from the input schema if it doesn't already exist
                            keywords.insert(key.to_string(), value.clone());
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


/// base schema strategy trait for scalar types
pub trait ScalarSchemaStrategy: SchemaStrategy {
    fn js_type() -> &'static str;

    fn to_schema(&self) -> Value {
        let mut schema = SchemaStrategy::to_schema(self);
        schema["type"] = Value::String(Self::js_type().to_string());
        schema
    }
}
