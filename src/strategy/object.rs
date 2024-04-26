use std::collections::HashMap;
use std::collections::hash_set::HashSet;

use json::JsonValue;

use crate::node::SchemaNode;
use crate::strategy::base::SchemaStrategy;

pub struct ObjectStrategy {
    // TODO: this is redeclared everywhere, how to avoid this?
    extra_keywords: JsonValue,
    properties: HashMap<String, SchemaNode>,
    pattern_properties: HashMap<String, SchemaNode>,
    required_properties: HashSet<String>,
    include_empty_required: bool,
}

impl ObjectStrategy {
    pub fn new() -> Self {
        ObjectStrategy {
            extra_keywords: json::object! {},
            properties: HashMap::new(),
            pattern_properties: HashMap::new(),
            required_properties: HashSet::new(),
            include_empty_required: false,
        }
    }
}

impl SchemaStrategy for ObjectStrategy {
    fn get_extra_keywords_mut(&mut self) -> &mut JsonValue {
        &mut self.extra_keywords
    }

    fn get_extra_keywords(&self) -> &JsonValue {
        &self.extra_keywords
    }

    fn match_schema(&self, schema: JsonValue) -> bool {
        schema["type"] == "object"
    }

    fn match_object(&self, object: JsonValue) -> bool {
        object.is_object()
    }

    fn add_object(&mut self, object: JsonValue) {
        let properties = HashSet::new();
        object.entries().for_each(|(prop, subobj)| {
            let mut pattern = None;
            if !self.properties.contains_key(prop) {
                pattern = self.matching_pattern(prop)
            }

            if let Some(pattern) = pattern {
                self.pattern_properties[pattern].add_object(subobj);
            } else {
                properties.insert(prop.to_string());
                self.properties[prop].add_object(subobj);
            }
        });

        if self.required_properties.len() == 0 {
            self.required_properties.extend(properties);
        } else {
            // take the intersection
            self.required_properties.retain(|p| self.required_properties.contains(p) && properties.contains(p));
        }
    }
}

impl ObjectStrategy {
    fn matching_pattern(&self, prop: &str) -> Option<&str> {
        unimplemented!()
    }
}