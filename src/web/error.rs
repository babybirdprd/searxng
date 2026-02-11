use axum::{
    http::StatusCode,
    response::{IntoResponse, Response},
    Json,
};
use serde_json::json;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum WebError {
    #[error("Internal Server Error")]
    Internal(#[from] anyhow::Error),
    #[error("Not Found")]
    NotFound,
    #[error("Engine error: {0}")]
    Engine(#[from] crate::engines::error::EngineError),
}

impl IntoResponse for WebError {
    fn into_response(self) -> Response {
        let (status, message) = match self {
            WebError::Internal(ref e) => {
                tracing::error!("Internal server error: {:?}", e);
                (StatusCode::INTERNAL_SERVER_ERROR, self.to_string())
            }
            WebError::NotFound => (StatusCode::NOT_FOUND, self.to_string()),
            WebError::Engine(ref e) => {
                tracing::error!("Engine error: {:?}", e);
                (StatusCode::INTERNAL_SERVER_ERROR, self.to_string())
            }
        };

        let body = Json(json!({
            "error": message,
        }));

        (status, body).into_response()
    }
}

pub async fn not_found_handler() -> impl IntoResponse {
    WebError::NotFound
}
