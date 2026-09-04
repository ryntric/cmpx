use std::collections::HashMap;

#[derive(Debug, PartialEq, Clone)]
pub struct Document {
    root: Node,
}

impl Document {
    pub fn root(&self) -> &Node {
        &self.root
    }
}

#[derive(Debug, PartialEq, Clone)]
pub enum Node {
    Null,
    Bool(bool),
    Number(Number),
    String(String),
    Array(Vec<Node>),
    Object(HashMap<String, Node>),
}

#[derive(Debug, PartialEq, Clone)]
pub enum Number {
    Float(f64),
    Unsigned(u64),
    Signed(i64),
}

impl Document {
    pub fn new(node: Node) -> Self {
        Self { root: node }
    }
}
