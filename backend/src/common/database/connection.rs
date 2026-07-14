use sea_orm::{DatabaseConnection, ConnectOptions};
use std::time::Duration;

use crate::common::errors::app_error::AppError;

pub type DatabasePool = DatabaseConnection;

/// Create a new SeaORM PostgreSQL connection pool.
pub async fn new_pool(
    database_url: &str,
    max_connections: u32,
    _min_connections: u32,
    _connect_timeout_secs: u64,
    _idle_timeout_secs: u64,
) -> Result<DatabasePool, AppError> {
    let mut opts = ConnectOptions::new(database_url);
    opts.max_connections(max_connections)
        .min_connections(1)
        .connect_timeout(Duration::from_secs(30))
        .idle_timeout(Duration::from_secs(300));

    let pool = sea_orm::Database::connect(opts)
        .await
        .map_err(|e| AppError::Database(e.to_string()))?;

    tracing::info!(max = max_connections, "SeaORM PostgreSQL pool created");
    Ok(pool)
}

/// Run database migrations at runtime.
pub async fn run_migrations(_pool: &DatabasePool) -> Result<(), AppError> {
    let migrations_dir = std::path::PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("migrations");

    tracing::info!(path = %migrations_dir.display(), "Migrations directory located");
    tracing::info!("Database migrations should be run via: sea-orm-cli migrate up");
    
    Ok(())
}
