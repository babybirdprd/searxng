use crate::engines::SearchEngine;
use crate::models::{SearchQuery, SearchResult};
use anyhow::Result;
use async_trait::async_trait;
use reqwest::Client;

pub struct DummyEngine;

#[async_trait]
impl SearchEngine for DummyEngine {
    fn name(&self) -> String {
        "dummy".to_string()
    }

    async fn search(&self, query: &SearchQuery, _client: &Client) -> Result<Vec<SearchResult>> {
        let results = vec![SearchResult {
            url: "https://example.com".to_string(),
            title: format!("Example result for {}", query.q),
            content: "This is a dummy result".to_string(),
            engine: "dummy".to_string(),
            score: 1.0,
        }];
        Ok(results)
    }
}
