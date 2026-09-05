use crate::source::{Source, SourceConfig, SourceFormat, SourceId};

use crate::loader::file::FileLoader;
use crate::loader::http::{HttpLoader, HttpMetadata};
pub use error::LoadSourceError;

pub mod error;

mod file;
mod http;

trait Loader<T> {
    async fn load(&self, source: &Source, config: &T) -> Result<LoadedSource, LoadSourceError>;
}

pub struct SourceLoader {
    file_loader: FileLoader,
    http_loader: HttpLoader,
}

impl SourceLoader {
    pub fn new() -> SourceLoader {
        Self {
            file_loader: FileLoader::new(),
            http_loader: HttpLoader::new(),
        }
    }

    pub async fn load(&self, source: &Source) -> Result<LoadedSource, LoadSourceError> {
        match source.config() {
            SourceConfig::Http(config) => self.http_loader.load(source, config).await,
            SourceConfig::File(config) => self.file_loader.load(source, config).await,
        }
    }
}

#[derive(Debug)]
pub struct LoadedSource {
    id: SourceId,
    name: String,
    bytes: Vec<u8>,
    format: SourceFormat,
    metadata: Option<LoadMetadata>,
}

impl LoadedSource {
    pub(crate) fn new(
        source: &Source,
        bytes: Vec<u8>,
        metadata: Option<LoadMetadata>,
    ) -> LoadedSource {
        LoadedSource {
            id: source.id(),
            name: source.name().to_owned(),
            bytes,
            format: source.format(),
            metadata,
        }
    }

    pub fn id(&self) -> SourceId {
        self.id
    }

    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn bytes(&self) -> &Vec<u8> {
        &self.bytes
    }

    pub fn format(&self) -> SourceFormat {
        self.format
    }

    pub fn metadata(&self) -> &Option<LoadMetadata> {
        &self.metadata
    }
}

#[derive(Debug)]
pub enum LoadMetadata {
    Http(HttpMetadata),
}
