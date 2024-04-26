use core::slice::{IterMut, Iter};
use json::JsonValue;

use crate::strategy::base::SchemaStrategy;
use crate::node::SchemaNode;

pub trait ListSchemaStrategy: SchemaStrategy {
    fn get_items_mut(&mut self) -> IterMut<SchemaNode>;
    fn get_items(&self) -> Iter<SchemaNode>;
    fn items_to_schema(&self) -> JsonValue;

    fn to_schema(&self) -> JsonValue {
        let mut schema = SchemaStrategy::to_schema(self);
        schema["type"] = "array".into();
        if self.get_items().len() > 0 {
            schema["items"] = self.items_to_schema();
        }
        schema
    }

    fn match_object(&self, object: JsonValue) -> bool {
        object.is_array()
    }
}

/// strategy for list-style array schemas. This is the default
/// strategy for arrays.
pub struct ListStrategy {
    extra_keywords: JsonValue,
    items: Vec<SchemaNode>,
}

impl ListStrategy {
    pub fn new() -> Self {
        ListStrategy {
            extra_keywords: json::object! {},
            items: vec![SchemaNode::new()]
        }
    }
}

impl SchemaStrategy for ListStrategy {
    // TODO: this placeholder is repeated everywhere, how to avoid this?
    fn get_extra_keywords_mut(&mut self) -> &mut JsonValue {
        &mut self.extra_keywords
    }

    fn get_extra_keywords(&self) -> &JsonValue {
        &self.extra_keywords
    }

    fn match_schema(&self, schema: JsonValue) -> bool {
        schema["type"] == "array" && schema["items"].is_object()
    }

    fn match_object(&self, object: JsonValue) -> bool {
        ListSchemaStrategy::match_object(self, object)
    }

    fn add_object(&mut self, object: JsonValue) {
        let items = self.get_items_mut();
        match object {
            JsonValue::Array(objects) => {
                objects.iter().for_each(|obj| {
                    items.for_each(|node| {
                        node.add_object(obj);
                    });
                });
            },
            _ => ()
        }
    }

    fn add_schema(&mut self, schema: JsonValue) {
        SchemaStrategy::add_schema(self, schema);
        if schema.has_key("items") {
            let items = self.get_items_mut();
            items.for_each(|node| {
                node.add_schema(&schema["items"]);
            });
        }
    }
}

impl ListSchemaStrategy for ListStrategy {
    fn get_items_mut(&mut self) -> IterMut<SchemaNode> {
        self.items.iter_mut()
    }

    fn get_items(&self) -> Iter<SchemaNode> {
        self.items.iter()
    }

    fn items_to_schema(&self) -> JsonValue {
        unimplemented!()
    }
}

// TODO: implement tuple strategy