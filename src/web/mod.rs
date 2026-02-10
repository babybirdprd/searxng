use crate::config::Settings;
use axum::{routing::get, Router};
use std::sync::Arc;

pub fn router(settings: Arc<Settings>) -> Router {
    Router::new()
        .route("/health", get(health_check))
        .with_state(settings)
}

async fn health_check() -> &'static str {
    "OK"
}
