pub mod error;

use arc_swap::ArcSwap;
use crate::config::Settings;
use crate::engines::registry::EngineRegistry;
use crate::models::{SearchQuery, SearchResult};
use axum::{
    extract::{Query, State},
    routing::get,
    Json, Router,
};
use error::{not_found_handler, WebError};
use std::sync::Arc;

#[derive(Clone)]
pub struct AppState {
    pub settings: Arc<ArcSwap<Settings>>,
    pub registry: Arc<EngineRegistry>,
}

pub fn router(state: AppState) -> Router {
    Router::new()
        .route("/health", get(health_check))
        .route("/search", get(search))
        .fallback(not_found_handler)
        .with_state(state)
}

async fn health_check() -> &'static str {
    "OK"
}

async fn search(
    State(state): State<AppState>,
    Query(query): Query<SearchQuery>,
) -> Result<Json<Vec<SearchResult>>, WebError> {
    let results = state.registry.search(&query).await;
    Ok(Json(results))
}
