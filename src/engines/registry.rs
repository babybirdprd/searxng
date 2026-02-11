use crate::config::Settings;
use crate::engines::aggregator::aggregate;
use crate::engines::SearchEngine;
use crate::models::{SearchQuery, SearchResult};
use reqwest::Client;
use std::collections::HashMap;
use std::sync::Arc;
use std::time::Duration;
use tokio::task::JoinSet;

struct EngineEntry {
    engine: Arc<dyn SearchEngine>,
    categories: Vec<String>,
}

pub struct EngineRegistry {
    engines: HashMap<String, EngineEntry>,
    settings: Arc<Settings>,
}

impl EngineRegistry {
    pub fn new(settings: Arc<Settings>) -> Self {
        Self {
            engines: HashMap::new(),
            settings,
        }
    }

    pub fn register_engine(&mut self, engine: Box<dyn SearchEngine>) {
        let categories = engine.categories();
        let id = engine.id();
        let entry = EngineEntry {
            engine: Arc::from(engine),
            categories,
        };
        self.engines.insert(id, entry);
    }

    pub async fn search(&self, query: &SearchQuery, client: &Client) -> Vec<SearchResult> {
        let mut join_set = JoinSet::new();
        let query_categories = query.get_categories();

        for (id, entry) in &self.engines {
            // Get configuration for this engine
            let config = self.settings.engines.get(id).cloned().unwrap_or_default();

            if !config.enabled {
                continue;
            }

            // Check if engine supports any of the query categories
            let category_match = query_categories.iter().any(|c| entry.categories.contains(c));

            if !category_match {
                continue;
            }

            let engine = entry.engine.clone();
            let query = query.clone();
            let client = client.clone();
            let id = id.clone();

            join_set.spawn(async move {
                let timeout_duration = Duration::from_secs(config.timeout);
                match tokio::time::timeout(timeout_duration, engine.search(&query, &client)).await {
                    Ok(result) => match result {
                        Ok(mut results) => {
                            // Apply weight
                            for res in &mut results {
                                res.score *= config.weight;
                            }
                            results
                        }
                        Err(e) => {
                            tracing::error!("Engine {} failed: {}", id, e);
                            vec![]
                        }
                    },
                    Err(_) => {
                        tracing::warn!("Engine {} timed out", id);
                        vec![]
                    }
                }
            });
        }

        let mut raw_results = Vec::new();
        while let Some(res) = join_set.join_next().await {
            match res {
                Ok(results) => raw_results.extend(results),
                Err(e) => tracing::error!("Task join error: {}", e),
            }
        }

        aggregate(raw_results)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::engines::error::EngineError;
    use crate::models::{ResultContent, SearchResult};
    use async_trait::async_trait;

    struct MockEngine {
        id: String,
        categories: Vec<String>,
    }

    #[async_trait]
    impl SearchEngine for MockEngine {
        fn id(&self) -> String {
            self.id.clone()
        }
        fn name(&self) -> String {
            self.id.clone()
        }
        fn categories(&self) -> Vec<String> {
            self.categories.clone()
        }
        async fn search(
            &self,
            _query: &SearchQuery,
            _client: &Client,
        ) -> Result<Vec<SearchResult>, EngineError> {
            Ok(vec![SearchResult {
                url: format!("http://{}", self.id),
                title: self.id.clone(),
                content: ResultContent::Text("content".to_string()),
                engine: self.id.clone(),
                score: 1.0,
                metadata: HashMap::new(),
            }])
        }
    }

    #[tokio::test]
    async fn test_search_category_filtering() {
        // Construct a dummy settings object manually to avoid file I/O dependency
        let settings = Arc::new(Settings {
             server: crate::config::ServerSettings {
                 bind_address: "127.0.0.1".into(),
                 port: 8080,
                 secret_key: "secret".into(),
             },
             debug: false,
             engines: HashMap::new(),
        });

        let mut registry = EngineRegistry::new(settings);

        registry.register_engine(Box::new(MockEngine {
            id: "general_engine".to_string(),
            categories: vec!["general".to_string()],
        }));
        registry.register_engine(Box::new(MockEngine {
            id: "image_engine".to_string(),
            categories: vec!["images".to_string()],
        }));

        let client = Client::new();

        // 1. Test "general" category (default)
        let query_general = SearchQuery {
            q: "test".to_string(),
            ..Default::default()
        };
        let results = registry.search(&query_general, &client).await;
        // Should include general_engine, but NOT image_engine
        assert!(results.iter().any(|r| r.engine == "general_engine"), "general_engine should match default category");
        assert!(!results.iter().any(|r| r.engine == "image_engine"), "image_engine should NOT match default category");

        // 2. Test "images" category
        let query_images = SearchQuery {
            q: "test".to_string(),
            categories: "images".to_string(),
            ..Default::default()
        };
        let results = registry.search(&query_images, &client).await;
        assert!(!results.iter().any(|r| r.engine == "general_engine"), "general_engine should NOT match images category");
        assert!(results.iter().any(|r| r.engine == "image_engine"), "image_engine should match images category");

        // 3. Test both
        let query_both = SearchQuery {
            q: "test".to_string(),
            categories: "general,images".to_string(),
            ..Default::default()
        };
        let results = registry.search(&query_both, &client).await;
        assert!(results.iter().any(|r| r.engine == "general_engine"), "general_engine should match combined category");
        assert!(results.iter().any(|r| r.engine == "image_engine"), "image_engine should match combined category");
    }
}
