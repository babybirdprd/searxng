# SearXNG Rust Port Roadmap

This roadmap outlines the comprehensive plan to port SearXNG to a fully idiomatic, high-performance Rust application. The ultimate goal is to eliminate all reliance on the legacy Python codebase, resulting in a **single binary** that is easier to deploy, maintain, and scale.

## Vision

-   **Single Binary**: All components (server, engines, templates, static assets) compiled into one executable using `rust-embed`.
-   **Zero Python**: Complete removal of Python dependencies. The application will be self-contained.
-   **High Performance**: Leverage Rust's async runtime (`tokio`) and zero-cost abstractions to handle high concurrency with minimal overhead.
-   **Safety**: Enforce strict type safety and memory safety to prevent common vulnerabilities.
-   **Maintainability**: Modular architecture with clear separation of concerns (Engines, Aggregator, Web, Config).

---

## Phase 1: Core Architecture (Complete)

-   [x] **Async Runtime**: Establish `tokio` as the async runtime.
-   [x] **Web Framework**: Use `axum` for high-performance HTTP handling.
-   [x] **Engine Trait**: Define `SearchEngine` trait for standardizing engine implementation.
-   [x] **Engine Registry**: Implement `EngineRegistry` for managing and executing engines concurrently.

## Phase 2: Robustness & Reliability (Current Focus)

-   [ ] **Circuit Breaker Pattern**:
    -   Implement a mechanism to temporarily disable engines that consistently fail or time out.
    -   Track failure counts and enforce cooldown periods.
-   [ ] **Advanced Throttling**:
    -   Refine per-engine rate limiting to prevent IP bans.
    -   Implement global and per-instance concurrency limits.
-   [ ] **Error Handling**:
    -   Expand `EngineError` to distinguish between transient (network) and permanent (parsing) errors.
    -   Implement structured logging with `tracing` (JSON output).

## Phase 3: Aggregation & Ranking (Current Focus)

-   [ ] **Result Aggregation**:
    -   [ ] **URL Normalization**: Canonicalize URLs before deduplication (strip `utm_*`, `fbclid`, etc.).
    -   [ ] **Deduplication**: Robustly merge results pointing to the same resource.
-   [ ] **Scoring Algorithm**:
    -   Implement a weighted scoring system: `Score = (EngineWeight * PositionScore) + DuplicateBoost`.
    -   Configurable weights per engine.

## Phase 4: Configuration & Environment

-   [ ] **Granular Configuration**:
    -   Support per-engine configuration (enabled, weight, timeout, throttle).
    -   Full environment variable support (e.g., `SEARXNG_ENGINES__GOOGLE__ENABLED=false`).
-   [ ] **Hot Reloading**: Watch `settings.yml` for changes and reload without restart.

## Phase 5: Engine Implementation & Expansion

-   [ ] **General Engines**:
    -   [ ] Google (improve scraping/API).
    -   [ ] Bing.
    -   [ ] Wikipedia (API).
    -   [ ] Reddit (API).
-   [ ] **Specialized Engines**:
    -   [ ] Images (Bing Images, Google Images).
    -   [ ] Videos (YouTube, Vimeo).
    -   [ ] News.
-   [ ] **Engine Features**:
    -   [ ] Language support (`lang=en-US`).
    -   [ ] Safe Search integration.
    -   [ ] Paging support.

## Phase 6: Frontend & Assets (Single Binary Goal)

-   [ ] **Templating**:
    -   Migrate from Jinja2 to **Askama** (compile-time checked Jinja-like templates).
    -   Port existing templates (simple, oscar) to Askama.
-   [ ] **Static Assets**:
    -   Embed CSS, JS, images, and fonts into the binary using `rust-embed`.
    -   Serve assets directly from memory with proper HTTP caching headers.
-   [ ] **Localization**:
    -   Implement `fluent` or `gettext` for i18n.
    -   Embed translation files.

## Phase 7: Testing & Quality Assurance

-   [ ] **Unit Tests**: comprehensive coverage for `aggregator`, `config`, and utility functions.
-   [ ] **Integration Tests**: End-to-end tests spinning up the `axum` server.
-   [ ] **Fuzzing**: Use `proptest` to fuzz input query parameters and engine responses.
-   [ ] **Benchmarks**: Use `criterion` to measure critical paths (aggregation, JSON parsing).

## Phase 8: Deployment & Final Polish

-   [ ] **Docker**: Create a scratch/distroless Docker image (< 20MB).
-   [ ] **CI/CD**: GitHub Actions for building, testing, and releasing.
-   [ ] **Documentation**: Complete developer and user guides.
