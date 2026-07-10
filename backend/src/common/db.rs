//! Shared database module — SeaORM connection management.
//!
//! Provides a type alias `DbConn` for convenience and helper functions
//! to establish a connection pool using SeaORM's `Database::connect`.

use sea_orm::{DatabaseConnection, ConnectOptions};
use tracing::info;

/// Type alias for a SeaORM database connection reference.
/// Use `&DbConn` in repository and service function signatures.
pub type DbConn = DatabaseConnection;

/// Connect to PostgreSQL using SeaORM.
///
/// # Arguments
/// * `database_url` — Full PostgreSQL connection string (e.g. `postgres://user:pass@host/db`)
/// * `max_connections` — Maximum number of connections in the pool
pub async fn connect(database_url: &str, max_connections: u32) -> Result<DbConn, sea_orm::DbErr> {
    let mut opts = ConnectOptions::new(database_url);
    opts.max_connections(max_connections)
        .min_connections(1)
        .connect_timeout(std::time::Duration::from_secs(30))
        .idle_timeout(std::time::Duration::from_secs(300));

    let db = sea_orm::Database::connect(opts).await?;
    info!(max_connections, "SeaORM PostgreSQL pool connected");
    Ok(db)
}
