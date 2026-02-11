use serde::{Deserialize, Serialize};
use std::collections::HashMap;

fn default_page() -> u32 {
    1
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SearchQuery {
    pub q: String,
    #[serde(default)]
    pub language: String,
    #[serde(default = "default_page")]
    pub page: u32,
    #[serde(default)]
    pub safesearch: u8,
    #[serde(default)]
    pub categories: String,
    #[serde(default)]
    pub time_range: String,
    #[serde(default)]
    pub format: String,
}

impl Default for SearchQuery {
    fn default() -> Self {
        Self {
            q: "".to_string(),
            language: "".to_string(),
            page: default_page(),
            safesearch: 0,
            categories: "".to_string(),
            time_range: "".to_string(),
            format: "".to_string(),
        }
    }
}

impl SearchQuery {
    pub fn get_categories(&self) -> Vec<String> {
        if self.categories.is_empty() {
            return vec!["general".to_string()];
        }
        self.categories
            .split(',')
            .map(|s| s.trim().to_string())
            .filter(|s| !s.is_empty())
            .collect()
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type", content = "data")]
pub enum ResultContent {
    Text(String),
    Image { src: String, thumbnail: Option<String> },
    Video { src: String, thumbnail: Option<String>, duration: Option<String> },
    Map { latitude: f64, longitude: f64, zoom: Option<u8> },
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SearchResult {
    pub url: String,
    pub title: String,
    pub content: ResultContent,
    pub engines: Vec<String>,
    pub score: f64,
    #[serde(default)]
    pub metadata: HashMap<String, String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EngineMetadata {
    pub name: String,
    pub display_name: String,
    pub enabled: bool,
}
