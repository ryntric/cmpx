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
pub struct HttpConfig {
    url: String,
    #[serde(default)]
    method: HttpMethod,
}

impl HttpConfig {
    fn new(url: impl Into<String>) -> Self {
        Self {
            url: url.into(),
            method: HttpMethod::default(),
        }
    }

    pub fn builder(url: impl Into<String>) -> HttpConfigBuilder {
        HttpConfigBuilder::new(url)
    }

    pub(crate) fn method(&self) -> HttpMethod {
        self.method
    }

    pub(crate) fn url(&self) -> &str {
        &self.url
    }
}

pub struct HttpConfigBuilder {
    config: HttpConfig,
}

impl HttpConfigBuilder {
    fn new(url: impl Into<String>) -> Self {
        Self {
            config: HttpConfig::new(url.into()),
        }
    }

    pub fn method(mut self, method: HttpMethod) -> Self {
        self.config.method = method;
        self
    }

    pub fn build(self) -> HttpConfig {
        self.config
    }
}
