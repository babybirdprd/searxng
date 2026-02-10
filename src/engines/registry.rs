use crate::engines::SearchEngine;
use crate::models::{SearchQuery, SearchResult};
use reqwest::Client;
use std::collections::HashMap;
use std::sync::Arc;
use tokio::task::JoinSet;
use std::time::Duration;

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
        self.engines.insert(engine.id(), Arc::from(engine));
    }

    pub async fn search(&self, query: &SearchQuery, client: &Client) -> Vec<SearchResult> {
        let mut join_set = JoinSet::new();

        // TODO: Logic to select specific engines based on query or preferences.
        // For now, run all registered engines.
        for (id, engine) in &self.engines {
            let engine = engine.clone();
            let query = query.clone();
            let client = client.clone();
            let id = id.clone();

            join_set.spawn(async move {
                // Add a timeout to each engine search
                let timeout_duration = Duration::from_secs(2);
                match tokio::time::timeout(timeout_duration, engine.search(&query, &client)).await {
                    Ok(result) => match result {
                        Ok(results) => results,
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

        let mut aggregated_results = Vec::new();
        while let Some(res) = join_set.join_next().await {
            match res {
                Ok(results) => aggregated_results.extend(results),
                Err(e) => tracing::error!("Task join error: {}", e),
            }
        }

        // TODO: Advanced ranking and deduplication logic here.
        // For now, just return concatenated results.

        aggregated_results
    }
}
