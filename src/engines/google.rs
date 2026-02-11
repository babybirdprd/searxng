use crate::config::EngineConfig;
use crate::engines::error::EngineError;
use crate::engines::SearchEngine;
use crate::models::{ResultContent, SearchQuery, SearchResult};
use async_trait::async_trait;
use reqwest::Client;
use scraper::{Html, Selector};
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
        client: &Client,
        _config: &EngineConfig,
    ) -> Result<Vec<SearchResult>, EngineError> {
        let url = "https://www.google.com/search";

        let start = (query.page - 1) * 10;

        let mut params = vec![
            ("q", query.q.clone()),
            ("start", start.to_string()),
        ];

        if query.safesearch > 0 {
            params.push(("safe", "active".to_string()));
        } else {
            params.push(("safe", "off".to_string()));
        }

        let resp = client.get(url).query(&params).send().await?;

        if !resp.status().is_success() {
             return Err(EngineError::Unexpected(anyhow::anyhow!("Google returned {}", resp.status())));
        }

        let text = resp.text().await?;
        let document = Html::parse_document(&text);

        // Google HTML can be tricky and changes often.
        // These selectors are for a basic non-JS version if possible, but
        // Google often returns different HTML based on User-Agent.
        let result_selector = Selector::parse("div.g")
            .map_err(|e| EngineError::Parsing(format!("Invalid result selector: {:?}", e)))?;
        let title_selector = Selector::parse("h3")
            .map_err(|e| EngineError::Parsing(format!("Invalid title selector: {:?}", e)))?;
        let url_selector = Selector::parse("a")
            .map_err(|e| EngineError::Parsing(format!("Invalid url selector: {:?}", e)))?;
        let snippet_selector = Selector::parse("div.VwiC3b, div.s, .st")
            .map_err(|e| EngineError::Parsing(format!("Invalid snippet selector: {:?}", e)))?;

        let mut results = Vec::new();

        for element in document.select(&result_selector) {
            let title_element = match element.select(&title_selector).next() {
                Some(el) => el,
                None => continue,
            };

            let title = title_element.text().collect::<Vec<_>>().join(" ");

            let url = match element.select(&url_selector).next().and_then(|el| el.value().attr("href")) {
                Some(href) => {
                    if href.starts_with("/url?q=") {
                        // Extract actual URL from Google redirect
                        let parts: Vec<&str> = href.split("/url?q=").collect();
                        if parts.len() > 1 {
                            parts[1].split('&').next().unwrap_or(href).to_string()
                        } else {
                            href.to_string()
                        }
                    } else {
                        href.to_string()
                    }
                },
                None => continue,
            };

            let content_text = match element.select(&snippet_selector).next() {
                Some(el) => el.text().collect::<Vec<_>>().join(" "),
                None => String::new(),
            };

            results.push(SearchResult {
                url,
                title,
                content: ResultContent::Text(content_text),
                engines: vec![self.id()],
                score: 1.0,
                metadata: HashMap::new(),
            });
        }

        Ok(results)
    }
}
