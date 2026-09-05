use crate::loader::LoadedSource;
use crate::parser::document::Document;
use crate::parser::error::ParserError;
use crate::parser::json::JsonDocumentParser;
use crate::source::SourceFormat;

pub mod document;
pub mod error;

mod json;

trait Parser {
    fn parse(bytes: &[u8]) -> Result<Document, ParserError>;
}

pub struct DocumentParser;

impl DocumentParser {
    pub fn parse(source: &LoadedSource) -> Result<Document, ParserError> {
        match source.format() {
            SourceFormat::Json => JsonDocumentParser::parse(source.bytes()),
            SourceFormat::Yaml => todo!(),
            SourceFormat::Xml => todo!(),
            SourceFormat::Text => todo!(),
        }
    }
}
