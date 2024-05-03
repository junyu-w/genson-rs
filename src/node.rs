use core::panic;

use serde_json::{Value, json};
use crate::strategy::base::{BasicSchemaStrategy, SchemaStrategy};
use crate::strategy::object::ObjectStrategy;
use crate::strategy::array::ListStrategy;
use crate::strategy::scalar::{BooleanStrategy, NullStrategy, NumberStrategy, StringStrategy, TypelessStrategy};

/// Basic schema generator class. SchemaNode objects can be loaded
/// up with existing schemas and objects before being serialized.
pub struct SchemaNode {
    active_strategies: Vec<BasicSchemaStrategy>
}

/// DataType wraps around different types of schema data that can be added
/// to a SchemaNode. It wraps references to Value objects and SchemaNode
/// objects so when it gets dropped the underlying data is not dropped.
pub enum DataType<'a> {
    Schema(&'a Value),
    Object(&'a Value),
    SchemaNode(&'a SchemaNode),
}

impl SchemaNode {
    pub fn new() -> Self {
        SchemaNode {
            active_strategies: vec![]
        }
    }

    pub fn add_schema(&mut self, data: DataType) -> &mut Self {
        let schema = match data {
            DataType::SchemaNode(node) => node.to_schema(),
            DataType::Schema(schema) => schema.clone(),
            _ => panic!("Invalid schema type")
        };
        
        for subschema in SchemaNode::get_subschemas(&schema) {
            let active_strategy = self.get_or_create_strategy_for_schema(&subschema);
            SchemaNode::add_schema_or_object_to_strategy(active_strategy, DataType::Schema(&subschema));
        }
        self
    }

    fn get_subschemas(schema: &Value) -> Vec<Value> {
        if let Value::Object(schema) = schema {
            if let Some(Value::Array(anyof)) = schema.get("anyOf") {
                return anyof.iter().map(|t| SchemaNode::get_subschemas(t)).flatten().collect();
            }
            else if let Some(Value::Array(types)) = schema.get("type") {
                return types.iter().map(|t| {
                    let mut new_schema = schema.clone();
                    new_schema["type"] = t.clone();
                    return Value::Object(new_schema);
                }).collect();
            }
            else {
                return vec![Value::Object(schema.clone())];
            }
        }
        return vec![schema.clone()];
    }

    /// Modify the schema to accomodate the object.
    pub fn add_object(&mut self, data: DataType) -> &mut Self {
        let object = match data {
            DataType::Object(obj) => obj,
            _ => panic!("Invalid object type")
        };

        let active_strategy = self.get_or_create_strategy_for_object(&object);
        SchemaNode::add_schema_or_object_to_strategy(active_strategy, DataType::Object(object));
        self
    }

    pub fn to_schema(&self) -> Value {
        // TODO: do I need this here?
        unimplemented!()
    }

    fn get_or_create_strategy_for_object(&mut self, object: &Value) -> &mut BasicSchemaStrategy {
        if let Some(idx) = self.get_strategy_for_kind(DataType::Object(object)) {
            return &mut self.active_strategies[idx];
        }
        if let Some(strategy) = self.create_strategy_for_kind(DataType::Object(object)) {
            return strategy;
        }
        panic!("Could not find matching schema type for object: {object}")
    }

    fn get_or_create_strategy_for_schema(&mut self, schema: &Value) -> &mut BasicSchemaStrategy {
        if let Some(idx) = self.get_strategy_for_kind(DataType::Schema(schema)) {
            return &mut self.active_strategies[idx];
        }
        if let Some(strategy) = self.create_strategy_for_kind(DataType::Schema(schema)) {
            return strategy;
        }
        panic!("Could not find matching schema type for schema: {schema}")
    }

    /// Get the strategy that matches the schema or object and return its index.
    fn get_strategy_for_kind(&self, schema_or_object: DataType) -> Option<usize> {
        self.active_strategies.iter().position(|strategy| {
            SchemaNode::strategy_does_match_schema_or_object(strategy, &schema_or_object)
        })
    }

    fn create_strategy_for_kind(&mut self, schema_or_object: DataType) -> Option<&mut BasicSchemaStrategy> {
        if let Some(mut strategy) = SchemaNode::create_strategy_for_schema_or_object(&schema_or_object) {
            if let Some(last_strategy) = self.active_strategies.last() {
                // if the last strategy is a typeless strategy, incorporate it into the newly created strategy
                if let BasicSchemaStrategy::Typeless(typeless) = last_strategy {
                    SchemaNode::add_schema_or_object_to_strategy(
                        &mut strategy, DataType::Schema(&typeless.to_schema())
                    );
                    self.active_strategies.pop();
                }
            }
            self.active_strategies.push(strategy);
            return Some(self.active_strategies.last_mut().unwrap());
        }

        // if no matching strategy found, create a typeless strategy and append to the active strategies
        // list if it's currently empty
        // ??: don't really understand the significance of typeless strategy yet
        else if let DataType::Schema(schema) = schema_or_object {
            if TypelessStrategy::match_schema(schema) {
                if self.active_strategies.is_empty() {
                    self.active_strategies.push(BasicSchemaStrategy::Typeless(TypelessStrategy::new()));
                }
                let first_strategy = self.active_strategies.first_mut().unwrap();
                return Some(first_strategy);
            }
        }
        return None;
    }

    fn strategy_does_match_schema_or_object(strategy: &BasicSchemaStrategy, schema_or_object: &DataType) -> bool {
        match schema_or_object {
            DataType::Object(obj) => match strategy {
                BasicSchemaStrategy::Object(_) => ObjectStrategy::match_object(obj),
                BasicSchemaStrategy::Boolean(_) => BooleanStrategy::match_object(obj),
                BasicSchemaStrategy::Null(_) => NullStrategy::match_object(obj),
                BasicSchemaStrategy::Number(_) => NumberStrategy::match_object(obj),
                BasicSchemaStrategy::String(_) => StringStrategy::match_object(obj),
                BasicSchemaStrategy::List(_) => ListStrategy::match_object(obj),
                _ => false
            },
            DataType::Schema(schema) => match strategy {
                BasicSchemaStrategy::Object(_) => ObjectStrategy::match_schema(schema),
                BasicSchemaStrategy::Boolean(_) => BooleanStrategy::match_schema(schema),
                BasicSchemaStrategy::Null(_) => NullStrategy::match_schema(schema),
                BasicSchemaStrategy::Number(_) => NumberStrategy::match_schema(schema),
                BasicSchemaStrategy::String(_) => StringStrategy::match_schema(schema),
                BasicSchemaStrategy::List(_) => ListStrategy::match_schema(schema),
                _ => false
            },
            _ => false
        }
    }

    /// Create a strategy for a schema or object based on which strategy it matches.
    /// TODO: this is a bit of a mess, but I'm not sure how to clean it up.
    fn create_strategy_for_schema_or_object(schema_or_object: &DataType) -> Option<BasicSchemaStrategy> {
        match schema_or_object {
            DataType::Object(obj) => {
                if ObjectStrategy::match_object(obj) {
                    Some(BasicSchemaStrategy::Object(ObjectStrategy::new()))
                } else if BooleanStrategy::match_object(obj) {
                    Some(BasicSchemaStrategy::Boolean(BooleanStrategy::new()))
                } else if NullStrategy::match_object(obj) {
                    Some(BasicSchemaStrategy::Null(NullStrategy::new()))
                } else if NumberStrategy::match_object(obj) {
                    Some(BasicSchemaStrategy::Number(NumberStrategy::new()))
                } else if StringStrategy::match_object(obj) {
                    Some(BasicSchemaStrategy::String(StringStrategy::new()))
                } else if ListStrategy::match_object(obj) {
                    Some(BasicSchemaStrategy::List(ListStrategy::new()))
                } else {
                    None
                }
            },
            DataType::Schema(schema) => {
                if ObjectStrategy::match_schema(schema) {
                    Some(BasicSchemaStrategy::Object(ObjectStrategy::new()))
                } else if BooleanStrategy::match_schema(schema) {
                    Some(BasicSchemaStrategy::Boolean(BooleanStrategy::new()))
                } else if NullStrategy::match_schema(schema) {
                    Some(BasicSchemaStrategy::Null(NullStrategy::new()))
                } else if NumberStrategy::match_schema(schema) {
                    Some(BasicSchemaStrategy::Number(NumberStrategy::new()))
                } else if StringStrategy::match_schema(schema) {
                    Some(BasicSchemaStrategy::String(StringStrategy::new()))
                } else if ListStrategy::match_schema(schema) {
                    Some(BasicSchemaStrategy::List(ListStrategy::new()))
                } else {
                    None
                }
            }
            _ => None
        }
    }

    fn add_schema_or_object_to_strategy(strategy: &mut BasicSchemaStrategy, schema_or_object: DataType) {
        match schema_or_object {
            DataType::Object(obj) => match strategy {
                BasicSchemaStrategy::Object(s) => s.add_object(obj),
                BasicSchemaStrategy::Boolean(s) => s.add_object(obj),
                BasicSchemaStrategy::Null(s) => s.add_object(obj),
                BasicSchemaStrategy::Number(s) => s.add_object(obj),
                BasicSchemaStrategy::String(s) => s.add_object(obj),
                BasicSchemaStrategy::List(s) => s.add_object(obj),
                _ => panic!("Invalid object type to add")
            },
            DataType::Schema(schema) => match strategy {
                BasicSchemaStrategy::Object(s) => s.add_schema(schema),
                BasicSchemaStrategy::Boolean(s) => s.add_schema(schema),
                BasicSchemaStrategy::Null(s) => s.add_schema(schema),
                BasicSchemaStrategy::Number(s) => s.add_schema(schema),
                BasicSchemaStrategy::String(s) => s.add_schema(schema),
                BasicSchemaStrategy::List(s) => s.add_schema(schema),
                _ => panic!("Invalid schema type to add")
            },
            _ => ()
        }
    }
}

