use crate::models::{ResultContent, SearchResult};
use std::collections::HashMap;
use url::Url;

/// Normalizes a URL by:
/// 1. Lowercasing the scheme and host.
/// 2. Removing fragments.
/// 3. Removing common tracking parameters.
fn normalize_url(url_str: &str) -> String {
    match Url::parse(url_str) {
        Ok(mut url) => {
            // Remove fragment
            url.set_fragment(None);

            // Remove tracking parameters
            let params_to_remove = [
                "utm_source",
                "utm_medium",
                "utm_campaign",
                "utm_term",
                "utm_content",
                "fbclid",
                "gclid",
                "msclkid",
            ];

            let pairs: Vec<(String, String)> = url
                .query_pairs()
                .filter(|(k, _)| !params_to_remove.contains(&k.as_ref()))
                .map(|(k, v)| (k.into_owned(), v.into_owned()))
                .collect();

            if pairs.is_empty() {
                url.set_query(None);
            } else {
                url.query_pairs_mut().clear().extend_pairs(pairs);
            }

            url.to_string()
        }
        Err(_) => url_str.to_string(), // Return original if parsing fails
    }
}

/// Aggregates search results from multiple engines.
///
/// It performs the following operations:
/// 1. Filters results based on the blocklist.
/// 2. Deduplicates results based on normalized URL.
/// 3. Merges results:
///    - Sums up scores (frequency boost).
///    - Combines engine lists.
/// 4. Sorts results by score in descending order.
pub fn aggregate(results: Vec<SearchResult>, blocklist: &[String]) -> Vec<SearchResult> {
    let mut unique_results: HashMap<String, SearchResult> = HashMap::new();

    for mut res in results {
        // Host Blocking
        if let Ok(url) = Url::parse(&res.url) {
            if let Some(host) = url.host_str() {
                if blocklist.iter().any(|blocked| host.contains(blocked)) {
                    continue;
                }
            }
        }

        // HTML Sanitization
        if let ResultContent::Text(ref text) = res.content {
            res.content = ResultContent::Text(ammonia::clean(text));
        }

        let normalized_url = normalize_url(&res.url);

        match unique_results.get_mut(&normalized_url) {
            Some(existing) => {
                // Merge scores: Sum them up.
                // This assumes scores already include weight and position decay.
                // Summing them boosts results found by multiple engines (Frequency).
                existing.score += res.score;

                // Merge engines
                for engine in res.engines {
                    if !existing.engines.contains(&engine) {
                        existing.engines.push(engine);
                    }
                }
            }
            None => {
                // Use normalized URL for the result too?
                // Maybe keep original URL but use normalized for key.
                // Let's keep original URL for display, or maybe normalized is better?
                // Roadmap says "Canonicalize URLs before deduplication".
                // Usually we want the cleanest URL.
                res.url = normalized_url.clone();
                unique_results.insert(normalized_url, res);
            }
        }
    }

    let mut final_results: Vec<SearchResult> = unique_results.into_values().collect();

    // Sort by score descending
    final_results.sort_by(|a, b| {
        b.score
            .partial_cmp(&a.score)
            .unwrap_or(std::cmp::Ordering::Equal)
    });

    final_results
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::models::{ResultContent, SearchResult};
    use std::collections::HashMap;

    #[test]
    fn test_normalize_url() {
        let url = "https://Example.com/Path?utm_source=google&q=test#fragment";
        let normalized = normalize_url(url);
        assert_eq!(normalized, "https://example.com/Path?q=test");

        let url_simple = "https://example.com";
        assert_eq!(normalize_url(url_simple), "https://example.com/");
    }

    #[test]
    fn test_aggregate_merges_and_boosts() {
        let res1 = SearchResult {
            url: "https://example.com?utm_source=foo".to_string(),
            title: "Example".to_string(),
            content: ResultContent::Text("Content".to_string()),
            engines: vec!["engine1".to_string()],
            score: 1.0,
            metadata: HashMap::new(),
        };
        let res2 = SearchResult {
            url: "https://example.com".to_string(),
            title: "Example".to_string(),
            content: ResultContent::Text("Content".to_string()),
            engines: vec!["engine2".to_string()],
            score: 0.8,
            metadata: HashMap::new(),
        };
        let res3 = SearchResult {
            url: "https://other.com".to_string(),
            title: "Other".to_string(),
            content: ResultContent::Text("Other Content".to_string()),
            engines: vec!["engine1".to_string()],
            score: 0.5,
            metadata: HashMap::new(),
        };

        let results = vec![res1, res2, res3];
        let aggregated = aggregate(results, &[]);

        assert_eq!(aggregated.len(), 2);

        // Find example.com result
        let example_res = aggregated
            .iter()
            .find(|r| r.url.contains("example.com"))
            .unwrap();

        // Check normalized URL
        assert_eq!(example_res.url, "https://example.com/");

        // Score should be sum: 1.0 + 0.8 = 1.8
        assert!((example_res.score - 1.8).abs() < f64::EPSILON);

        // Engines should contain both
        assert!(example_res.engines.contains(&"engine1".to_string()));
        assert!(example_res.engines.contains(&"engine2".to_string()));

        let other_res = aggregated
            .iter()
            .find(|r| r.url.contains("other.com"))
            .unwrap();
        assert!((other_res.score - 0.5).abs() < f64::EPSILON);
    }

    #[test]
    fn test_aggregate_sanitization() {
        let res = SearchResult {
            url: "https://example.com".to_string(),
            title: "Example".to_string(),
            content: ResultContent::Text("<script>alert('xss')</script>Safe content".to_string()),
            engines: vec!["engine1".to_string()],
            score: 1.0,
            metadata: HashMap::new(),
        };

        let aggregated = aggregate(vec![res], &[]);
        if let ResultContent::Text(ref text) = aggregated[0].content {
            assert!(!text.contains("<script>"));
            assert!(text.contains("Safe content"));
        } else {
            panic!("Wrong content type");
        }
    }

    #[test]
    fn test_aggregate_host_blocking() {
        let res1 = SearchResult {
            url: "https://blocked.com/path".to_string(),
            title: "Blocked".to_string(),
            content: ResultContent::Text("Content".to_string()),
            engines: vec!["engine1".to_string()],
            score: 1.0,
            metadata: HashMap::new(),
        };
        let res2 = SearchResult {
            url: "https://allowed.com/path".to_string(),
            title: "Allowed".to_string(),
            content: ResultContent::Text("Content".to_string()),
            engines: vec!["engine1".to_string()],
            score: 1.0,
            metadata: HashMap::new(),
        };

        let blocklist = vec!["blocked.com".to_string()];
        let results = vec![res1, res2];
        let aggregated = aggregate(results, &blocklist);

        assert_eq!(aggregated.len(), 1);
        assert_eq!(aggregated[0].url, "https://allowed.com/path");
    }
}
