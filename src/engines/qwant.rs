use crate::config::EngineConfig;
use crate::engines::error::EngineError;
use crate::engines::SearchEngine;
use crate::models::{ResultContent, SearchQuery, SearchResult};
use async_trait::async_trait;
use reqwest::Client;
use std::collections::HashMap;

pub struct Qwant;

#[async_trait]
impl SearchEngine for Qwant {
    fn id(&self) -> String {
        "qwant".to_string()
    }

    fn name(&self) -> String {
        "Qwant".to_string()
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
        let url = "https://api.qwant.com/v3/search/web";

        let count = 10;
        let offset = (query.page - 1) * count;

        let language = if query.language.is_empty() {
            "en_US"
        } else {
            &query.language
        };

        let params = [
            ("q", query.q.as_str()),
            ("count", &count.to_string()),
            ("offset", &offset.to_string()),
            ("locale", language),
            ("safesearch", &query.safesearch.to_string()),
        ];

        let resp = client.get(url)
            .query(&params)
            .header("User-Agent", crate::engines::DEFAULT_USER_AGENT)
            .send().await?;

        if !resp.status().is_success() {
             return Err(EngineError::Unexpected(anyhow::anyhow!("Qwant returned {}", resp.status())));
        }

        let body: serde_json::Value = resp.json().await?;

        if body["status"].as_str() != Some("success") {
            return Err(EngineError::Unexpected(anyhow::anyhow!("Qwant API error: {:?}", body["status"])));
        }

        let mut results = Vec::new();

        if let Some(mainline) = body["data"]["result"]["items"]["mainline"].as_array() {
            for row in mainline {
                let row_type = row["type"].as_str().unwrap_or_default();
                if row_type != "web" && row_type != "videos" {
                    continue;
                }

                if let Some(items) = row["items"].as_array() {
                    for item in items {
                        let title = item["title"].as_str().unwrap_or_default().to_string();
                        let url = item["url"].as_str().unwrap_or_default().to_string();
                        let desc = item["desc"].as_str().unwrap_or_default().to_string();

                        let content = if row_type == "videos" {
                            ResultContent::Video {
                                src: url.clone(),
                                thumbnail: item["thumbnail"].as_str().map(|s| s.to_string()),
                                duration: item["duration"].as_u64().map(|d| format!("{}s", d / 1000)),
                            }
                        } else {
                            ResultContent::Text(desc)
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
            }
        }

        Ok(results)
    }
}
