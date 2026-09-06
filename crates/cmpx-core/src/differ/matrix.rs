use crate::differ::DiffSource;
use crate::parser::document::{DocumentPath, Node};

#[derive(Debug)]
pub struct DiffCell<'a> {
    path: DocumentPath,
    value: &'a Node,
}

impl<'a> DiffCell<'a> {
    pub fn new(path: DocumentPath, value: &'a Node) -> DiffCell<'a> {
        Self { path, value }
    }
}

#[derive(Debug, Default)]
pub struct DiffColumn<'a> {
    cells: Vec<DiffCell<'a>>,
}

impl<'a> DiffColumn<'a> {
    pub fn push(&mut self, cell: DiffCell<'a>) {
        self.cells.push(cell);
    }
}

#[derive(Debug)]
pub struct DiffRow<'a> {
    columns: Vec<DiffCell<'a>>,
}

impl<'a> From<&'a DiffSource> for DiffColumn<'a> {
    fn from(source: &'a DiffSource) -> Self {
        let mut cells = Vec::new();
        source.document().traverse(&mut |node, path| {
            cells.push(DiffCell::new(path, node));
        });

        Self { cells }
    }
}
