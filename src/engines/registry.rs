use crate::config::{EngineConfig, Settings};
use crate::engines::aggregator::aggregate;
use crate::engines::SearchEngine;
use crate::models::{SearchQuery, SearchResult};
use reqwest::Client;
use std::collections::HashMap;
use std::sync::Arc;
use std::time::Duration;
use tokio::sync::Mutex;
use tokio::task::JoinSet;

struct CircuitBreaker {
    failures: u32,
    cooldown_until: Option<std::time::Instant>,
}

struct EngineEntry {
    engine: Arc<dyn SearchEngine>,
    categories: Vec<String>,
    config: EngineConfig,
    last_request: Arc<Mutex<Option<std::time::Instant>>>,
    circuit_breaker: Arc<Mutex<CircuitBreaker>>,
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
        let config = self
            .settings
            .engines
            .get(&id)
            .cloned()
            .unwrap_or_default();

        let entry = EngineEntry {
            engine: Arc::from(engine),
            categories,
            config,
            last_request: Arc::new(Mutex::new(None)),
            circuit_breaker: Arc::new(Mutex::new(CircuitBreaker {
                failures: 0,
                cooldown_until: None,
            })),
        };
        self.engines.insert(id, entry);
    }

    pub async fn search(&self, query: &SearchQuery, client: &Client) -> Vec<SearchResult> {
        let mut join_set = JoinSet::new();
        let query_categories = query.get_categories();

        for (id, entry) in &self.engines {
            if !entry.config.enabled {
                continue;
            }

            // Check if engine supports any of the query categories
            let category_match = query_categories.iter().any(|c| entry.categories.contains(c));

            if !category_match {
                continue;
            }

            // Circuit Breaker Check
            let circuit_breaker = entry.circuit_breaker.clone();
            {
                let cb = circuit_breaker.lock().await;
                if let Some(cooldown) = cb.cooldown_until {
                    if std::time::Instant::now() < cooldown {
                        tracing::debug!("Engine {} is in cooldown", id);
                        continue;
                    }
                }
            }

            let engine = entry.engine.clone();
            let query = query.clone();
            let client = client.clone();
            let id = id.clone();
            let config = entry.config.clone();
            let last_request = entry.last_request.clone();

            join_set.spawn(async move {
                // Throttling Logic
                if config.throttle > 0 {
                    let sleep_duration = {
                        let mut last = last_request.lock().await;
                        let now = std::time::Instant::now();
                        let throttle_duration = Duration::from_millis(config.throttle);

                        let (wait, new_last) = match *last {
                            Some(last_time) => {
                                let target = last_time + throttle_duration;
                                if target > now {
                                    (Some(target - now), target)
                                } else {
                                    (None, now)
                                }
                            }
                            None => (None, now),
                        };

                        *last = Some(new_last);
                        wait
                    };

                    if let Some(d) = sleep_duration {
                        tokio::time::sleep(d).await;
                    }
                }

                let timeout_duration = Duration::from_secs(config.timeout);
                match tokio::time::timeout(timeout_duration, engine.search(&query, &client, &config)).await {
                    Ok(result) => match result {
                        Ok(mut results) => {
                            // Success: Reset circuit breaker
                            let mut cb = circuit_breaker.lock().await;
                            cb.failures = 0;
                            cb.cooldown_until = None;

                            // Apply weight
                            for res in &mut results {
                                res.score *= config.weight;
                            }
                            results
                        }
                        Err(e) => {
                            tracing::error!("Engine {} failed: {}", id, e);

                            // Failure: Update circuit breaker
                            let mut cb = circuit_breaker.lock().await;
                            cb.failures += 1;
                            if cb.failures >= 3 {
                                cb.cooldown_until = Some(std::time::Instant::now() + Duration::from_secs(60));
                                tracing::warn!("Engine {} circuit broken ({} failures)", id, cb.failures);
                            }

                            vec![]
                        }
                    },
                    Err(_) => {
                        tracing::warn!("Engine {} timed out", id);

                        // Timeout: Treat as failure
                        let mut cb = circuit_breaker.lock().await;
                        cb.failures += 1;
                        if cb.failures >= 3 {
                            cb.cooldown_until = Some(std::time::Instant::now() + Duration::from_secs(60));
                            tracing::warn!("Engine {} circuit broken ({} failures - timeout)", id, cb.failures);
                        }

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
        fail: bool,
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
            _config: &EngineConfig,
        ) -> Result<Vec<SearchResult>, EngineError> {
            if self.fail {
                return Err(EngineError::Unexpected(anyhow::anyhow!("Simulated failure")));
            }
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
            fail: false,
        }));
        registry.register_engine(Box::new(MockEngine {
            id: "image_engine".to_string(),
            categories: vec!["images".to_string()],
            fail: false,
        }));

        let client = Client::new();

        let query_general = SearchQuery {
            q: "test".to_string(),
            ..Default::default()
        };
        let results = registry.search(&query_general, &client).await;
        assert!(results.iter().any(|r| r.engine == "general_engine"));
        assert!(!results.iter().any(|r| r.engine == "image_engine"));

        // 2. Test "images" category
        let query_images = SearchQuery {
            q: "test".to_string(),
            categories: "images".to_string(),
            ..Default::default()
        };
        let results = registry.search(&query_images, &client).await;
        assert!(!results.iter().any(|r| r.engine == "general_engine"));
        assert!(results.iter().any(|r| r.engine == "image_engine"));
    }

    #[tokio::test]
    async fn test_search_throttling() {
        let mut engines_config = HashMap::new();
        engines_config.insert(
            "throttled_engine".to_string(),
            EngineConfig {
                throttle: 500, // 500ms throttle
                ..Default::default()
            },
        );

        let settings = Arc::new(Settings {
             server: crate::config::ServerSettings {
                 bind_address: "127.0.0.1".into(),
                 port: 8080,
                 secret_key: "secret".into(),
             },
             debug: false,
             engines: engines_config,
        });

        let mut registry = EngineRegistry::new(settings);
        registry.register_engine(Box::new(MockEngine {
            id: "throttled_engine".to_string(),
            categories: vec!["general".to_string()],
            fail: false,
        }));

        let client = Client::new();
        let query = SearchQuery::default();

        let start = std::time::Instant::now();

        // First request should be immediate
        registry.search(&query, &client).await;
        let elapsed_first = start.elapsed();
        // Allow a bit of leeway for task spawning overhead
        assert!(elapsed_first < Duration::from_millis(200), "First request took too long: {:?}", elapsed_first);

        // Second request should be throttled
        let start_second = std::time::Instant::now();
        registry.search(&query, &client).await;
        let _elapsed_second = start_second.elapsed();

        let total_elapsed = start.elapsed();
        assert!(total_elapsed >= Duration::from_millis(500), "Total time {:?} should be >= 500ms", total_elapsed);
    }

    #[tokio::test]
    async fn test_circuit_breaker() {
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
            id: "failing_engine".to_string(),
            categories: vec!["general".to_string()],
            fail: true,
        }));

        let client = Client::new();
        let query = SearchQuery::default();

        // 3 failures to trip the breaker
        for _ in 0..3 {
            registry.search(&query, &client).await;
        }

        // Check if circuit breaker is tripped
        let entry = registry.engines.get("failing_engine").unwrap();
        {
            let cb = entry.circuit_breaker.lock().await;
            assert!(cb.cooldown_until.is_some(), "Circuit breaker should be tripped");
        }
    }
}
