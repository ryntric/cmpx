use super::Loadable;

use crate::loader::error::SourceLoadError;
use crate::source::http::{HttpMethod, HttpSource};

use reqwest::Method;

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

impl Loadable for HttpSource {
    async fn load(&self) -> Result<Vec<u8>, SourceLoadError> {
        let response = reqwest::Client::new()
            .request(self.method().into(), self.url())
            .send()
            .await?;

        Ok(response.bytes().await?.to_vec())
    }
}
