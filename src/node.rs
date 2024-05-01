use serde_json::{Value, json};
use crate::strategy::base::{SchemaStrategy, BasicSchemaStrategy};

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
            let active_strategy = self.get_strategy_for_schema(&subschema);
            SchemaNode::add_schema_or_object(active_strategy, DataType::Schema(&subschema));
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

        let active_strategy = self.get_strategy_for_object(&object);
        SchemaNode::add_schema_or_object(active_strategy, DataType::Object(object));
        self
    }

    pub fn to_schema(&self) -> Value {
        // TODO: do I need this here?
        unimplemented!()
    }

    fn get_strategy_for_object(&mut self, object: &Value) -> &mut BasicSchemaStrategy {
        return self.get_strategy_for_kind(DataType::Object(object));
    }

    fn get_strategy_for_schema(&mut self, schema: &Value) -> &mut BasicSchemaStrategy {
        return self.get_strategy_for_kind(DataType::Schema(schema));
    }

    fn get_strategy_for_kind(&mut self, schema_or_object_ref: DataType) -> &mut BasicSchemaStrategy {
        let active_strategy = self.active_strategies.iter_mut().find(|strategy| { 
            SchemaNode::match_schema_or_object(strategy, &schema_or_object_ref)
        });
        if let Some(s) = active_strategy {
            return s;
        }

        unimplemented!()
    }

    fn match_schema_or_object(strategy: &BasicSchemaStrategy, schema_or_object: &DataType) -> bool {
        match schema_or_object {
            DataType::Object(obj) => match strategy {
                BasicSchemaStrategy::Object(s) => s.match_object(obj),
                BasicSchemaStrategy::Boolean(s) => s.match_object(obj),
                BasicSchemaStrategy::Null(s) => s.match_object(obj),
                BasicSchemaStrategy::Number(s) => s.match_object(obj),
                BasicSchemaStrategy::String(s) => s.match_object(obj),
                BasicSchemaStrategy::List(s) => s.match_object(obj),
                _ => false
            },
            DataType::Schema(schema) => match strategy {
                BasicSchemaStrategy::Object(s) => s.match_schema(schema),
                BasicSchemaStrategy::Boolean(s) => s.match_schema(schema),
                BasicSchemaStrategy::Null(s) => s.match_schema(schema),
                BasicSchemaStrategy::Number(s) => s.match_schema(schema),
                BasicSchemaStrategy::String(s) => s.match_schema(schema),
                BasicSchemaStrategy::List(s) => s.match_schema(schema),
                _ => false
            },
            _ => false
        }
    }

    fn add_schema_or_object(strategy: &mut BasicSchemaStrategy, schema_or_object: DataType) {
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

