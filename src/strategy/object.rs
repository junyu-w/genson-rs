use std::collections::HashMap;
use std::collections::hash_set::HashSet;
use regex::Regex;

use serde_json::{Value, json};

use crate::node::{SchemaNode, DataType};
use crate::strategy::base::SchemaStrategy;

pub struct ObjectStrategy {
    // TODO: this is redeclared everywhere, how to avoid this?
    extra_keywords: Value,
    properties: HashMap<String, SchemaNode>,
    pattern_properties: HashMap<String, SchemaNode>,
    required_properties: HashSet<String>,
    include_empty_required: bool,
}

impl ObjectStrategy {
    pub fn new() -> Self {
        ObjectStrategy {
            extra_keywords: json!({}),
            properties: HashMap::new(),
            pattern_properties: HashMap::new(),
            required_properties: HashSet::new(),
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

    fn match_object(object: &Value) -> bool {
        object.is_object()
    }

    fn add_object(&mut self, object: &Value) {
        let mut properties = HashSet::new();
        if let Value::Object(object) = object {
            object.iter().for_each(|(prop, subobj)| {
                if !self.properties.contains_key(prop) {
                    let pattern_matcher = |p: &str| Regex::new(p).unwrap().is_match(prop);
                    self.pattern_properties.iter_mut().find(|(p, _)| pattern_matcher(p)).map(|(_, node)| {
                        node.add_object(DataType::Object(subobj));
                    });
                } else {
                    properties.insert(prop.to_string());
                    self.properties.get_mut(prop).unwrap().add_object(DataType::Object(subobj));
                }
            });
        }
       

        if self.required_properties.len() == 0 {
            self.required_properties.extend(properties);
        } else {
            // take the intersection
            self.required_properties.retain(|p| properties.contains(p));
        }
    }

    fn to_schema(&self) -> Value {
        let mut schema = SchemaStrategy::to_schema(self);
        schema["type"] = "object".into();
        if self.properties.len() > 0 {
            schema["propperties"] = self.properties_to_schema(&self.properties);
        }
        if self.pattern_properties.len() > 0 {
            schema["patternProperties"] = self.properties_to_schema(&self.pattern_properties);
        }
        if self.required_properties.len() > 0 && self.include_empty_required {
            let required_props: Vec<String> = self.required_properties.iter().map(|p| p.to_string()).collect();
            schema["required"] = required_props.into();
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