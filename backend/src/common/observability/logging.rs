//! Centralized logging configuration.
//!
//! Provides structured logging setup with environment-based filtering.

use tracing_subscriber::{fmt, EnvFilter};

/// Initialize the logging subsystem.
///
/// Uses `RUST_LOG` environment variable for filter configuration.
/// Defaults to `info` level if not set.
pub fn init_logging() {
    let env_filter = EnvFilter::try_from_default_env()
        .unwrap_or_else(|_| EnvFilter::new("info,aeroxe_broadband=debug"));

    fmt()
        .with_env_filter(env_filter)
        .with_target(true)
        .with_thread_ids(true)
        .json()
        .init();
}

/// Initialize logging with a custom filter.
pub fn init_logging_with_filter(filter: &str) {
    let env_filter = EnvFilter::try_new(filter)
        .unwrap_or_else(|_| EnvFilter::new(filter));

    fmt()
        .with_env_filter(env_filter)
        .with_target(true)
        .with_thread_ids(true)
        .json()
        .init();
}
