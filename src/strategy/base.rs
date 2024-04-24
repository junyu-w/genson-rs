use json::JsonValue;


pub trait SchemaStrategy {

    // fn new(node_class: node::SchemaNode) -> Self;
    // fn match_schema(schema: JsonValue) -> bool;
    // fn match_object(object: JsonValue) -> bool;
    fn add_schema(&mut self, schema: JsonValue) {
        self.add_extra_keywords(schema)
    }
    fn add_object(&mut self, object: JsonValue);

    fn to_schema(&self) -> JsonValue;

    fn add_extra_keywords(&mut self, schema: JsonValue) {
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
}

pub struct TypelessStrategy {
    extra_keywords: JsonValue
}

impl SchemaStrategy for TypelessStrategy {
    fn add_schema(&mut self, schema: JsonValue) {
    }

    fn add_object(&mut self, object: JsonValue) {
    }

    fn to_schema(&self) -> JsonValue {
        self.extra_keywords.clone()
    }

    fn get_extra_keywords_mut(&mut self) -> &mut JsonValue {
        &mut self.extra_keywords
    }
}

impl TypelessStrategy {
    pub fn new() -> Self {
        TypelessStrategy {
            extra_keywords: json::object! {}
        }
    }
}
