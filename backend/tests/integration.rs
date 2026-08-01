//! Repository/integration tests (require Docker + testcontainers).
//!
//! These spin up a real PostgreSQL 16 container, so they are ignored by
//! default. Run them explicitly with:
//!
//! ```sh
//! cargo test --test integration -- --ignored --nocapture
//! ```

mod common;
#[path = "integration/mod.rs"]
mod integration;
