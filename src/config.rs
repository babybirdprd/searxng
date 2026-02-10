use config::{Config, ConfigError, Environment, File};
use serde::Deserialize;
use std::env;

#[derive(Debug, Deserialize, Clone)]
pub struct ServerSettings {
    pub bind_address: String,
    pub port: u16,
    pub secret_key: String,
}

#[derive(Debug, Deserialize, Clone)]
pub struct Settings {
    pub server: ServerSettings,
    pub debug: bool,
}

impl Settings {
    pub fn new() -> Result<Self, ConfigError> {
        let run_mode = env::var("RUN_MODE").unwrap_or_else(|_| "development".into());

        let s = Config::builder()
            // Start with default values
            .set_default("debug", false)?
            .set_default("server.bind_address", "127.0.0.1")?
            .set_default("server.port", 8080)?
            .set_default("server.secret_key", "changeme")?
            // Merge with config file (if exists)
            .add_source(File::with_name("settings").required(false))
            .add_source(File::with_name(&format!("settings.{}", run_mode)).required(false))
            // Merge with environment variables (e.g. SEARXNG_SERVER__PORT=8080)
            .add_source(Environment::with_prefix("SEARXNG").separator("__"))
            .build()?;

        s.try_deserialize()
    }
}
