use std::collections::HashMap;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct DocumentPath {
    segments: Vec<PathSegment>,
}

impl DocumentPath {
    pub fn child(&self, segment: PathSegment) -> DocumentPath {
        let mut path = self.clone();
        path.segments.push(segment);
        path
    }

    pub fn root() -> DocumentPath {
        Self {
            segments: vec![PathSegment::Root],
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum PathSegment {
    Field(String),
    Index(usize),
    Root,
}

pub trait Traversable {
    fn traverse<'a>(&'a self, path: DocumentPath, func: &mut impl FnMut(&'a Node, DocumentPath));
}

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
impl Document {
    pub fn traverse<'a>(&'a self, func: &mut impl FnMut(&'a Node, DocumentPath)) {
        self.root.traverse(DocumentPath::root(), func);
    }
}

impl Traversable for Node {
    fn traverse<'a>(&'a self, path: DocumentPath, func: &mut impl FnMut(&'a Node, DocumentPath)) {
        match self {
            Self::String(_) | Self::Number(_) | Self::Bool(_) | Self::Null => {
                func(self, path.clone())
            }
            Self::Array(vec) => {
                if vec.is_empty() {
                    func(self, path.clone())
                } else {
                    for (index, node) in vec.iter().enumerate() {
                        node.traverse(path.child(PathSegment::Index(index)), func);
                    }
                }
            }
            Self::Object(map) => {
                if map.is_empty() {
                    func(self, path.clone())
                } else {
                    for (key, node) in map {
                        node.traverse(path.child(PathSegment::Field(key.clone())), func);
                    }
                }
            }
        }
    }
}
