pub mod base;
pub mod scalar;
pub mod array;
pub mod object;

use serde_json::Value;

use array::ListStrategy;
use object::ObjectStrategy;
use scalar::{BooleanStrategy, NullStrategy, NumberStrategy, StringStrategy, TypelessStrategy};
use base::{SchemaStrategy, TypelessSchemaStrategy};

use self::array::ListSchemaStrategy;

#[derive(Debug)]
pub enum BasicSchemaStrategy {
    Object(ObjectStrategy),
    List(ListStrategy),
    Null(NullStrategy),
    Boolean(BooleanStrategy),
    Number(NumberStrategy),
    String(StringStrategy),
    Typeless(TypelessStrategy),
}

// TODO: the match check is repeated everywhere, maybe we can optimize this with a macro!

impl BasicSchemaStrategy {

    pub fn new_for_object(object: &Value) -> Option<Self> {
        if ObjectStrategy::match_object(object) {
            Some(BasicSchemaStrategy::Object(ObjectStrategy::new()))
        } else if <ListStrategy as ListSchemaStrategy>::match_object(object) {
            Some(BasicSchemaStrategy::List(ListStrategy::new()))
        } else if <NullStrategy as SchemaStrategy>::match_object(object) {
            Some(BasicSchemaStrategy::Null(NullStrategy::new()))
        } else if <BooleanStrategy as SchemaStrategy>::match_object(object) {
            Some(BasicSchemaStrategy::Boolean(BooleanStrategy::new()))
        } else if <NumberStrategy as SchemaStrategy>::match_object(object) {
            Some(BasicSchemaStrategy::Number(NumberStrategy::new()))
        } else if <StringStrategy as SchemaStrategy>::match_object(object) {
            Some(BasicSchemaStrategy::String(StringStrategy::new()))
        } else {
            None
        }
    }

    pub fn new_for_schema(schema: &Value) -> Option<Self> {
        if ObjectStrategy::match_schema(schema) {
            Some(BasicSchemaStrategy::Object(ObjectStrategy::new()))
        } else if ListStrategy::match_schema(schema) {
            Some(BasicSchemaStrategy::List(ListStrategy::new()))
        } else if <NullStrategy as SchemaStrategy>::match_schema(schema) {
            Some(BasicSchemaStrategy::Null(NullStrategy::new()))
        } else if <BooleanStrategy as SchemaStrategy>::match_schema(schema) {
            Some(BasicSchemaStrategy::Boolean(BooleanStrategy::new()))
        } else if <NumberStrategy as SchemaStrategy>::match_schema(schema) {
            Some(BasicSchemaStrategy::Number(NumberStrategy::new()))
        } else if <StringStrategy as SchemaStrategy>::match_schema(schema) {
            Some(BasicSchemaStrategy::String(StringStrategy::new()))
        } else {
            None
        }
    }

    pub fn to_schema(&self) -> Value {
        match self {
            BasicSchemaStrategy::Object(strategy) => strategy.to_schema(),
            BasicSchemaStrategy::List(strategy) => ListSchemaStrategy::to_schema(strategy),
            BasicSchemaStrategy::Null(strategy) => TypelessSchemaStrategy::to_schema(strategy),
            BasicSchemaStrategy::Boolean(strategy) => TypelessSchemaStrategy::to_schema(strategy),
            BasicSchemaStrategy::Number(strategy) => TypelessSchemaStrategy::to_schema(strategy),
            BasicSchemaStrategy::String(strategy) => TypelessSchemaStrategy::to_schema(strategy),
            BasicSchemaStrategy::Typeless(strategy) => strategy.to_schema(),
        }
    }

    pub fn match_object(&self, object: &Value) -> bool {
        match self {
            BasicSchemaStrategy::Object(_) => ObjectStrategy::match_object(object),
            BasicSchemaStrategy::List(_) => <ListStrategy as ListSchemaStrategy>::match_object(object),
            BasicSchemaStrategy::Null(_) => <NullStrategy as SchemaStrategy>::match_object(object),
            BasicSchemaStrategy::Boolean(_) => <BooleanStrategy as SchemaStrategy>::match_object(object),
            BasicSchemaStrategy::Number(_) => <NumberStrategy as SchemaStrategy>::match_object(object),
            BasicSchemaStrategy::String(_) => <StringStrategy as SchemaStrategy>::match_object(object),
            BasicSchemaStrategy::Typeless(_) => TypelessStrategy::match_object(object),
        }
    }

    pub fn match_schema(&self, schema: &Value) -> bool {
        match self {
            BasicSchemaStrategy::Object(_) => ObjectStrategy::match_schema(schema),
            BasicSchemaStrategy::List(_) => ListStrategy::match_schema(schema),
            BasicSchemaStrategy::Null(_) => <NullStrategy as SchemaStrategy>::match_schema(schema),
            BasicSchemaStrategy::Boolean(_) => <BooleanStrategy as SchemaStrategy>::match_schema(schema),
            BasicSchemaStrategy::Number(_) => <NumberStrategy as SchemaStrategy>::match_schema(schema),
            BasicSchemaStrategy::String(_) => <StringStrategy as SchemaStrategy>::match_schema(schema),
            BasicSchemaStrategy::Typeless(_) => TypelessStrategy::match_schema(schema),
        }
    }

    pub fn add_schema(&mut self, schema: &Value) {
        match self {
            BasicSchemaStrategy::Object(strategy) => strategy.add_schema(schema),
            BasicSchemaStrategy::List(strategy) => strategy.add_schema(schema),
            BasicSchemaStrategy::Null(strategy) => strategy.add_schema(schema),
            BasicSchemaStrategy::Boolean(strategy) => strategy.add_schema(schema),
            BasicSchemaStrategy::Number(strategy) => strategy.add_schema(schema),
            BasicSchemaStrategy::String(strategy) => strategy.add_schema(schema),
            BasicSchemaStrategy::Typeless(strategy) => strategy.add_schema(schema),
        }
    }

    pub fn add_object(&mut self, object: &Value) {
        match self {
            BasicSchemaStrategy::Object(strategy) => strategy.add_object(object),
            BasicSchemaStrategy::List(strategy) => strategy.add_object(object),
            BasicSchemaStrategy::Null(strategy) => strategy.add_object(object),
            BasicSchemaStrategy::Boolean(strategy) => strategy.add_object(object),
            BasicSchemaStrategy::Number(strategy) => strategy.add_object(object),
            BasicSchemaStrategy::String(strategy) => strategy.add_object(object),
            BasicSchemaStrategy::Typeless(strategy) => strategy.add_object(object),
        }
    }
}