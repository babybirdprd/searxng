# SearXNG Rust Port Roadmap

This roadmap outlines the comprehensive plan to port SearXNG to a fully idiomatic, high-performance Rust application. The goal is to eliminate reliance on the legacy Python codebase, resulting in a single binary that is easier to deploy, maintain, and scale.

## Vision

-   **Single Binary**: All components (server, engines, assets) compiled into one executable using `rust-embed`.
-   **High Performance**: Leverage Rust's async runtime (`tokio`) and zero-cost abstractions to handle high concurrency with minimal overhead.
-   **Safety**: enforce strict type safety and memory safety to prevent common vulnerabilities.
-   **Maintainability**: Modular architecture with clear separation of concerns (Engines, Aggregator, Web, Config).

## Phase 1: Core Architecture & Foundation (Current Status: In Progress)

-   [x] **Async Runtime**: Establish `tokio` as the async runtime.
-   [x] **Web Framework**: Use `axum` for high-performance HTTP handling.
-   [x] **Engine Trait**: Define `SearchEngine` trait for standardizing engine implementation.
-   [x] **Engine Registry**: Implement `EngineRegistry` for managing and executing engines concurrently.
-   [ ] **Configuration System**:
    -   [x] Basic `Settings` struct using `config` crate.
    -   [x] **Granular Engine Config**: Support per-engine settings (weight, timeout, throttle, tokens).
    -   [x] **Environment Overrides**: Fully support `SEARXNG__` env vars for all settings.
    -   [ ] **Hot Reloading**: watch config file for changes (optional).
-   [ ] **Error Handling**:
    -   [x] Basic `EngineError` using `thiserror`.
    -   [ ] **Structured Logging**: Implement `tracing` with `tracing-subscriber` (JSON output for prod).
    -   [ ] **Global Error Handling**: `axum` error handlers for graceful 500/404 responses.

## Phase 2: Engine Expansion & Robustness

-   [ ] **Engine Execution**:
        -   [x] **Throttling**: Implement per-engine rate limiting (token bucket or simple sleep) to respect `throttle` config.
        -   [x] **Circuit Breakers**: temporarily disable engines that consistently fail or time out.
    -   [ ] **Proxy Support**: specific proxy configuration per engine (`reqwest` proxy support).
-   [ ] **Engine Implementations**:
    -   [x] **DuckDuckGo**: Basic HTML scraping.
    -   [ ] **Google**: Basic implementation (needs expansion).
    -   [ ] **Bing**: HTML scraping or API.
    -   [ ] **Wikipedia**: API integration.
    -   [ ] **Reddit**: JSON API integration.
    -   [ ] **General**: Port remaining general engines.
    -   [ ] **Images/Videos**: Add support for `ResultContent::Image` and `ResultContent::Video`.
-   [ ] **Engine Features**:
    -   [x] **Category Filtering**: Engines should only run if they match the query category.
    -   [x] **Language Support**: Pass language codes to engines (e.g., `lang=en-US`).
    -   [ ] **Safe Search**: Implement safe search filtering at the engine level.
    -   [ ] **Paging**: Support `page` parameter in `SearchEngine::search`.

## Phase 3: Aggregation & Ranking

-   [ ] **Result Aggregation**:
    -   [x] Basic deduplication by URL.
    -   [x] **Normalization**: Canonicalize URLs before deduplication (strip tracking params).
    -   [x] **Scoring Algorithm**: Implement a weighted scoring system:
        -   Engine weight (configured).
        -   Result position (higher rank = higher score).
        -   Frequency (more engines = higher boost).
    -   [x] **Mixed Content**: robustly handle merging text, image, and map results.
-   [ ] **Filtering & Sanitization**:
    -   [ ] **Host Blocking**: Filter results from blocked domains (configurable blacklist).
    -   [ ] **HTML Sanitization**: Ensure result snippets are safe to render (use `ammonia`).

## Phase 4: Web Interface & User Experience

-   [ ] **Templating**:
    -   [ ] Integrate `askama` for type-safe, compiled HTML templates.
    -   [ ] Port existing Jinja2 templates to `askama`.
    -   [ ] **Themes**: Support multiple themes (simple, oscar).
-   [ ] **Static Assets**:
    -   [ ] Embed static assets (CSS, JS, images, fonts) into the binary using `rust-embed`.
    -   [ ] Serve static assets with proper caching headers (`Cache-Control`, `ETag`).
-   [ ] **API**:
    -   [x] Basic JSON API.
    -   [ ] **RSS/Atom**: generating feeds for search results.
    -   [ ] **OpenSearch**: Support OpenSearch description document.
-   [ ] **Localization (i18n)**:
    -   [ ] Use `fluent` or `gettext` for translating UI strings.
    -   [ ] Auto-detect user language from headers.

## Phase 5: Advanced Features

-   [ ] **Caching**:
    -   [ ] **In-Memory Cache**: `moka` or `lru` crate for caching search results (TTL based).
    -   [ ] **Redis Cache**: Optional Redis backend for distributed caching.
-   [ ] **Plugins/Middleware**:
    -   [ ] **Query Plugins**: Modify query before search (e.g., bang commands `!g`).
    -   [ ] **Result Plugins**: Modify results after aggregation.
-   [ ] **Metrics & Monitoring**:
    -   [ ] **Prometheus**: Expose metrics via `/metrics` (request count, latency, engine health).
    -   [ ] **Health Check**: Detailed health status for k8s probes.

## Phase 6: Testing & Quality Assurance

-   [ ] **Unit Tests**: comprehensive unit tests for all modules (`engines`, `aggregator`, `config`).
-   [ ] **Integration Tests**: End-to-end tests spinning up the `axum` server and querying mock engines.
-   [ ] **Property-Based Testing**: Use `proptest` for fuzzing inputs.
-   [ ] **Benchmarks**: Use `criterion` to measure performance critical paths (aggregation, parsing).

## Phase 7: Deployment & Maintenance

-   [ ] **Docker**: Create a minimal Docker image (scratch or distroless).
-   [ ] **CI/CD**: GitHub Actions for testing, linting (`clippy`), and building releases.
-   [ ] **Documentation**:
    -   [ ] Developer guide (architecture, contributing).
    -   [ ] User guide (configuration, deployment).
