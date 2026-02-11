use crate::config::{EngineConfig, Settings};
use crate::engines::aggregator::aggregate;
use crate::engines::circuit_breaker::CircuitBreaker;
use crate::engines::SearchEngine;
use crate::models::{SearchQuery, SearchResult};
use reqwest::Client;
use std::collections::HashMap;
use std::sync::Arc;
use std::time::Duration;
use tokio::sync::Mutex;
use tokio::task::JoinSet;

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

        let circuit_breaker = Arc::new(Mutex::new(CircuitBreaker::new(
            config.failure_threshold,
            Duration::from_secs(config.cooldown),
        )));

        let entry = EngineEntry {
            engine: Arc::from(engine),
            categories,
            config,
            last_request: Arc::new(Mutex::new(None)),
            circuit_breaker,
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

            let engine = entry.engine.clone();
            let query = query.clone();
            let client = client.clone();
            let id = id.clone();
            let config = entry.config.clone();
            let last_request = entry.last_request.clone();
            let circuit_breaker = entry.circuit_breaker.clone();

            join_set.spawn(async move {
                // Circuit Breaker Check
                {
                    let mut cb = circuit_breaker.lock().await;
                    if !cb.check() {
                        tracing::warn!("Engine {} circuit breaker is open", id);
                        return vec![];
                    }
                }

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
                            circuit_breaker.lock().await.report_success();
                            // Apply weight and position decay
                            for (index, res) in results.iter_mut().enumerate() {
                                // Simple position decay: higher rank (lower index) gets more score
                                // Formula: weight / (index + 1)
                                res.score = config.weight / (index as f64 + 1.0);
                            }
                            results
                        }
                        Err(e) => {
                            circuit_breaker.lock().await.report_failure();
                            tracing::error!("Engine {} failed: {}", id, e);
                            vec![]
                        }
                    },
                    Err(_) => {
                        circuit_breaker.lock().await.report_failure();
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
        fail: bool,
        call_count: Arc<Mutex<u32>>,
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
            let mut count = self.call_count.lock().await;
            *count += 1;

            if self.fail {
                return Err(EngineError::Unexpected(anyhow::anyhow!("Mock failure")));
            }

            Ok(vec![SearchResult {
                url: format!("http://{}", self.id),
                title: self.id.clone(),
                content: ResultContent::Text("content".to_string()),
                engines: vec![self.id.clone()],
                score: 1.0,
                metadata: HashMap::new(),
            }])
        }
    }

    #[tokio::test]
    async fn test_search_category_filtering() {
        // Construct a dummy settings object manually
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
            call_count: Arc::new(Mutex::new(0)),
        }));
        registry.register_engine(Box::new(MockEngine {
            id: "image_engine".to_string(),
            categories: vec!["images".to_string()],
            fail: false,
            call_count: Arc::new(Mutex::new(0)),
        }));

        let client = Client::new();

        // 1. Test "general" category (default)
        let query_general = SearchQuery {
            q: "test".to_string(),
            ..Default::default()
        };
        let results = registry.search(&query_general, &client).await;
        assert!(results.iter().any(|r| r.engines.contains(&"general_engine".to_string())), "general_engine should match default category");
        assert!(!results.iter().any(|r| r.engines.contains(&"image_engine".to_string())), "image_engine should NOT match default category");

        // 2. Test "images" category
        let query_images = SearchQuery {
            q: "test".to_string(),
            categories: "images".to_string(),
            ..Default::default()
        };
        let results = registry.search(&query_images, &client).await;
        assert!(!results.iter().any(|r| r.engines.contains(&"general_engine".to_string())), "general_engine should NOT match images category");
        assert!(results.iter().any(|r| r.engines.contains(&"image_engine".to_string())), "image_engine should match images category");
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
            call_count: Arc::new(Mutex::new(0)),
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
    async fn test_circuit_breaker_integration() {
        let mut engines_config = HashMap::new();
        engines_config.insert(
            "failing_engine".to_string(),
            EngineConfig {
                failure_threshold: 2,
                cooldown: 1, // 1 second
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
        let call_count = Arc::new(Mutex::new(0));

        registry.register_engine(Box::new(MockEngine {
            id: "failing_engine".to_string(),
            categories: vec!["general".to_string()],
            fail: true,
            call_count: call_count.clone(),
        }));

        let client = Client::new();
        let query = SearchQuery::default();

        // 1. First failure
        registry.search(&query, &client).await;
        assert_eq!(*call_count.lock().await, 1);

        // 2. Second failure (threshold reached)
        registry.search(&query, &client).await;
        assert_eq!(*call_count.lock().await, 2);

        // 3. Third request - should be blocked by circuit breaker
        registry.search(&query, &client).await;
        assert_eq!(*call_count.lock().await, 2, "Should not call engine when circuit is open");

        // 4. Wait for cooldown (1.1s to be safe)
        tokio::time::sleep(Duration::from_millis(1100)).await;

        // 5. Fourth request - should be allowed (Half-Open)
        registry.search(&query, &client).await;
        assert_eq!(*call_count.lock().await, 3, "Should call engine after cooldown");
    }
}
