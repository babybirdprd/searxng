use crate::models::SearchResult;
use std::collections::HashMap;

/// Aggregates search results from multiple engines.
///
/// It performs the following operations:
/// 1. Deduplicates results based on URL.
/// 2. Boosts the score if the same result (by URL) is returned by multiple engines.
/// 3. Sorts results by score in descending order.
pub fn aggregate(results: Vec<SearchResult>) -> Vec<SearchResult> {
    let mut unique_results: HashMap<String, SearchResult> = HashMap::new();

    for res in results {
        match unique_results.get_mut(&res.url) {
            Some(existing) => {
                // If duplicates found, boost the score.
                // We add a fraction of the new result's score to the existing one.
                // This rewards results that appear in multiple engines.
                // The boosting factor is arbitrary for now, let's say 10% of the new score.
                existing.score += res.score * 0.1;
            }
            None => {
                unique_results.insert(res.url.clone(), res);
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
    use crate::models::SearchResult;

    #[test]
    fn test_aggregate_boosts_score() {
        let res1 = SearchResult {
            url: "https://example.com".to_string(),
            title: "Example".to_string(),
            content: "Content".to_string(),
            engine: "engine1".to_string(),
            score: 1.0,
        };
        let res2 = SearchResult {
            url: "https://example.com".to_string(),
            title: "Example".to_string(),
            content: "Content".to_string(),
            engine: "engine2".to_string(),
            score: 1.0,
        };
        let res3 = SearchResult {
            url: "https://other.com".to_string(),
            title: "Other".to_string(),
            content: "Other Content".to_string(),
            engine: "engine1".to_string(),
            score: 0.5,
        };

        let results = vec![res1, res2, res3];
        let aggregated = aggregate(results);

        assert_eq!(aggregated.len(), 2);

        let example_res = aggregated.iter().find(|r| r.url == "https://example.com").unwrap();
        // Initial score 1.0 + boost (1.0 * 0.1) = 1.1
        assert!((example_res.score - 1.1).abs() < f64::EPSILON);

        let other_res = aggregated.iter().find(|r| r.url == "https://other.com").unwrap();
        assert!((other_res.score - 0.5).abs() < f64::EPSILON);
    }
}
