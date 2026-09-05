use crate::differ::PathSegment::Root;
use crate::parser::document::{Document, Node};
use crate::source::SourceId;
use std::collections::HashMap;
use thiserror::Error;

pub struct Differ;

impl Differ {
    pub fn diff(sources: &[DiffSource]) {
        let mut matrix = DiffMatrix::new(sources.len());
        for source in sources {
            Self::collect(
                source.document.root(),
                DocumentPath::root(),
                source.id,
                &mut matrix,
            )
        }
        println!("{:?}", matrix);
    }

    fn collect<'a>(
        node: &'a Node,
        path: DocumentPath,
        id: SourceId,
        matrix: &mut DiffMatrix<'a>,
    ) {
        match node {
            Node::Object(object) => {
                for (key, value) in object {
                    Self::collect(value, path.field(key), id, matrix);
                }
            }

            Node::Array(array) => {
                for (index, value) in array.iter().enumerate() {
                    Self::collect(value, path.index(index), id, matrix);
                }
            }

            _ => matrix.set(id, path, Some(node)),
        }
    }
}

pub struct DiffSource {
    id: SourceId,
    name: String,
    document: Document,
}

impl DiffSource {
    pub fn new(id: SourceId, name: impl Into<String>, document: Document) -> Self {
        Self {
            id,
            name: name.into(),
            document,
        }
    }

    pub fn id(&self) -> SourceId {
        self.id
    }

    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn document(&self) -> &Document {
        &self.document
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
            segments: vec![Root],
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

    pub fn set(&mut self, id: SourceId, path: DocumentPath, value: Option<&'a Node>) {
        let entry = self.rows.entry(path);
        let row = entry.or_insert_with(|| DiffRow::new());
        row.set(id, value);
    }
}

#[derive(Debug)]
struct DiffRow<'a> {
    values: HashMap<SourceId, Option<&'a Node>>,
}

impl<'a> DiffRow<'a> {
    pub fn new() -> Self {
        Self {
            values: HashMap::default(),
        }
    }

    pub fn set(&mut self, id: SourceId, value: Option<&'a Node>) {
        let entry = self.values.entry(id);
        entry.insert_entry(value);
    }
}
