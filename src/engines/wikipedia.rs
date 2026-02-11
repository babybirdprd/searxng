use crate::config::EngineConfig;
use crate::engines::error::EngineError;
use crate::engines::SearchEngine;
use crate::models::{ResultContent, SearchQuery, SearchResult};
use async_trait::async_trait;
use reqwest::Client;
use std::collections::HashMap;

pub struct Wikipedia;

#[async_trait]
impl SearchEngine for Wikipedia {
    fn id(&self) -> String {
        "wikipedia".to_string()
    }

    fn name(&self) -> String {
        "Wikipedia".to_string()
    }

    fn categories(&self) -> Vec<String> {
        vec!["general".to_string()]
    }

    async fn search(
        &self,
        query: &SearchQuery,
        client: &Client,
        _config: &EngineConfig,
    ) -> Result<Vec<SearchResult>, EngineError> {
        let language = if query.language.is_empty() {
            "en"
        } else {
            query.language.split('-').next().unwrap_or("en")
        };

        let url = format!("https://{}.wikipedia.org/w/api.php", language);

        let limit = 20;
        let offset = (query.page - 1) * limit;

        let params = [
            ("action", "query"),
            ("format", "json"),
            ("generator", "search"),
            ("gsrsearch", &query.q),
            ("gsrlimit", &limit.to_string()),
            ("gsroffset", &offset.to_string()),
            ("prop", "pageimages|extracts"),
            ("piprop", "thumbnail"),
            ("pithumbsize", "300"),
            ("exintro", "1"),
            ("explaintext", "1"),
            ("exsentences", "2"),
        ];

        let resp = client.get(&url).query(&params).send().await?;

        if !resp.status().is_success() {
             return Err(EngineError::Unexpected(anyhow::anyhow!("Wikipedia returned {}", resp.status())));
        }

        let body: serde_json::Value = resp.json().await?;

        let mut results = Vec::new();

        if let Some(pages) = body["query"]["pages"].as_object() {
            for (_, page) in pages {
                let title = page["title"].as_str().unwrap_or_default().to_string();
                let extract = page["extract"].as_str().unwrap_or_default().to_string();

                let page_url = format!("https://{}.wikipedia.org/wiki/{}", language, title.replace(' ', "_"));

                let content = if let Some(thumbnail) = page["thumbnail"]["source"].as_str() {
                    ResultContent::Image {
                        src: thumbnail.to_string(),
                        thumbnail: Some(thumbnail.to_string()),
                    }
                } else {
                    ResultContent::Text(extract)
                };

                // If it's an image, we still want the text snippet?
                // Actually SearchResult has only one content.
                // In SearXNG, Wikipedia often returns a text result with a thumbnail metadata.
                // Our models.rs ResultContent is an enum.

                results.push(SearchResult {
                    url: page_url,
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
