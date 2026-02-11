use crate::models::SearchResult;
use std::collections::HashMap;
use url::Url;

/// Normalizes a URL by removing common tracking parameters.
fn normalize_url(url: &str) -> String {
    match Url::parse(url) {
        Ok(mut u) => {
            // Remove common tracking parameters
            // We need to collect first to iterate, then clear and rebuild
            let query_params: Vec<(String, String)> = u.query_pairs().into_owned().collect();

            {
                let mut pairs = u.query_pairs_mut();
                pairs.clear();
                for (key, value) in query_params {
                    if !key.starts_with("utm_")
                       && key != "fbclid"
                       && key != "gclid"
                       && key != "ref"
                       && key != "yclid"
                       && key != "_hsenc"
                       && key != "_hsmi"
                       && key != "mc_cid"
                       && key != "mc_eid" {
                        pairs.append_pair(&key, &value);
                    }
                }
            }

            // If query is empty, remove the '?'
            if u.query() == Some("") {
                u.set_query(None);
            }

            // Remove fragment (anchor) as it usually points to same page content
            u.set_fragment(None);

            u.to_string()
        },
        Err(_) => url.to_string()
    }
}

/// Aggregates search results from multiple engines.
///
/// It performs the following operations:
/// 1. Deduplicates results based on URL.
/// 2. Boosts the score if the same result (by URL) is returned by multiple engines.
/// 3. Sorts results by score in descending order.
pub fn aggregate(results: Vec<SearchResult>) -> Vec<SearchResult> {
    let mut unique_results: HashMap<String, SearchResult> = HashMap::new();

    for res in results {
        let normalized = normalize_url(&res.url);

        match unique_results.get_mut(&normalized) {
            Some(existing) => {
                // If duplicates found, boost the score.
                // We add a fraction of the new result's score to the existing one.
                // This rewards results that appear in multiple engines.
                // The boosting factor is arbitrary for now, let's say 10% of the new score.
                existing.score += res.score * 0.1;
            }
            None => {
                unique_results.insert(normalized, res);
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
        assert_eq!(
            normalize_url("https://example.com/?utm_source=google&q=test"),
            "https://example.com/?q=test"
        );
        assert_eq!(
            normalize_url("https://example.com/#anchor"),
            "https://example.com/"
        );
         assert_eq!(
            normalize_url("https://example.com/?fbclid=123"),
            "https://example.com/"
        );
    }

    #[test]
    fn test_aggregate_deduplication() {
        let res1 = SearchResult {
            url: "https://example.com/?utm_source=twitter".to_string(),
            title: "Example".to_string(),
            content: ResultContent::Text("Content".to_string()),
            engine: "engine1".to_string(),
            score: 1.0,
            metadata: HashMap::new(),
        };
        let res2 = SearchResult {
            url: "https://example.com/".to_string(),
            title: "Example".to_string(),
            content: ResultContent::Text("Content".to_string()),
            engine: "engine2".to_string(),
            score: 1.0,
            metadata: HashMap::new(),
        };

        let results = vec![res1, res2];
        let aggregated = aggregate(results);

        assert_eq!(aggregated.len(), 1);
        assert!(aggregated[0].score > 1.0);
    }

    #[test]
    fn test_aggregate_boosts_score() {
        let res1 = SearchResult {
            url: "https://example.com".to_string(),
            title: "Example".to_string(),
            content: ResultContent::Text("Content".to_string()),
            engine: "engine1".to_string(),
            score: 1.0,
            metadata: HashMap::new(),
        };
        let res2 = SearchResult {
            url: "https://example.com".to_string(),
            title: "Example".to_string(),
            content: ResultContent::Text("Content".to_string()),
            engine: "engine2".to_string(),
            score: 1.0,
            metadata: HashMap::new(),
        };
        let res3 = SearchResult {
            url: "https://other.com".to_string(),
            title: "Other".to_string(),
            content: ResultContent::Text("Other Content".to_string()),
            engine: "engine1".to_string(),
            score: 0.5,
            metadata: HashMap::new(),
        };

        let results = vec![res1, res2, res3];
        let aggregated = aggregate(results);

        assert_eq!(aggregated.len(), 2);

        let example_res = aggregated
            .iter()
            .find(|r| r.url == "https://example.com")
            .unwrap();
        // Initial score 1.0 + boost (1.0 * 0.1) = 1.1
        assert!((example_res.score - 1.1).abs() < f64::EPSILON);
    }
}
