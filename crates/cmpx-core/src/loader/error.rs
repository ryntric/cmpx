use thiserror::Error;

#[derive(Debug, Error)]
pub enum LoadSourceError {
    #[error("I/O error: {0}")]
    Io(#[from] std::io::Error),

    #[error("HTTP error: {0}")]
    Http(#[from] reqwest::Error),
}
