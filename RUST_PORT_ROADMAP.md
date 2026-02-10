# SearXNG Rust Port Roadmap

## Goal
The goal of this project is to create a high-performance, single-binary, idiomatic Rust port of SearXNG. This port aims to eliminate the complexity of the original Python codebase while retaining its core functionality and extensibility.

## Architectural Vision
- **Single Binary**: The entire application (web server, engine logic, aggregation) will be compiled into a single executable, simplifying deployment.
- **Async/Await**: Leverage Rust's `async`/`await` ecosystem (primarily `tokio`) for high-concurrency handling of search requests.
- **Type Safety**: Utilize Rust's strong type system to prevent runtime errors, specifically using Enums for result types and `Result` for error handling.
- **Zero-Cost Abstractions**: designing the system to minimize overhead, using traits for engine definitions and generics where appropriate.
- **Configuration**: A robust configuration system that mirrors the flexibility of `settings.yml` but with stricter validation.

## Milestones

### 1. Core Architecture (Current Phase)
- [x] Basic project structure (`src/main.rs`, `src/lib.rs`).
- [x] Configuration loading (`config` crate).
- [x] Basic Engine Registry and Trait definition.
- [x] Refined `SearchResult` model (Enums for Text, Image, Video, etc.).
- [x] robust Error Handling (`thiserror` for libraries, `anyhow` for apps).

### 2. Engine Implementation
- [x] Port core engines (Google, DuckDuckGo).
- [ ] Implement a generic HTTP client with rate limiting and proxy support.
- [ ] Support for different result types (HTML scraping, JSON APIs).

### 3. Result Aggregation
- [x] Advanced deduplication logic.
- [ ] Scoring and ranking improvements.
- [ ] Pagination support.

### 4. Web Interface
- [ ] REST API for search results (JSON).
- [ ] HTML templating (using `askama` or `tera`) for the frontend.
- [ ] Static asset serving.

### 5. Plugin System (Future)
- [ ] WASM-based plugins for extending functionality without recompilation.

## Technical Guidelines

### Error Handling
- Use `thiserror` for defining library-level errors (e.g., `EngineError`).
- Use `anyhow` for application-level error handling in `main.rs` and top-level handlers.
- Errors should be propagated, not swallowed, unless explicitly handled.

### Concurrency
- Use `tokio::task::JoinSet` for managing concurrent engine requests.
- Ensure proper timeout handling for every network request.
- Use `Arc` and `Mutex`/`RwLock` judiciously for shared state.

### Testing
- Unit tests for all core logic (engines, aggregation).
- Integration tests for the web server and full search flow.
- use `mockall` or similar for mocking HTTP responses in tests.

## Contribution
- Follow idiomatic Rust coding standards (`cargo fmt`, `cargo clippy`).
- Document all public functions and structs.
- Write tests for new features.
