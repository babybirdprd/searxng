mod config;
mod engines;
mod models;
mod web;

use crate::config::Settings;
use std::sync::Arc;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    tracing_subscriber::registry()
        .with(
            tracing_subscriber::EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| "searxng_rs=debug,tower_http=debug".into()),
        )
        .with(tracing_subscriber::fmt::layer())
        .init();

    let settings = Settings::new()?;
    let settings = Arc::new(settings);

    let app = web::router(settings.clone());

    let addr = format!("{}:{}", settings.server.bind_address, settings.server.port);
    let listener = tokio::net::TcpListener::bind(&addr).await?;

    tracing::info!("listening on {}", addr);

    axum::serve(listener, app).await?;

    Ok(())
}
