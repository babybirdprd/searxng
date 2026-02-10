use crate::engines::error::EngineError;
use crate::engines::SearchEngine;
use crate::models::{SearchQuery, SearchResult};
use async_trait::async_trait;
use reqwest::Client;
use scraper::{Html, Selector};

pub struct DuckDuckGo;

#[async_trait]
impl SearchEngine for DuckDuckGo {
    fn id(&self) -> String {
        "duckduckgo".to_string()
    }

    fn name(&self) -> String {
        "DuckDuckGo".to_string()
    }

    fn categories(&self) -> Vec<String> {
        vec!["general".to_string()]
    }

    async fn search(&self, query: &SearchQuery, client: &Client) -> Result<Vec<SearchResult>, EngineError> {
        let url = "https://html.duckduckgo.com/html/";

        let params = [
            ("q", query.q.as_str()),
            ("b", ""),
            ("kl", "wt-wt"),
        ];

        let resp = client.post(url)
            .form(&params)
            .send()
            .await?;

        if !resp.status().is_success() {
            return Err(EngineError::Unexpected(anyhow::anyhow!("DuckDuckGo returned status: {}", resp.status())));
        }

        let text = resp.text().await?;
        let document = Html::parse_document(&text);

        // Selectors
        let result_selector = Selector::parse("div#links > div.web-result").expect("Invalid selector");
        let title_selector = Selector::parse("h2 > a").expect("Invalid selector");
        let snippet_selector = Selector::parse("a.result__snippet").expect("Invalid selector");

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

            let content = match element.select(&snippet_selector).next() {
                Some(el) => el.text().collect::<Vec<_>>().join(" "),
                None => String::new(),
            };

            results.push(SearchResult {
                url,
                title,
                content,
                engine: self.id(),
                score: 1.0,
            });
        }

        Ok(results)
    }
}
