pub mod aggregator;
pub mod circuit_breaker;
pub mod duckduckgo;
pub mod dummy;
pub mod error;
pub mod google;
pub mod registry;

use crate::config::EngineConfig;
use crate::models::{SearchQuery, SearchResult};
use async_trait::async_trait;
use error::EngineError;
use reqwest::Client;

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
    async fn search(
        &self,
        query: &SearchQuery,
        client: &Client,
        config: &EngineConfig,
    ) -> Result<Vec<SearchResult>, EngineError>;
}

pub fn create_client(user_agent: &str, proxy: Option<&str>) -> reqwest::Result<Client> {
    let mut builder = Client::builder().user_agent(user_agent);

    if let Some(proxy_url) = proxy {
        builder = builder.proxy(reqwest::Proxy::all(proxy_url)?);
    }

    builder.build()
}
