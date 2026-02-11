use serde::{Deserialize, Serialize};
use std::collections::HashMap;

fn default_page() -> u32 {
    1
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct SearchQuery {
    pub q: String,
    #[serde(default)]
    pub language: String,
    #[serde(default = "default_page")]
    pub page: u32,
    #[serde(default)]
    pub safesearch: u8,
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
    pub engine: String,
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
