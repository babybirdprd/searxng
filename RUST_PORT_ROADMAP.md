# Rust Port Roadmap for SearXNG

## Goal
To provide a high-performance, memory-safe, single-binary distribution of SearXNG by porting the core logic to Rust. This port aims to re-architect the codebase to leverage Rust's ownership model, type system, and zero-cost abstractions, rather than performing a line-by-line translation.

## Principles
- **Idiomatic Rust**: Leverage ownership, type system, and concurrency features (e.g., `async/await`, `tokio`).
- **Performance**: Use zero-cost abstractions where possible. Minimize allocations and cloning.
- **Maintainability**: Modular architecture with clear boundaries (e.g., `SearchEngine` trait, `EngineRegistry`).
- **Compatibility**: Maintain compatibility with existing SearXNG ecosystem where reasonable, but prioritize better architecture.
- **Single Binary**: The end goal is a single executable that includes everything needed to run a SearXNG instance.

## Phase 1: Foundation (Completed)
- [x] Basic project structure (`tokio`, `axum`, `reqwest`).
- [x] `SearchEngine` trait definition.
- [x] `EngineRegistry` for concurrent execution.
- [x] Basic configuration loading (`config` crate).
- [x] Dummy engine for testing.

## Phase 2: Core Engine Architecture (In Progress)
- [x] Implement robust engine configuration (enable/disable, weights, throttling).
- [ ] Add `tokens` and `extra` configuration fields to support API keys and engine-specific settings.
- [ ] Implement `Google` engine skeleton to validate multi-engine architecture.
- [ ] Enhance `ResultAggregator` to handle score boosting for duplicate results.
- [ ] Refine error handling using `thiserror` for library errors and `anyhow` for application errors.
- [ ] Implement common middleware for engines (e.g., User-Agent rotation, request delays).

## Phase 3: Web Interface & API (Next)
- [ ] Implement HTML templating (using `Askama` or `Tera`).
- [ ] Port static assets handling (embed in binary using `rust-embed`).
- [ ] Implement advanced query parsing (search syntax like `!ddg`, `!g`).
- [ ] Add support for different output formats (JSON, CSV, RSS).

## Phase 4: Advanced Features
- [ ] Redis integration for caching (using `redis` crate).
- [ ] Rate limiting and anti-bot protection.
- [ ] Plugin system (possibly via compile-time features or WASM).
- [ ] Proxy support for engines.

## Phase 5: Polish & Release
- [ ] Comprehensive testing (unit, integration).
- [ ] Benchmarking against Python implementation.
- [ ] CI/CD pipelines.
- [ ] Documentation.
