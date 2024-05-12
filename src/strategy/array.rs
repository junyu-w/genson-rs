use core::slice::{IterMut, Iter};
use serde_json::{Value, json};
use rayon::prelude::*;

use crate::strategy::base::SchemaStrategy;
use crate::node::{SchemaNode, DataType};

// The number of objects overwhich parallel processing is more efficient
// than serial processing. This is a heuristic value and may not always work
// for all cases.
const PARALLEL_PROCESSING_BOUNDARY: usize = 10;

pub trait ListSchemaStrategy: SchemaStrategy {
    fn get_items_mut(&mut self) -> IterMut<SchemaNode>;
    fn get_items(&self) -> Iter<SchemaNode>;
    fn items_to_schema(&self) -> Value;

    fn to_schema(&self) -> Value {
        let mut schema = SchemaStrategy::to_schema(self);
        schema["type"] = "array".into();
        if self.get_items().len() > 0 {
            schema["items"] = self.items_to_schema();
        }
        schema
    }

    fn match_object(object: &Value) -> bool {
        object.is_array()
    }
}

/// strategy for list-style array schemas. This is the default
/// strategy for arrays. List-style arrays are arrays where all
/// items are of the same type.
#[derive(Debug)]
pub struct ListStrategy {
    extra_keywords: Value,
    items: [SchemaNode; 1],
}

impl ListStrategy {
    pub fn new() -> Self {
        ListStrategy {
            extra_keywords: json!({}),
            items: [SchemaNode::new()],
        }
    }
}

impl SchemaStrategy for ListStrategy {
    // TODO: this placeholder is repeated everywhere, how to avoid this?
    fn get_extra_keywords_mut(&mut self) -> &mut Value {
        &mut self.extra_keywords
    }

    fn get_extra_keywords(&self) -> &Value {
        &self.extra_keywords
    }

    fn match_schema(schema: &Value) -> bool {
        schema["type"] == "array" && schema["items"].is_object()
    }

    fn match_object(object: &Value) -> bool {
        <Self as ListSchemaStrategy>::match_object(object)
    }

    fn add_object(&mut self, object: &Value) {
        match object {
            Value::Array(objects) => {
                let items = self.get_items_mut();
                items.for_each(|node| {
                    // if the number of objects is less than 10, it is more efficient to
                    // add them to the schema node directly without incurring the overhead 
                    // of parallel processing
                    if objects.len() < PARALLEL_PROCESSING_BOUNDARY {
                        objects.iter().for_each(|obj| {
                            node.add_object(DataType::Object(obj));
                        });
                    } else {
                        // when the number of objects are large, it is more efficient to
                        // parallelize process of objects by splitting them into partitions
                        // and processing each partition in parallel with their own schema node
                        // and then merging the results
                        let combined_node = objects.par_iter().fold(
                            || SchemaNode::new(),
                            |mut temp_node, obj| {
                                temp_node.add_object(DataType::Object(obj));
                                temp_node
                            }
                        ).reduce_with(
                            |mut first_node, next_node| {
                                first_node.add_schema(DataType::SchemaNode(&next_node));
                                first_node
                            } 
                        ).unwrap_or(SchemaNode::new());
                        node.add_schema(DataType::SchemaNode(&combined_node));
                    }
                });
            },
            _ => ()
        }
    }

    fn add_schema(&mut self, schema: &Value) {
        if let Value::Object(schema) = schema {
            if schema.contains_key("items") {
                let items = self.get_items_mut();
                items.for_each(|node| {
                    node.add_schema(DataType::Schema(&schema["items"]));
                });
            }
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

    fn items_to_schema(&self) -> Value {
        self.items[0].to_schema()
    }
}

/// strategy for tuple-style array schemas. Tuple-style arrays are arrays
/// where each item can have a different schema. The "items" keyword is an
/// array of schemas, one for each item in the tuple.
#[derive(Debug)]
pub struct TupleStrategy {
    extra_keywords: Value,
    items: Vec<SchemaNode>,
}

impl TupleStrategy {
    pub fn new() -> Self {
        TupleStrategy {
            extra_keywords: json!({}),
            items: vec![SchemaNode::new()],
        }
    }

    fn add_items<Adder>(&mut self, items: &Vec<Value>, node_adder: Adder) 
    where Adder: Fn(&mut SchemaNode, &Value)
    {
        while self.items.len() < items.len() {
            self.items.push(SchemaNode::new());
        }
        for (idx, item) in items.iter().enumerate() {
            node_adder(&mut self.items[idx], item);
        }
    }
}

impl SchemaStrategy for TupleStrategy {
    fn get_extra_keywords_mut(&mut self) -> &mut Value {
        &mut self.extra_keywords
    }

    fn get_extra_keywords(&self) -> &Value {
        &self.extra_keywords
    }

    fn match_schema(schema: &Value) -> bool {
        schema["type"] == "array" && schema["items"].is_array()
    }

    fn match_object(object: &Value) -> bool {
        <Self as ListSchemaStrategy>::match_object(object)
    }

    fn add_object(&mut self, object: &Value) {
        if let Value::Array(objects) = object {
            self.add_items(objects, |node, obj| {
                node.add_object(DataType::Object(obj));
            });
        }
    }

    fn add_schema(&mut self, schema: &Value) {
        self.add_extra_keywords(schema);
        if schema.is_object() && schema["items"].is_array() {
            let items = schema["items"].as_array().unwrap();
            self.add_items(items, |node, item| {
                node.add_schema(DataType::Schema(item));
            });
        }
    }
}

impl ListSchemaStrategy for TupleStrategy {
    fn get_items_mut(&mut self) -> IterMut<SchemaNode> {
        self.items.iter_mut()
    }

    fn get_items(&self) -> Iter<SchemaNode> {
        self.items.iter()
    }

    fn items_to_schema(&self) -> Value {
        Value::Array(
            self.items.iter()
                .map(|node| node.to_schema())
                .collect()
        )
    }
}