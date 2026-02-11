use crate::config::EngineConfig;
use crate::engines::error::EngineError;
use crate::engines::SearchEngine;
use crate::models::{ResultContent, SearchQuery, SearchResult};
use async_trait::async_trait;
use reqwest::Client;
use std::collections::HashMap;

pub struct Google;

#[async_trait]
impl SearchEngine for Google {
    fn id(&self) -> String {
        "google".to_string()
    }

    fn name(&self) -> String {
        "Google".to_string()
    }

    fn categories(&self) -> Vec<String> {
        vec!["general".to_string()]
    }

    async fn search(
        &self,
        query: &SearchQuery,
        _client: &Client,
        _config: &EngineConfig,
    ) -> Result<Vec<SearchResult>, EngineError> {
        // For now, return a mock result to validate multi-engine support.
        // In the future, this will implement actual scraping or API calls.
        let results = vec![SearchResult {
            url: "https://www.google.com/search?q=rust".to_string(),
            title: format!("Google Search Result for {}", query.q),
            content: ResultContent::Text("This is a mock result from the Google engine.".to_string()),
            engines: vec![self.id()],
            score: 1.0,
            metadata: HashMap::new(),
        }];

        // Simulate some network delay
        tokio::time::sleep(std::time::Duration::from_millis(100)).await;

        Ok(results)
    }
}
