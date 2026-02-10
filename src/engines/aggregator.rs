use crate::models::SearchResult;
use std::collections::HashMap;

/// Aggregates search results from multiple engines.
///
/// It performs the following operations:
/// 1. Deduplicates results based on URL (keeping the one with the highest score).
/// 2. Sorts results by score in descending order.
pub fn aggregate(results: Vec<SearchResult>) -> Vec<SearchResult> {
    let mut unique_results: HashMap<String, SearchResult> = HashMap::new();

    for res in results {
        match unique_results.get_mut(&res.url) {
            Some(existing) => {
                // If duplicates found, keep the one with higher score.
                // We could also sum scores or boost the score if multiple engines return it.
                // For now, let's just take the max.
                if res.score > existing.score {
                    *existing = res;
                }
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
