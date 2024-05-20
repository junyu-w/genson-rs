use std::collections::HashMap;
use std::collections::hash_set::HashSet;
use regex::Regex;

use serde_json::{Value, json, Map};
use simd_json;
use simd_json::prelude::TypedContainerValue;

use crate::node::{SchemaNode, DataType};
use crate::strategy::base::SchemaStrategy;

#[derive(Debug)]
pub struct ObjectStrategy {
    // TODO: this is redeclared everywhere, how to avoid this?
    extra_keywords: Value,
    properties: HashMap<String, SchemaNode>,
    pattern_properties: HashMap<String, SchemaNode>,
    required_properties: Option<HashSet<String>>,
    include_empty_required: bool,
}

impl ObjectStrategy {
    pub fn new() -> Self {
        ObjectStrategy {
            extra_keywords: json!({}),
            properties: HashMap::new(),
            pattern_properties: HashMap::new(),
            required_properties: None,
            include_empty_required: false,
        }
    }
}

impl SchemaStrategy for ObjectStrategy {
    fn get_extra_keywords_mut(&mut self) -> &mut Value {
        &mut self.extra_keywords
    }

    fn get_extra_keywords(&self) -> &Value {
        &self.extra_keywords
    }

    fn match_schema(schema: &Value) -> bool {
        schema["type"] == "object"
    }

    fn match_object(object: &simd_json::BorrowedValue) -> bool {
        object.is_object()
    }

    fn add_object(&mut self, object: &simd_json::BorrowedValue) {
        let mut properties = HashSet::new();
        if let simd_json::BorrowedValue::Object(object) = object {
            object.iter().for_each(|(prop, subobj)| {
                let mut pattern: Option<&str> = None;
                if !self.properties.contains_key(prop.as_ref()) {
                    let pattern_matcher = |p: &str| Regex::new(p).unwrap().is_match(prop);
                    self.pattern_properties
                        .iter_mut()
                        .find(|(p, _)| pattern_matcher(p))
                        .map(|(p, node)| {
                            pattern = Some(p);
                            node.add_object(DataType::Object(subobj));
                        });
                }

                if pattern.is_none() {
                    properties.insert(prop.to_string());
                    if !self.properties.contains_key(prop.as_ref()) {
                        self.properties.insert(prop.to_string(), SchemaNode::new());
                    }
                    self.properties.get_mut(prop.as_ref()).unwrap().add_object(DataType::Object(subobj));
                }
            });
        }

        if self.required_properties.is_none() {
            self.required_properties = Some(properties);
        } else {
            // take the intersection
            self.required_properties.as_mut().unwrap().retain(|p| properties.contains(p));
        }
    }

    fn add_schema(&mut self, schema: &Value) {
        if let Value::Object(schema_object) = schema {
            self.add_extra_keywords(schema);

            // properties updater updates the internal properties and pattern_properties with the schema_object,
            // creating schema node as needed for each property
            let properties_updater = 
                    |properties: &mut HashMap<String, SchemaNode>, schema_object: &Map<String, Value>, prop_key: &str| {
                if let Some(schema_properties) = schema_object[prop_key].as_object() {
                    schema_properties.iter().for_each(|(prop, sub_schema)| {
                        let sub_node = properties.entry(prop.to_string())
                            .or_insert(SchemaNode::new());
                        sub_node.add_schema(DataType::Schema(sub_schema));
                    });
                }
            };

            if schema_object.contains_key("properties") {
                properties_updater(&mut self.properties, schema_object, "properties");
            }
            if schema_object.contains_key("patternProperties") {
                properties_updater(&mut self.pattern_properties, schema_object, "patternProperties");
            }
            if schema_object.contains_key("required") {
                if let Value::Array(required_fields) = &schema_object["required"] {
                    if required_fields.len() == 0 {
                        // if the input schema object has required fields being empty, that means 
                        // including empty required fields in the schema is the desired behavior
                        // and should be followed
                        self.include_empty_required = true;
                    }
                    if self.required_properties.is_none() {
                        let required_fields_set: HashSet<String> = required_fields.iter().map(|v| v.as_str().unwrap().to_string()).collect();
                        self.required_properties = Some(required_fields_set);
                    } else {
                        // take the intersection
                        self.required_properties.as_mut().unwrap().retain(|p| required_fields.contains(&Value::String(p.to_string())));
                    }
                }
            }
        } else {
            panic!("Invalid schema type - must be a valid JSON object")
        }
    }

    fn to_schema(&self) -> Value {
        let mut schema = self.extra_keywords.clone();
        schema["type"] = "object".into();
        if self.properties.len() > 0 {
            schema["properties"] = self.properties_to_schema(&self.properties);
        }
        if self.pattern_properties.len() > 0 {
            schema["patternProperties"] = self.properties_to_schema(&self.pattern_properties);
        }
        if self.required_properties.is_some() || self.include_empty_required {
            let mut required_props: Vec<String>;
            if let Some(required_properties) = &self.required_properties {
                required_props = required_properties.iter().map(|p| p.to_string()).collect();
            } else {
                required_props = vec![];
            }
            required_props.sort();

            if required_props.len() > 0 || self.include_empty_required {
                schema["required"] = required_props.into();
            } else {
                // this is done in case there's a conflict with the required properties
                // from extra keywords
                schema.as_object_mut().unwrap().remove("required");
            }
        } else {
            schema.as_object_mut().unwrap().remove("required");
        }
        schema
    }
}

impl ObjectStrategy {
    fn properties_to_schema(&self, properties: &HashMap<String, SchemaNode>) -> Value {
        let mut schema_properties = json!({});
        properties.iter().for_each(|(prop, node)| {
            schema_properties[prop] = node.to_schema();
        });
        schema_properties
    }
}