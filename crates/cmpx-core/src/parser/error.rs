use thiserror::Error;

#[derive(Debug, Error)]
pub enum ParserError {
    #[error("Deserialization error: {0}")]
    DeserializationError(#[from] serde_json::Error),
}
