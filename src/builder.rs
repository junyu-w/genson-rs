use serde_json::{Value, json};

use crate::node::{DataType, SchemaNode};

const DEFAULT_SCHEMA_URI: &str = "http://json-schema.org/schema#";
const NULL_SCHEMA_URI: &str = "NULL";


pub struct SchemaBuilder {
    schema_uri: Option<String>,
    root_node: SchemaNode,
}

impl SchemaBuilder {
    /// Create a new SchemaBuilder object. The schema_uri parameter is optional, a value
    /// of "AUTO" will automatically detect the schema URI based on the input schema, if no
    /// schema URI was detected, a default URI of "http://json-schema.org/schema#" will be used. 
    /// A value of None will leave out the "$schema" keyword in the output schema.
    pub fn new(schema_uri: Option<&str>) -> Self {
        // TODO: the functionality to allow non-default node class with extended 
        //  strategies is not supported yet
        let root_node: SchemaNode = SchemaNode::new();

        if let Some(uri) = schema_uri {
            if uri == "AUTO" {
                SchemaBuilder { schema_uri: None, root_node }
            } else {
                SchemaBuilder { schema_uri: Some(uri.to_string()), root_node }
            }
        } else {
            SchemaBuilder { schema_uri: Some(NULL_SCHEMA_URI.to_string()), root_node }
        }
    }

    /// Merge in raw JSON schema object
    pub fn add_schema(&mut self, mut schema: Value) {
        // TODO: support passing in another schema builder object
        if let Value::Object(ref mut schema_obj) = schema {
            if schema_obj.contains_key("$schema") && self.schema_uri.is_none() {
                self.schema_uri = Some(schema_obj["$schema"].to_string());
                schema_obj.remove("$schema");
            }
            self.root_node.add_schema(DataType::Schema(&schema));
        }
        panic!("Invalid schema type - must be a valid JSON object")
    }

    /// Merge in another SchemaNode object
    pub fn add_schema_node(&mut self, node: SchemaNode) {
        self.root_node.add_schema(DataType::SchemaNode(&node));
    }

    /// Modify the schema to accomodate the input object
    pub fn add_object(&mut self, object: Value) {
        self.root_node.add_object(DataType::Object(&object));
    }

    /// Export the currently constructed schema as a JSON object
    pub fn to_schema(&self) -> Value {
        let mut base_schema = self.get_base_schema();
        
        let base_schema_map = base_schema.as_object_mut().unwrap();
        let node_schema = self.root_node.to_schema();
        let node_schema_map = node_schema.as_object().unwrap();
        
        for (key, value) in node_schema_map.iter() {
            base_schema_map.insert(key.to_string(), value.clone());
        }
        return base_schema;
    }

    fn get_base_schema(&self) -> Value {
        if let Some(uri) = &self.schema_uri {
            if uri == NULL_SCHEMA_URI {
                return json!({});
            } else {
                return json!({"$schema": uri});
            }
        } else {
            return json!({"$schema": DEFAULT_SCHEMA_URI});
        }
    }

    /// Serialize the currently constructed schema to a JSON string
    pub fn to_json(&self) -> String {
        let schema = self.to_schema();
        return schema.to_string();
    }


}
