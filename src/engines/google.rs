use crate::engines::error::EngineError;
use crate::engines::SearchEngine;
use crate::models::{SearchQuery, SearchResult};
use async_trait::async_trait;
use reqwest::Client;

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

    async fn search(&self, query: &SearchQuery, _client: &Client) -> Result<Vec<SearchResult>, EngineError> {
        // For now, return a mock result to validate multi-engine support.
        // In the future, this will implement actual scraping or API calls.
        let results = vec![
            SearchResult {
                url: "https://www.google.com/search?q=rust".to_string(),
                title: format!("Google Search Result for {}", query.q),
                content: "This is a mock result from the Google engine.".to_string(),
                engine: self.id(),
                score: 1.0,
            }
        ];

        // Simulate some network delay
        tokio::time::sleep(std::time::Duration::from_millis(100)).await;

        Ok(results)
    }
}
