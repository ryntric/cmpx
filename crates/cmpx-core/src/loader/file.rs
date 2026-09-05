use super::Loader;

use super::LoadedSource;
use crate::loader::error::LoadSourceError;
use crate::source::Source;
use crate::source::file::FileConfig;

pub(crate) struct FileLoader;

impl FileLoader {
    pub fn new() -> FileLoader {
        Self
    }
}

impl Loader<FileConfig> for FileLoader {
    async fn load(
        &self,
        source: &Source,
        config: &FileConfig,
    ) -> Result<LoadedSource, LoadSourceError> {
        let bytes = tokio::fs::read(config.path()).await?;
        Ok(LoadedSource::new(source, bytes, None))
    }
}
