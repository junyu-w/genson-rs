use json::JsonValue;
use crate::strategy::base;

pub struct SchemaNode {
    active_strategies: Vec<Box<dyn base::SchemaStrategy>>
}

impl SchemaNode {
    pub fn new() -> Self {
        SchemaNode {
            active_strategies: vec![]
        }
    }

    pub fn add_schema(&mut self, schema: SchemaNode) -> &mut Self {
        self
    }

    pub fn add_object(&mut self, object: JsonValue) -> &mut Self {
        let strategy = self.get_strategy_for_object(object);
        self.active_strategies.push(Box::new(strategy));
        self
    }

    pub fn to_schema(self) -> JsonValue {
        JsonValue::Null
    }

    fn get_strategy_for_object(self, object: JsonValue) -> impl base::SchemaStrategy {
        base::TypelessStrategy::new()
    }
}

