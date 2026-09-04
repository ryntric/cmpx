use crate::parser::document::{Document, Node};
use std::collections::HashMap;
use thiserror::Error;

pub struct Differ;

impl Differ {
    pub fn diff(sources: &[DiffSource]) {
        let mut matrix = DiffMatrix::new(sources.len());

        for (index, source) in sources.iter().enumerate() {
            Self::collect(
                source.document.root(),
                DocumentPath::root(),
                index,
                &mut matrix,
            )
        }
        println!("{:?}", matrix);
    }

    fn collect<'a>(
        node: &'a Node,
        path: DocumentPath,
        source_index: usize,
        matrix: &mut DiffMatrix<'a>,
    ) {
        match node {
            Node::Object(object) => {
                for (key, value) in object {
                    Self::collect(value, path.field(key), source_index, matrix);
                }
            }

            Node::Array(array) => {
                for (index, value) in array.iter().enumerate() {
                    Self::collect(value, path.index(index), source_index, matrix);
                }
            }

            _ => matrix.set(source_index, path, Some(node)),
        }
    }
}

pub struct DiffSource<'a> {
    name: &'a str,
    document: Document,
}

impl<'a> DiffSource<'a> {
    pub fn new(name: &'a str, document: Document) -> Self {
        Self { name, document }
    }
}

#[derive(Debug, Error)]
pub enum DiffError {
    #[error("at least two documents are required")]
    NotEnoughDocuments,

    #[error("nodes have incompatible types")]
    IncompatibleNodes,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct DocumentPath {
    segments: Vec<PathSegment>,
}

impl DocumentPath {
    pub fn root() -> Self {
        Self {
            segments: Vec::new(),
        }
    }

    pub fn field(&self, field: impl Into<String>) -> Self {
        let mut segments = self.segments.clone();
        segments.push(PathSegment::Field(field.into()));

        Self { segments }
    }

    pub fn index(&self, index: usize) -> Self {
        let mut segments = self.segments.clone();
        segments.push(PathSegment::Index(index));

        Self { segments }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum PathSegment {
    Field(String),
    Index(usize),
    Root,
}
#[derive(Debug)]
struct DiffMatrix<'a> {
    source_count: usize,
    rows: HashMap<DocumentPath, DiffRow<'a>>,
}

impl<'a> DiffMatrix<'a> {
    pub fn new(source_count: usize) -> Self {
        Self {
            source_count,
            rows: HashMap::new(),
        }
    }

    pub fn set(&mut self, index: usize, path: DocumentPath, value: Option<&'a Node>) {
        let entry = self.rows.entry(path);
        let row = entry.or_insert_with(|| DiffRow::new(self.source_count));
        row.set(index, value);
    }
}

#[derive(Debug)]
struct DiffRow<'a> {
    values: Vec<Option<&'a Node>>,
}

impl<'a> DiffRow<'a> {
    pub fn new(size: usize) -> Self {
        Self {
            values: vec![None; size],
        }
    }

    pub fn set(&mut self, index: usize, value: Option<&'a Node>) {
        self.values[index] = value;
    }
}
