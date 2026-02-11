use crate::config::Settings;
use crate::engines::aggregator::aggregate;
use crate::engines::SearchEngine;
use crate::models::{SearchQuery, SearchResult};
use reqwest::Client;
use std::collections::HashMap;
use std::sync::Arc;
use std::time::Duration;
use tokio::task::JoinSet;

pub struct EngineRegistry {
    engines: HashMap<String, Arc<dyn SearchEngine>>,
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
        self.engines.insert(engine.id(), Arc::from(engine));
    }

    pub async fn search(&self, query: &SearchQuery, client: &Client) -> Vec<SearchResult> {
        let mut join_set = JoinSet::new();

        for (id, engine) in &self.engines {
            // Get configuration for this engine
            let config = self.settings.engines.get(id).cloned().unwrap_or_default();

            if !config.enabled {
                continue;
            }

            let engine = engine.clone();
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
