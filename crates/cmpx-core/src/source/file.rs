use serde::{Deserialize, Serialize};
use std::path::{Path, PathBuf};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FileConfig {
    path: PathBuf,
}

impl FileConfig {
    pub fn path(&self) -> &Path {
        &self.path
    }
}
