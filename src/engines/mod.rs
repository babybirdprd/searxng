use crate::models::{SearchQuery, SearchResult};
use async_trait::async_trait;
use reqwest::Client;

pub mod dummy;
pub mod registry;

#[async_trait]
pub trait SearchEngine: Send + Sync {
    fn name(&self) -> String;
    async fn search(&self, query: &SearchQuery, client: &Client) -> Result<Vec<SearchResult>, anyhow::Error>;
}
