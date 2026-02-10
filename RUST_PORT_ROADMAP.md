# Rust Port Roadmap for SearXNG

## Goal
To provide a high-performance, memory-safe, single-binary distribution of SearXNG by porting the core logic to Rust.

## Principles
- **Idiomatic Rust**: Leverage ownership, type system, and concurrency features.
- **Performance**: Zero-cost abstractions where possible.
- **Maintainability**: Modular architecture.
- **Compatibility**: Maintain compatibility with existing SearXNG ecosystem where reasonable, but prioritize better architecture.

## Phase 1: Foundation (Current)
- [x] Basic project structure (tokio, axum, reqwest).
- [x] `SearchEngine` trait definition.
- [x] `EngineRegistry` for concurrent execution.
- [x] Basic configuration loading.
- [x] Dummy engine for testing.

## Phase 2: Core Engine Architecture (In Progress)
- [x] Implement robust engine configuration (enable/disable, weights, throttling).
- [ ] Add concrete engine implementations (DuckDuckGo, Google, Bing, etc.).
- [ ] Implement HTML parsing capabilities (using `scraper` or similar).
- [ ] Enhance error handling and reporting.
- [x] Add result aggregation and ranking logic.

## Phase 3: Web Interface & API
- [ ] Implement HTML templating (Askama or similar).
- [ ] Port static assets handling.
- [ ] Implement advanced query parsing (search syntax like `!ddg`).
- [ ] Add support for different output formats (JSON, CSV, RSS).

## Phase 4: Advanced Features
- [ ] Redis integration for caching.
- [ ] Rate limiting and anti-bot protection.
- [ ] Plugin system (if needed, or compile-time features).
- [ ] Proxy support.

## Phase 5: Polish & Release
- [ ] Comprehensive testing (unit, integration).
- [ ] Benchmarking against Python implementation.
- [ ] CI/CD pipelines.
- [ ] Documentation.
