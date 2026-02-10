pub mod dummy;
pub mod registry;
pub mod error;
pub mod aggregator;
pub mod duckduckgo;

use crate::models::{SearchQuery, SearchResult};
use async_trait::async_trait;
use reqwest::Client;
use error::EngineError;

#[async_trait]
pub trait SearchEngine: Send + Sync {
    /// Unique identifier for the engine (e.g. "google", "ddg").
    fn id(&self) -> String;

    /// Human-readable name.
    fn name(&self) -> String;

    /// Categories the engine supports (e.g. "general", "images").
    fn categories(&self) -> Vec<String> {
        vec!["general".to_string()]
    }

    /// Weight for result ranking. Higher means more important.
    fn weight(&self) -> f64 {
        1.0
    }

    /// Perform the search.
    async fn search(&self, query: &SearchQuery, client: &Client) -> Result<Vec<SearchResult>, EngineError>;
}
