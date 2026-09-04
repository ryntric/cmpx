use crate::parser::document::{Node, Number};
use crate::parser::error::ParserError;
use crate::parser::{Document, DocumentParser};
use serde_json::{Map, Value};
use std::collections::HashMap;

pub(crate) struct JsonDocumentParser {}

impl DocumentParser for JsonDocumentParser {
    fn parse(bytes: &[u8]) -> Result<Document, ParserError> {
        let value: Value = serde_json::from_slice::<Value>(bytes)?;
        let node: Node = value.into();
        Ok(node.into())
    }
}

impl From<Value> for Node {
    fn from(value: Value) -> Node {
        match value {
            Value::Number(val) => Node::Number(val.into()),
            Value::Bool(val) => Node::Bool(val),
            Value::String(val) => Node::String(val),
            Value::Object(val) => val.into(),
            Value::Array(val) => val.into(),
            Value::Null => Node::Null,
        }
    }
}

impl From<Node> for Document {
    fn from(value: Node) -> Self {
        Document::new(value)
    }
}

impl From<Vec<Value>> for Node {
    fn from(value: Vec<Value>) -> Self {
        Node::Array(
            value
                .into_iter()
                .map(|value| value.into())
                .collect::<Vec<Node>>(),
        )
    }
}

impl From<Map<String, Value>> for Node {
    fn from(value: Map<String, Value>) -> Self {
        Node::Object(
            value
                .into_iter()
                .map(|(key, value)| (key, Node::from(value)))
                .collect::<HashMap<String, Node>>(),
        )
    }
}

impl From<serde_json::Number> for Number {
    fn from(value: serde_json::Number) -> Self {
        if let Some(number) = value.as_i64() {
            return Number::Signed(number);
        }

        if let Some(number) = value.as_u64() {
            return Number::Unsigned(number);
        }

        if let Some(number) = value.as_f64() {
            return Number::Float(number);
        }

        unreachable!("serde_json::Number must contain a valid number")
    }
}
