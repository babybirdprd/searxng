use arc_swap::ArcSwap;
use notify::{RecursiveMode, Watcher};
use searxng_rs::config::Settings;
use searxng_rs::engines::bing::Bing;
use searxng_rs::engines::{create_client, DEFAULT_USER_AGENT};
use searxng_rs::engines::duckduckgo::DuckDuckGo;
use searxng_rs::engines::dummy::DummyEngine;
use searxng_rs::engines::google::Google;
use searxng_rs::engines::qwant::Qwant;
use searxng_rs::engines::reddit::Reddit;
use searxng_rs::engines::registry::EngineRegistry;
use searxng_rs::engines::wikipedia::Wikipedia;
use searxng_rs::web;
use searxng_rs::web::AppState;
use std::sync::Arc;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let run_mode = std::env::var("RUN_MODE").unwrap_or_else(|_| "development".into());

    let env_filter = tracing_subscriber::EnvFilter::try_from_default_env()
        .unwrap_or_else(|_| "searxng_rs=debug,tower_http=debug".into());

    if run_mode == "development" {
        tracing_subscriber::registry()
            .with(env_filter)
            .with(tracing_subscriber::fmt::layer())
            .init();
    } else {
        tracing_subscriber::registry()
            .with(env_filter)
            .with(tracing_subscriber::fmt::layer().json())
            .init();
    }

    let settings = Arc::new(ArcSwap::from(Arc::new(Settings::new()?)));

    // Setup hot reloading
    let settings_clone = settings.clone();
    let mut watcher = notify::recommended_watcher(move |res: notify::Result<notify::Event>| {
        if let Ok(event) = res {
            if event.kind.is_modify() || event.kind.is_create() {
                tracing::info!("Config file changed, reloading...");
                match Settings::new() {
                    Ok(new_settings) => {
                        settings_clone.store(Arc::new(new_settings));
                        tracing::info!("Config reloaded successfully");
                    }
                    Err(e) => tracing::error!("Failed to reload config: {}", e),
                }
            }
        }
    })?;

    watcher.watch(std::path::Path::new("."), RecursiveMode::NonRecursive)?;

    let client = create_client(DEFAULT_USER_AGENT, None)?;

    let mut registry = EngineRegistry::new(settings.clone(), client);
    registry.register_engine(Box::new(DummyEngine));
    registry.register_engine(Box::new(DuckDuckGo));
    registry.register_engine(Box::new(Google));
    registry.register_engine(Box::new(Bing));
    registry.register_engine(Box::new(Wikipedia));
    registry.register_engine(Box::new(Reddit));
    registry.register_engine(Box::new(Qwant));
    let registry = Arc::new(registry);

    let state = AppState {
        settings: settings.clone(),
        registry,
    };

    let app = web::router(state);

    let current_settings = settings.load();
    let addr = format!(
        "{}:{}",
        current_settings.server.bind_address, current_settings.server.port
    );
    let listener = tokio::net::TcpListener::bind(&addr).await?;

    tracing::info!("listening on {}", addr);

    axum::serve(listener, app).await?;

    Ok(())
}
