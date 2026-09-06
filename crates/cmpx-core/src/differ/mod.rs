mod matrix;

use crate::differ::matrix::DiffColumn;
use crate::parser::document::{Document, DocumentPath, Node};
use crate::source::SourceId;
use std::collections::HashMap;
use thiserror::Error;

pub struct Differ;

impl Differ {
    pub fn diff(sources: &[DiffSource]) {
        let mut matrix = DiffMatrix::new(sources.len());
        for source in sources {
            let column = DiffColumn::from(source);
            println!("{:?}", column);
            println!()
        }
        println!("{:?}", matrix);
    }

    fn collect<'a>(node: &'a Node, path: DocumentPath, id: SourceId, matrix: &mut DiffMatrix<'a>) {}
}

pub struct DiffSource {
    id: SourceId,
    document: Document,
}

impl DiffSource {
    pub fn new(id: SourceId, document: Document) -> Self {
        Self { id, document }
    }

    pub fn id(&self) -> SourceId {
        self.id
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
