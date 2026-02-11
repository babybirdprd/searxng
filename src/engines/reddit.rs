use crate::config::EngineConfig;
use crate::engines::error::EngineError;
use crate::engines::SearchEngine;
use crate::models::{ResultContent, SearchQuery, SearchResult};
use async_trait::async_trait;
use reqwest::Client;
use std::collections::HashMap;

pub struct Reddit;

#[async_trait]
impl SearchEngine for Reddit {
    fn id(&self) -> String {
        "reddit".to_string()
    }

    fn name(&self) -> String {
        "Reddit".to_string()
    }

    fn categories(&self) -> Vec<String> {
        vec!["general".to_string(), "social media".to_string()]
    }

    async fn search(
        &self,
        query: &SearchQuery,
        client: &Client,
        _config: &EngineConfig,
    ) -> Result<Vec<SearchResult>, EngineError> {
        let url = "https://www.reddit.com/search.json";

        let limit = 20;
        // Reddit uses after/before for paging, but we can try to use a large limit or just one page for now as it's a port.
        // Or we can simulate it.

        let mut params = vec![
            ("q", query.q.clone()),
            ("limit", limit.to_string()),
        ];

        if query.safesearch == 0 {
             params.push(("include_over_18", "on".to_string()));
        }

        // Reddit doesn't use simple page numbers, but we'll do our best.
        // For now, let's just fetch the first page or use "after" if we had it.

        let resp = client.get(url)
            .query(&params)
            // Reddit requires a custom User-Agent to avoid 429
            .header("User-Agent", "Mozilla/5.0 (compatible; SearXNG-rs/0.1.0; +https://github.com/searxng/searxng-rs)")
            .send().await?;

        if !resp.status().is_success() {
            return Err(EngineError::Unexpected(anyhow::anyhow!("Reddit returned {}", resp.status())));
        }

        let body: serde_json::Value = resp.json().await?;
        let mut results = Vec::new();

        if let Some(children) = body["data"]["children"].as_array() {
            for child in children {
                let data = &child["data"];
                let title = data["title"].as_str().unwrap_or_default().to_string();
                let permalink = data["permalink"].as_str().unwrap_or_default();
                let url = format!("https://www.reddit.com{}", permalink);
                let selftext = data["selftext"].as_str().unwrap_or_default().to_string();
                let thumbnail = data["thumbnail"].as_str().filter(|s| s.starts_with("http")).map(|s| s.to_string());

                let is_video = data["is_video"].as_bool().unwrap_or(false);
                let content = if is_video {
                    ResultContent::Video {
                        src: data["url"].as_str().unwrap_or_default().to_string(),
                        thumbnail: thumbnail.clone(),
                        duration: None, // Reddit API has it somewhere else or not easily available here
                    }
                } else if let Some(t) = thumbnail {
                    ResultContent::Image {
                        src: data["url"].as_str().unwrap_or_default().to_string(),
                        thumbnail: Some(t),
                    }
                } else {
                    ResultContent::Text(selftext)
                };

                results.push(SearchResult {
                    url,
                    title,
                    content,
                    engines: vec![self.id()],
                    score: 1.0,
                    metadata: HashMap::new(),
                });
            }
        }

        Ok(results)
    }
}
