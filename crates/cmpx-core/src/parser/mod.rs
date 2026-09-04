use crate::parser::document::Document;
use crate::parser::error::ParserError;
use crate::parser::json::JsonDocumentParser;
use crate::source::{SourceData, SourceFormat};

pub mod document;
pub mod error;

mod json;

pub trait DocumentParser {
    fn parse(bytes: &[u8]) -> Result<Document, ParserError>;
}

pub fn parse(source_data: SourceData) -> Result<Document, ParserError> {
    match source_data.metadata().format() {
        SourceFormat::Json => JsonDocumentParser::parse(source_data.data()),
        SourceFormat::Yaml => todo!(),
        SourceFormat::Xml => todo!(),
        SourceFormat::Text => todo!(),
    }
}
