pub mod file;
pub mod http;

use crate::source::file::FileConfig;
use crate::source::http::HttpConfig;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(transparent)]
pub struct SourceId(Uuid);

impl SourceId {
    pub fn new() -> Self {
        Self(Uuid::new_v4())
    }
}

impl Default for SourceId {
    fn default() -> Self {
        Self::new()
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Source {
    id: SourceId,
    name: String,
    format: SourceFormat,
    #[serde(flatten)]
    config: SourceConfig,
}

impl Source {
    pub fn new(name: impl Into<String>, format: SourceFormat, config: SourceConfig) -> Source {
        Self {
            id: SourceId::default(),
            name: name.into(),
            format,
            config,
        }
    }

    pub fn id(&self) -> SourceId {
        self.id
    }

    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn format(&self) -> SourceFormat {
        self.format
    }

    pub fn config(&self) -> &SourceConfig {
        &self.config
    }
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum SourceConfig {
    Http(HttpConfig),
    File(FileConfig),
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum SourceFormat {
    Json,
    Yaml,
    Xml,
    Text,
}
