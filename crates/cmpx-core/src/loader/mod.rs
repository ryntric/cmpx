use crate::source::{SourceData, SourceDefinition, SourceKind, SourceMetadata};

pub use error::SourceLoadError;

mod error;
mod file;
mod http;

pub(crate) trait Loadable {
    async fn load(&self) -> Result<Vec<u8>, SourceLoadError>;
}

pub struct Loader {}



pub async fn load(definition: &SourceDefinition) -> Result<SourceData, SourceLoadError> {
    let data = match definition.kind() {
        SourceKind::File(source) => source.load().await?,
        SourceKind::Http(source) => source.load().await?,
    };
    let metadata = SourceMetadata::new(definition.format());
    Ok(SourceData::new(data, metadata))
}
