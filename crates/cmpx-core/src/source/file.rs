use serde::{Deserialize, Serialize};
use std::path::{Path, PathBuf};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FileSource {
    path: PathBuf,
}

impl FileSource {
    pub fn path(&self) -> &Path {
        &self.path
    }
}
