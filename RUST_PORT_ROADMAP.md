# SearXNG Rust Port Roadmap

This roadmap outlines the plan to port SearXNG to a fully idiomatic, high-performance Rust application. The goal is to eliminate reliance on the Python codebase, resulting in a single binary that is easier to deploy and maintain.

## Vision

-   **Single Binary**: All components (server, engines, assets) compiled into one executable.
-   **High Performance**: Leverage Rust's async runtime (Tokio) and zero-cost abstractions.
-   **Safety**: Use Rust's type system to prevent common errors (null pointer, data races).
-   **Maintainability**: Modular architecture with clear separation of concerns.

## Phase 1: Core Architecture & Foundation (Current Status: In Progress)

-   [x] **Async Runtime**: Establish Tokio as the async runtime.
-   [x] **Web Framework**: Use Axum for high-performance HTTP handling.
-   [x] **Engine Trait**: Define `SearchEngine` trait for standardizing engine implementation.
-   [x] **Engine Registry**: Implement `EngineRegistry` for managing and executing engines concurrently.
-   [ ] **Configuration**:
    -   [x] Basic `Settings` struct using `config` crate.
    -   [ ] Support for comprehensive YAML/TOML configuration files.
    -   [ ] Environment variable overrides for all settings.
    -   [ ] Hot-reloading of configuration (optional).
-   [ ] **Error Handling**:
    -   [x] Basic `EngineError` using `thiserror`.
    -   [ ] comprehensive error types for all subsystems (Config, Web, Aggregator).
    -   [ ] Structured logging with `tracing`.

## Phase 2: Engine Expansion & Robustness

-   [ ] **Engine Features**:
    -   [x] **Category Filtering**: Engines should only run if they match the query category.
    -   [ ] **Time Range**: Support for filtering results by time (day, week, month, year).
    -   [ ] **Language Support**: Pass language codes to engines.
    -   [ ] **Safe Search**: Implement safe search filtering at the engine level where supported.
    -   [ ] **Paging**: Support pagination in engines.
-   [ ] **Engine Implementations**:
    -   [ ] Port major general engines: Bing, Yahoo, Startpage.
    -   [ ] Port specialized engines: Wikipedia, Reddit, GitHub, StackOverflow.
    -   [ ] Port image/video engines: Google Images, Bing Images, YouTube.
    -   [ ] Implement "meta" engines (e.g., aggregating other aggregators).
-   [ ] **Resilience**:
    -   [ ] **Rate Limiting**: Handle 429 errors gracefully with backoff and circuit breakers.
    -   [ ] **Proxy Support**: specific proxy configuration per engine or global.
    -   [ ] **CAPTCHA Handling**: Detect CAPTCHAs and potentially solve or skip.

## Phase 3: Aggregation & Ranking

-   [ ] **Result Aggregation**:
    -   [x] Basic deduplication by URL.
    -   [ ] **Advanced Deduplication**: Normalize URLs, handle tracking parameters.
    -   [ ] **Scoring**: Implement a sophisticated scoring algorithm based on engine weights, position, and frequency.
    -   [ ] **Mixed Content**: Handle merging results of different types (text, image, video).
-   [ ] **Filtering**:
    -   [ ] **Host Blocking**: Filter results from blocked domains (e.g., spam, ads).
    -   [ ] **Keyword Blocking**: Filter results containing blocked keywords.

## Phase 4: Web Interface & User Experience

-   [ ] **Templating**:
    -   [ ] Integrate `askama` or `tera` for HTML rendering.
    -   [ ] Port existing Jinja2 templates to the chosen Rust template engine.
    -   [ ] Support for themes (simple, oscar, etc.).
-   [ ] **Static Assets**:
    -   [ ] Embed static assets (CSS, JS, images) into the binary using `rust-embed`.
    -   [ ] Serve static assets efficiently with caching headers.
-   [ ] **API**:
    -   [x] Basic JSON API.
    -   [ ] RSS/Atom feed support.
    -   [ ] CSV/Opensearch support.
-   [ ] **Localization**:
    -   [ ] Support internationalization (i18n) for UI text.
    -   [ ] Auto-detect user language.

## Phase 5: Advanced Features

-   [ ] **Caching**:
    -   [ ] In-memory caching (LRU) for frequent queries.
    -   [ ] Redis support for distributed caching.
-   [ ] **Plugins/Middleware**:
    -   [ ] Design a plugin system for extending functionality (e.g., query modification, result post-processing).
-   [ ] **Metrics & Monitoring**:
    -   [ ] Expose Prometheus metrics (request count, latency, engine health).
    -   [ ] Health check endpoint with detailed status.

## Phase 6: Testing & Quality Assurance

-   [ ] **Unit Tests**: comprehensive unit tests for all modules.
-   [ ] **Integration Tests**: End-to-end tests spinning up the server and querying mock engines.
-   [ ] **Robot Tests**: Port existing robot framework tests or replace with Rust-native integration tests.
-   [ ] **Benchmarks**: Measure performance and memory usage to ensure improvements over Python version.

## Phase 7: Deployment & Maintenance

-   [ ] **Docker**: Create a minimal Docker image (scratch or distroless).
-   [ ] **CI/CD**: Set up GitHub Actions for building, testing, and releasing.
-   [ ] **Documentation**: Write user and developer documentation.
