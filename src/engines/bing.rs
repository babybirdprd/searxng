use crate::config::EngineConfig;
use crate::engines::error::EngineError;
use crate::engines::SearchEngine;
use crate::models::{ResultContent, SearchQuery, SearchResult};
use async_trait::async_trait;
use reqwest::Client;
use scraper::{Html, Selector};
use std::collections::HashMap;

pub struct Bing;

#[async_trait]
impl SearchEngine for Bing {
    fn id(&self) -> String {
        "bing".to_string()
    }

    fn name(&self) -> String {
        "Bing".to_string()
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
        let url = "https://www.bing.com/search";

        let first = (query.page - 1) * 10 + 1;

        let mut params = vec![
            ("q", query.q.clone()),
            ("first", first.to_string()),
        ];

        if query.safesearch > 0 {
            params.push(("adlt", if query.safesearch == 1 { "moderate".to_string() } else { "strict".to_string() }));
        } else {
            params.push(("adlt", "off".to_string()));
        }

        let resp = client.get(url).query(&params).send().await?;

        if !resp.status().is_success() {
             return Err(EngineError::Unexpected(anyhow::anyhow!("Bing returned {}", resp.status())));
        }

        let text = resp.text().await?;
        let document = Html::parse_document(&text);

        let result_selector = Selector::parse("li.b_algo")
            .map_err(|e| EngineError::Parsing(format!("Invalid result selector: {:?}", e)))?;
        let title_selector = Selector::parse("h2 > a")
            .map_err(|e| EngineError::Parsing(format!("Invalid title selector: {:?}", e)))?;
        let snippet_selector = Selector::parse(".b_caption p, .b_algo_text")
            .map_err(|e| EngineError::Parsing(format!("Invalid snippet selector: {:?}", e)))?;

        let mut results = Vec::new();

        for element in document.select(&result_selector) {
            let title_element = match element.select(&title_selector).next() {
                Some(el) => el,
                None => continue,
            };

            let title = title_element.text().collect::<Vec<_>>().join(" ");
            let url = match title_element.value().attr("href") {
                Some(href) => href.to_string(),
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
