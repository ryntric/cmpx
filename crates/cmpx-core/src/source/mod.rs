pub mod file;
pub mod http;

use file::*;
use http::*;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct SourceDefinition {
    name: String,
    format: SourceFormat,
    #[serde(flatten)]
    kind: SourceKind,
}

impl SourceDefinition {
    pub fn http(name: impl Into<String>, format: SourceFormat, source: HttpSource) -> Self {
        Self {
            name: name.into(),
            format,
            kind: SourceKind::Http(source),
        }
    }

    pub fn file(name: impl Into<String>, format: SourceFormat, source: FileSource) -> Self {
        Self {
            name: name.into(),
            format,
            kind: SourceKind::File(source),
        }
    }

    pub fn format(&self) -> SourceFormat {
        self.format
    }

    pub fn name(&self) -> &str {
        &self.name
    }

    pub(crate) fn kind(&self) -> &SourceKind {
        &self.kind
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type", content = "config", rename_all = "snake_case")]
pub(crate) enum SourceKind {
    Http(HttpSource),
    File(FileSource),
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum SourceFormat {
    Json,
    Yaml,
    Xml,
    Text,
}

#[derive(Debug, Clone)]
pub struct SourceData {
    data: Vec<u8>,
    metadata: SourceMetadata,
}

impl SourceData {
    pub fn new(data: Vec<u8>, metadata: SourceMetadata) -> Self {
        Self { data, metadata }
    }

    pub fn data(&self) -> &[u8] {
        &self.data
    }

    pub fn metadata(&self) -> &SourceMetadata {
        &self.metadata
    }
}

#[derive(Debug, Clone, Copy)]
pub struct SourceMetadata {
    format: SourceFormat,
}

impl SourceMetadata {
    pub fn new(format: SourceFormat) -> Self {
        Self { format }
    }

    pub fn format(&self) -> SourceFormat {
        self.format
    }
}
