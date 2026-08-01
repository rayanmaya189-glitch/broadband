//! End-to-end workflow tests (require Docker + testcontainers).
//!
//! These spin up a real PostgreSQL 16 container, so they are ignored by
//! default. Run them explicitly with:
//!
//! ```sh
//! cargo test --test e2e -- --ignored --nocapture
//! ```

mod common;
#[path = "e2e/mod.rs"]
mod e2e;
