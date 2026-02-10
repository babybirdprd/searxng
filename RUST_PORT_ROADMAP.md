# SearXNG Rust Port Roadmap

## Philosophy
The goal of this port is **not** a direct line-by-line translation of the Python codebase. Instead, we aim to re-architect SearXNG to leverage Rust's strengths:
- **Ownership & Concurrency:** Utilize Rust's ownership model and async runtime (Tokio) for safe, efficient, and concurrent search engine querying without the overhead of Python's GIL or multiprocessing.
- **Type Safety:** Enforce correctness at compile time using robust types for search queries, results, and engine configurations.
- **Zero-Cost Abstractions:** Use traits and generics to define search engine interfaces that incur no runtime penalty.
- **Single Binary:** The final artifact should be a single, statically linked binary with embedded assets (templates, static files), easy to deploy and run anywhere.

## Roadmap

### Phase 1: Core Architecture & Foundation (In Progress)
- [x] **Project Setup**: Initialize Rust project with `cargo`, set up directory structure.
- [x] **Web Framework**: Integrate `axum` for high-performance, asynchronous HTTP handling.
- [x] **Configuration**: Implement a type-safe configuration system using `config-rs`.
- [ ] **Engine Trait Definition**: Define the `SearchEngine` trait for standardizing engine interactions.
- [ ] **Engine Registry**: Implement a mechanism to register, configure, and retrieve search engines dynamically.
- [ ] **Concurrent Dispatch**: Build the logic to query multiple engines in parallel using `tokio` and aggregate results.

### Phase 2: Search Logic & Data Handling
- [ ] **HTTP Client**: Integrate `reqwest` with a shared connection pool for efficient outbound requests.
- [ ] **Result Aggregation**: Implement algorithms for deduplication, scoring, and ranking of search results from various engines.
- [ ] **Error Handling**: Robust error handling for network failures, timeouts, and parsing errors (using `thiserror` and `anyhow`).
- [ ] **Safe Search & Filtering**: Implement logic for safe search levels and language/region filtering.

### Phase 3: User Interface & Experience
- [ ] **Templating**: Integreate a compile-time templating engine (e.g., `askama` or `rinja`) for high-performance HTML rendering.
- [ ] **Static Assets**: Embed static assets (CSS, JS, images) into the binary using `rust-embed`.
- [ ] **Search API**: Expose a JSON API for programmatic access, maintaining compatibility with existing clients where possible.
- [ ] **Preferences**: Implement user preferences (stored in cookies or local storage) without server-side state.

### Phase 4: Engine Implementations
- [ ] **General Engines**: Port major general engines (Google, Bing, DuckDuckGo, Wikipedia).
- [ ] **Specialized Engines**: Port specialized engines (Images, Videos, IT, Maps).
- [ ] **Scraper Logic**: Implement robust HTML parsing (using `scraper` or `select`) and JSON handling for extracting results.

### Phase 5: Advanced Features & Optimization
- [ ] **Rate Limiting**: Implement request rate limiting to prevent abuse.
- [ ] **Image Proxy**: secure image proxying to protect user privacy.
- [ ] **Plugin System**: (Optional) Explore WASM-based plugins for extending functionality without recompilation.
- [ ] **Metrics & Monitoring**: Integrate `tracing` and `metrics` for observability.

## Architecture Deviations from Python
1.  **Concurrency Model**: Replaced Python's threading/multiprocessing with Rust's async/await (Tokio). This allows handling thousands of concurrent connections and engine requests with minimal overhead.
2.  **Configuration**: Static, type-checked configuration structs instead of dynamic Python dictionaries.
3.  **Templating**: Compile-time template rendering (Askama) instead of runtime (Jinja2) for significant performance gains and safety.
4.  **No Global State**: Explicit dependency injection of configuration and HTTP clients, avoiding global mutable state.

## Getting Started
To run the project:
```bash
cargo run
```
