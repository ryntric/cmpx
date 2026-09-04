use crate::loader::error::SourceLoadError;
use crate::source::file::FileSource;

use super::Loadable;

impl Loadable for FileSource {
    async fn load(&self) -> Result<Vec<u8>, SourceLoadError> {
        let data = tokio::fs::read(self.path()).await?;
        Ok(data)
    }
}
