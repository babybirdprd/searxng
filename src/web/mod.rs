use crate::config::Settings;
use crate::engines::registry::EngineRegistry;
use crate::models::{SearchQuery, SearchResult};
use axum::{
    extract::{Query, State},
    routing::get,
    Json, Router,
};
use std::sync::Arc;

#[derive(Clone)]
pub struct AppState {
    pub settings: Arc<Settings>,
    pub registry: Arc<EngineRegistry>,
}

pub fn router(state: AppState) -> Router {
    Router::new()
        .route("/health", get(health_check))
        .route("/search", get(search))
        .with_state(state)
}

async fn health_check() -> &'static str {
    "OK"
}

async fn search(
    State(state): State<AppState>,
    Query(query): Query<SearchQuery>,
) -> Json<Vec<SearchResult>> {
    let results = state.registry.search(&query).await;
    Json(results)
}
