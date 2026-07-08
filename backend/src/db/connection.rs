//! PostgreSQL connection pool setup.

use sqlx::postgres::{PgPoolOptions, PgPool};
use std::time::Duration;

use crate::error::AppError;

/// Type alias for the database pool.
pub type DatabasePool = PgPool;

/// Create a new PostgreSQL connection pool.
pub async fn new_pool(
    database_url: &str,
    max_connections: u32,
    min_connections: u32,
    connect_timeout_secs: u64,
    idle_timeout_secs: u64,
) -> Result<DatabasePool, AppError> {
    let pool = PgPoolOptions::new()
        .max_connections(max_connections)
        .min_connections(min_connections)
        .acquire_timeout(Duration::from_secs(connect_timeout_secs))
        .idle_timeout(Duration::from_secs(idle_timeout_secs))
        .connect(database_url)
        .await?;

    tracing::info!(
        max = max_connections,
        min = min_connections,
        "PostgreSQL connection pool created"
    );

    Ok(pool)
}

/// Run database migrations from the migrations directory.
///
/// The `sqlx::migrate!` macro resolves relative to the crate root.
/// Migrations live in `backend/migrations/`.
pub async fn run_migrations(pool: &DatabasePool) -> Result<(), AppError> {
    sqlx::migrate!("migrations")
        .run(pool)
        .await
        .map_err(|e| anyhow::anyhow!("Migration failed: {e}"))?;

    tracing::info!("Database migrations completed successfully");
    Ok(())
}
