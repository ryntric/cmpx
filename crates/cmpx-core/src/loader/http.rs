use super::Loader;

use crate::loader::error::LoadSourceError;
use crate::source::http::{HttpConfig, HttpMethod};

use crate::loader::LoadedSource;
use crate::source::Source;
use reqwest::{Client, Method};

#[derive(Default)]
pub(crate) struct HttpLoader {
    client: Client,
}

impl HttpLoader {
    pub fn new() -> HttpLoader {
        Self {
            client: Client::default(),
        }
    }
}

impl Loader<HttpConfig> for HttpLoader {
    async fn load(
        &self,
        source: &Source,
        config: &HttpConfig,
    ) -> Result<LoadedSource, LoadSourceError> {
        let response = self
            .client
            .request(config.method().into(), config.url())
            .send()
            .await?;

        let bytes = response.bytes().await?.to_vec();
        Ok(LoadedSource::new(source, bytes, None))
    }
}

#[derive(Debug)]
pub struct HttpMetadata {
    method: HttpMethod,
}

impl From<HttpMethod> for Method {
    fn from(value: HttpMethod) -> Self {
        match value {
            HttpMethod::Get => Self::GET,
            HttpMethod::Post => Self::POST,
            HttpMethod::Put => Self::PUT,
            HttpMethod::Delete => Self::DELETE,
            HttpMethod::Patch => Self::PATCH,
            HttpMethod::Query => Self::QUERY,
        }
    }
}
