use serde::{Deserialize, Serialize};

#[derive(Clone, Copy, Debug, Default, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum HttpMethod {
    #[default]
    Get,
    Post,
    Put,
    Delete,
    Patch,
    Query,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct HttpSource {
    url: String,
    #[serde(default)]
    method: HttpMethod,
}

impl HttpSource {
    fn new(url: impl Into<String>) -> Self {
        Self {
            url: url.into(),
            method: HttpMethod::default(),
        }
    }

    pub fn builder(url: impl Into<String>) -> HttpSourceBuilder {
        HttpSourceBuilder::new(url)
    }

    pub(crate) fn method(&self) -> HttpMethod {
        self.method
    }

    pub(crate) fn url(&self) -> &str {
        &self.url
    }
}

pub struct HttpSourceBuilder {
    source: HttpSource,
}

impl HttpSourceBuilder {
    fn new(url: impl Into<String>) -> Self {
        Self {
            source: HttpSource::new(url),
        }
    }

    pub fn method(mut self, method: HttpMethod) -> Self {
        self.source.method = method;
        self
    }

    pub fn build(self) -> HttpSource {
        self.source
    }
}
