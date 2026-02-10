use crate::engines::{SearchEngine, error::EngineError};
use crate::models::{SearchQuery, SearchResult};
use async_trait::async_trait;
use reqwest::Client;

pub struct DummyEngine;

#[async_trait]
impl SearchEngine for DummyEngine {
    fn id(&self) -> String {
        "dummy".to_string()
    }

    fn name(&self) -> String {
        "Dummy Engine".to_string()
    }

    fn categories(&self) -> Vec<String> {
        vec!["general".to_string()]
    }

    fn weight(&self) -> f64 {
        1.0
    }

    async fn search(&self, query: &SearchQuery, _client: &Client) -> Result<Vec<SearchResult>, EngineError> {
        // Simulate an asynchronous operation
        tokio::time::sleep(tokio::time::Duration::from_millis(50)).await;

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
