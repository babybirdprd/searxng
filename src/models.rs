use serde::{Deserialize, Serialize};

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
pub struct SearchResult {
    pub url: String,
    pub title: String,
    pub content: String,
    pub engine: String,
    pub score: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EngineMetadata {
    pub name: String,
    pub display_name: String,
    pub enabled: bool,
}
