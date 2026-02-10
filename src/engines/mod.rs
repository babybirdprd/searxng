use crate::models::{SearchQuery, SearchResult};
use async_trait::async_trait;

pub mod dummy;

#[async_trait]
pub trait SearchEngine: Send + Sync {
    fn name(&self) -> String;
    async fn search(&self, query: &SearchQuery) -> Result<Vec<SearchResult>, anyhow::Error>;
}
