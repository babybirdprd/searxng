use thiserror::Error;

#[derive(Error, Debug)]
pub enum EngineError {
    #[error("Network error: {0}")]
    Network(#[from] reqwest::Error),
    #[error("Parsing error: {0}")]
    Parsing(String),
    #[error("Timeout")]
    Timeout,
    #[error("Rate limited")]
    RateLimited,
    #[error("Unexpected error: {0}")]
    Unexpected(#[from] anyhow::Error),
}
