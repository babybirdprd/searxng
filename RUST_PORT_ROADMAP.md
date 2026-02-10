# SearXNG Rust Port Roadmap

## Vision
The goal is to create a high-performance, single-binary, memory-safe, and privacy-respecting meta-search engine. This port aims to replace the Python implementation with a modern Rust architecture, leveraging its type system, concurrency model, and ecosystem. We are stripping away legacy complexity in favor of clean, idiomatic design.

## Key Principles
1.  **Single Binary:** Everything needed to run the instance should be compile-time included or easily distributable.
2.  **Zero-Cost Abstractions:** Use Rust's traits and generics to handle engine polymorphism without runtime overhead where possible.
3.  **Concurrency:** Utilize `tokio` for efficient asynchronous I/O, handling thousands of concurrent upstream requests.
4.  **Type Safety:** Leverage the type system to prevent common classes of bugs (e.g., proper error handling with `Result`, strong typing for configuration).
5.  **Modularity:** Clean separation of concerns (Engines, Aggregation, Frontend, Configuration).

## Milestones

### Phase 1: Core Architecture (Current)
- [x] **Project Skeleton:** Basic `axum` web server, `tokio` runtime, `reqwest` client.
- [x] **Configuration:** Loading settings from files and environment variables (`config` crate).
- [x] **Engine Trait Definition:** Define a robust `SearchEngine` trait that supports:
    - Asynchronous searching.
    - Configuration injection.
    - Error handling (Retries, Timeouts).
    - Metadata (Name, Categories, Weight).
- [ ] **Engine Registry:** Dynamic loading and management of engines based on configuration.
- [x] **Result Aggregation:** Basic merging of results from multiple engines.

### Phase 2: Engine Implementation & Stabilization
- [ ] **Common Engines:** Implement core engines (Google, Bing, DuckDuckGo, Wikipedia, Qwant).
- [ ] **Generic Engine Types:** Create reusable engine patterns (e.g., `XPathEngine`, `JsonEngine`) to easily port simple engines without writing custom Rust code for each.
- [ ] **Rate Limiting & Proxy Support:** Robust handling of upstream rate limits, utilizing rotating proxies if configured.
- [ ] **Scoring & Deduplication:** Implement algorithms to rank results and remove duplicates.

### Phase 3: Frontend & User Experience
- [ ] **Templating:** Integrate `askama` for type-safe, compile-time compiled HTML templates.
- [ ] **Static Assets:** Embed CSS/JS assets into the binary using `rust-embed`.
- [ ] **Search Interface:** Implement the core search UI (Search bar, Results page).
- [ ] **Preferences:** Handle user preferences (Cookies/LocalStorage).

### Phase 4: Advanced Features & Optimization
- [ ] **Categories:** Support for Images, Videos, News, IT, etc.
- [ ] **Localization:** Internationalization support.
- [ ] **Cache Layer:** In-memory or Redis-based caching for search results.
- [ ] **Metrics:** Prometheus metrics for monitoring engine health and performance.

### Phase 5: Production Readiness
- [ ] **Security Audit:** Review for common web vulnerabilities.
- [ ] **Dockerization:** Minimal scratch/distroless Docker image.
- [ ] **CI/CD:** Automated testing and release pipeline.

## Architectural Decisions (vs Python)
- **Concurrency:** Rust's `async`/`await` model allows us to spawn a task per engine request with minimal overhead, unlike Python's threading/multiprocessing.
- **Error Handling:** We use `Result<T, E>` everywhere. No exceptions. This forces developers to handle failure cases explicitly.
- **Configuration:** Strictly typed configuration using structs, validated at startup.
- **Templates:** `askama` compiles templates to Rust code, ensuring no runtime template errors and high performance.
