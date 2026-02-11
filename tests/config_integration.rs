use searxng_rs::config::Settings;
use std::env;

#[test]
fn test_env_var_override() {
    // Set environment variable to override dummy engine weight
    // Note: The config crate uses double underscore as separator for nested keys
    // SEARXNG__ENGINES__DUMMY__WEIGHT corresponds to engines.dummy.weight
    unsafe {
        env::set_var("SEARXNG__ENGINES__DUMMY__WEIGHT", "42.0");
        env::set_var("SEARXNG__ENGINES__DUMMY__TIMEOUT", "10");
    }

    let settings = Settings::new().expect("Failed to load settings");

    // Verify overrides
    let dummy_config = settings.engines.get("dummy");

    assert!(dummy_config.is_some(), "Dummy engine config should exist");
    let config = dummy_config.unwrap();

    assert_eq!(config.weight, 42.0);
    assert_eq!(config.timeout, 10);

    unsafe {
        env::remove_var("SEARXNG__ENGINES__DUMMY__WEIGHT");
        env::remove_var("SEARXNG__ENGINES__DUMMY__TIMEOUT");
    }
}
