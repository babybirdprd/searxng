# SearXNG Rust Port Roadmap

## Goal
Create a high-performance, memory-safe, single-binary metasearch engine by porting SearXNG to Rust.
The focus is on re-architecting for Rust idioms (ownership, type safety, concurrency) rather than a direct translation.
We aim to "strip the fat" and produce a lean, self-contained executable.

## Architectural Shifts
- **Runtime -> Compile-time**: Use strong typing and compile-time checks where possible (e.g., Askama for templates).
- **Dynamic -> Static Dispatch**: Use Generics and Enums over dynamic dispatch where appropriate for performance.
- **Concurrency**: Leverage `tokio` for asynchronous I/O and parallel engine requests, replacing Python's threading/multiprocessing.
- **Single Binary**: Embed static assets and templates into the final executable using `rust-embed` or similar.
- **Configuration**: Strongly typed configuration using `config-rs` or `serde`.

## Phases

### Phase 1: Foundation & Core Types
- [ ] Initialize Cargo workspace/project.
- [ ] Define core domain models (SearchQuery, SearchResult, Category, etc.).
- [ ] Implement Configuration system (load from TOML/YAML/Env).
- [ ] Set up logging/tracing infrastructure (`tracing`).

### Phase 2: Search Engine Architecture
- [ ] Define `SearchEngine` trait (async).
- [ ] Implement engine registry (static or compile-time map).
- [ ] Implement 1-2 example engines (e.g., DuckDuckGo, Wikipedia) to validate the trait.
- [ ] Implement the `SearchOrchestrator` to handle concurrent requests to multiple engines, aggregation, and timeout management.
    - [ ] Deduplication logic.
    - [ ] Result ranking/sorting.

### Phase 3: Web Interface (Axum)
- [ ] Set up `axum` web server.
- [ ] Implement basic routes (`/`, `/search`, `/config`).
- [ ] Integrate `askama` for type-safe templating.
- [ ] Embed static assets (CSS, JS, Images) into the binary.

### Phase 4: Preferences & Settings
- [ ] Implement user preferences handling (cookies/local storage/server-side).
- [ ] Map preferences to search parameters.

### Phase 5: Optimization & Hardening
- [ ] Implement Caching layer (In-memory via `moka` or similar to avoid external Redis dependency for basic setups).
- [ ] Rate limiting (in-memory).
- [ ] Error handling strategy (`thiserror` for libs, `anyhow` for app).
- [ ] Http Client tuning (`reqwest` middleware).

### Phase 6: Engine Expansion & Cleanup
- [ ] Port additional high-priority engines.
- [ ] "Strip the Fat": Evaluate which features from Python are essential. Drop complex/unused plugins or legacy support.
- [ ] Comprehensive testing (Unit, Integration).
- [ ] CI/CD for cross-compilation (build single binaries for Linux, macOS, Windows).

## Technical Choices
- **Web Framework**: `axum` (ergonomic, fast, runs on tokio).
- **Async Runtime**: `tokio`.
- **HTTP Client**: `reqwest`.
- **Templating**: `askama` (Jinja-like but compile-time checked).
- **Serialization**: `serde`, `serde_json`.
- **Asset Embedding**: `rust-embed`.
- **HTML Parsing**: `scraper` or `html5ever` (for scraping engines).

## Notes
- The "plugins" system of Python might be replaced by compile-time features or a simplified middleware system if dynamic loading isn't strictly necessary.
- External dependencies (Redis) should be optional. The default should be "works out of the box".
