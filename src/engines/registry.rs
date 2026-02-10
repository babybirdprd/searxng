use crate::engines::SearchEngine;
use crate::models::{SearchQuery, SearchResult};
use reqwest::Client;
use std::collections::HashMap;
use std::sync::Arc;
use tokio::task::JoinSet;

pub struct EngineRegistry {
    engines: HashMap<String, Arc<dyn SearchEngine>>,
}

impl EngineRegistry {
    pub fn new() -> Self {
        Self {
            engines: HashMap::new(),
        }
    }

    pub fn register_engine(&mut self, engine: Box<dyn SearchEngine>) {
        self.engines.insert(engine.name(), Arc::from(engine));
    }

    pub async fn search(&self, query: &SearchQuery, client: &Client) -> Vec<SearchResult> {
        let mut join_set = JoinSet::new();

        // TODO: Logic to select specific engines based on query or preferences.
        // For now, run all registered engines.
        for (name, engine) in &self.engines {
            let engine = engine.clone();
            let query = query.clone();
            let client = client.clone();
            let name = name.clone();

            join_set.spawn(async move {
                match engine.search(&query, &client).await {
                    Ok(results) => results,
                    Err(e) => {
                        tracing::error!("Engine {} failed: {}", name, e);
                        vec![]
                    }
                }
            });
        }

        let mut aggregated_results = Vec::new();
        while let Some(res) = join_set.join_next().await {
            match res {
                Ok(results) => aggregated_results.extend(results),
                Err(e) => tracing::error!("Task join error: {}", e),
            }
        }

        aggregated_results
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::engines::dummy::DummyEngine;

    #[tokio::test]
    async fn test_registry_search() {
        let mut registry = EngineRegistry::new();
        registry.register_engine(Box::new(DummyEngine));

        let client = Client::builder().build().unwrap();
        let query = SearchQuery {
            q: "test".to_string(),
            ..Default::default()
        };

        let results = registry.search(&query, &client).await;
        assert!(!results.is_empty());
        assert_eq!(results[0].engine, "dummy");
    }
}
