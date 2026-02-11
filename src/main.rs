use searxng_rs::config::Settings;
use searxng_rs::engines::duckduckgo::DuckDuckGo;
use searxng_rs::engines::dummy::DummyEngine;
use searxng_rs::engines::create_client;
use searxng_rs::engines::google::Google;
use searxng_rs::engines::registry::EngineRegistry;
use searxng_rs::web::AppState;
use searxng_rs::web;
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

    let client = create_client(
        "Mozilla/5.0 (compatible; SearXNG/1.0; +https://github.com/searxng/searxng)",
        None,
    )?;

    let mut registry = EngineRegistry::new(settings.clone(), client);
    registry.register_engine(Box::new(DummyEngine));
    registry.register_engine(Box::new(DuckDuckGo));
    registry.register_engine(Box::new(Google));
    let registry = Arc::new(registry);

    let state = AppState {
        settings: settings.clone(),
        registry,
    };

    let app = web::router(state);

    let addr = format!("{}:{}", settings.server.bind_address, settings.server.port);
    let listener = tokio::net::TcpListener::bind(&addr).await?;

    tracing::info!("listening on {}", addr);

    axum::serve(listener, app).await?;

    Ok(())
}
