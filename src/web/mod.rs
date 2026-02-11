pub mod error;
pub mod templates;

use arc_swap::ArcSwap;
use crate::config::Settings;
use crate::engines::registry::EngineRegistry;
use crate::models::SearchQuery;
use axum::{
    extract::{Path, Query, State},
    http::{header, StatusCode},
    response::{IntoResponse, Response},
    routing::get,
    Json, Router,
};
use error::{not_found_handler, WebError};
use rust_embed::RustEmbed;
use std::sync::Arc;

#[derive(Clone)]
pub struct AppState {
    pub settings: Arc<ArcSwap<Settings>>,
    pub registry: Arc<EngineRegistry>,
}

#[derive(RustEmbed)]
#[folder = "static/"]
struct StaticAssets;

pub fn router(state: AppState) -> Router {
    Router::new()
        .route("/", get(index))
        .route("/health", get(health_check))
        .route("/search", get(search))
        .route("/opensearch.xml", get(opensearch))
        .route("/static/*file", get(static_handler))
        .fallback(not_found_handler)
        .with_state(state)
}

async fn health_check() -> &'static str {
    "OK"
}

async fn index(State(state): State<AppState>) -> impl IntoResponse {
    let settings = state.settings.load();
    templates::IndexTemplate {
        instance_name: settings.general.instance_name.clone(),
    }
}

async fn opensearch(State(state): State<AppState>) -> impl IntoResponse {
    let settings = state.settings.load();
    let template = templates::OpenSearchTemplate {
        instance_name: settings.general.instance_name.clone(),
        base_url: settings.server.base_url.clone(),
    };
    ([(header::CONTENT_TYPE, "application/opensearchdescription+xml")], template).into_response()
}

async fn static_handler(Path(path): Path<String>) -> impl IntoResponse {
    match StaticAssets::get(&path) {
        Some(content) => {
            let mime = mime_guess::from_path(path).first_or_octet_stream();
            let etag = format!("\"{}\"", hex::encode(content.metadata.sha256_hash()));

            (
                [
                    (header::CONTENT_TYPE, mime.as_ref().to_string()),
                    (header::CACHE_CONTROL, "public, max-age=31536000".to_string()),
                    (header::ETAG, etag),
                ],
                content.data,
            )
                .into_response()
        }
        None => (StatusCode::NOT_FOUND, "Not Found").into_response(),
    }
}

async fn search(
    State(state): State<AppState>,
    Query(query): Query<SearchQuery>,
) -> Result<Response, WebError> {
    let results = state.registry.search(&query).await;

    let settings = state.settings.load();
    match query.format.as_str() {
        "json" => Ok(Json(results).into_response()),
        "rss" => {
            let template = templates::RssTemplate {
                query: query.q.clone(),
                results,
                instance_name: settings.general.instance_name.clone(),
                base_url: settings.server.base_url.clone(),
            };
            Ok(([(header::CONTENT_TYPE, "application/rss+xml")], template).into_response())
        }
        "atom" => {
            let template = templates::AtomTemplate {
                query: query.q.clone(),
                results,
                instance_name: settings.general.instance_name.clone(),
                base_url: settings.server.base_url.clone(),
            };
            Ok(([(header::CONTENT_TYPE, "application/atom+xml")], template).into_response())
        }
        _ => {
            let template = templates::ResultsTemplate {
                query: query.q.clone(),
                results,
                instance_name: settings.general.instance_name.clone(),
            };
            Ok(template.into_response())
        }
    }
}
