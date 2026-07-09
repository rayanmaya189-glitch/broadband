use std::time::Duration;

use sqlx::PgPool;
use tokio_util::sync::CancellationToken;
use tracing::{info, error};

use crate::app::SharedState;

/// Default check interval: daily at 03:00 UTC.
const DEFAULT_INTERVAL_SECS: u64 = 86400;

/// Data retention policies: (table_name, retention_days, timestamp_column)
const RETENTION_POLICIES: &[(&str, i32, &str)] = &[
    // Device logs: 90 days
    ("device_logs", 90, "created_at"),
    // Device metrics: 90 days
    ("device_metrics", 90, "recorded_at"),
    // Notifications: 90 days
    ("notifications", 90, "created_at"),
    // Events: 365 days (1 year)
    ("events", 365, "published_at"),
    // Bandwidth usage: 365 days (1 year)
    ("bandwidth_usage", 365, "recorded_at"),
    // Customer sessions: 90 days
    ("customer_sessions", 90, "created_at"),
    // Audit logs: 2555 days (7 years) — legal retention
    ("audit_logs", 2555, "created_at"),
];

/// Delete expired data based on retention policies.
async fn cleanup_expired_data(pool: &PgPool) -> Result<u64, sqlx::Error> {
    let mut total_deleted = 0u64;

    for (table, retention_days, ts_column) in RETENTION_POLICIES {
        let query = format!(
            "DELETE FROM {} WHERE {} < NOW() - INTERVAL '{} days' LIMIT 10000",
            table, ts_column, retention_days
        );

        loop {
            match sqlx::query(&query).execute(pool).await {
                Ok(result) => {
                    let rows = result.rows_affected();
                    total_deleted += rows;
                    if rows == 0 {
                        break;
                    }
                    info!(
                        table = table,
                        deleted = rows,
                        "Deleted expired rows"
                    );
                }
                Err(e) => {
                    error!(
                        table = table,
                        error = %e,
                        "Failed to delete expired rows"
                    );
                    break;
                }
            }
        }
    }

    Ok(total_deleted)
}

/// Main data cleanup loop.
pub async fn run_data_cleanup(state: SharedState, token: CancellationToken) {
    let interval_secs = std::env::var("DATA_CLEANUP_INTERVAL_SECS")
        .ok()
        .and_then(|v| v.parse().ok())
        .unwrap_or(DEFAULT_INTERVAL_SECS);

    let mut interval = tokio::time::interval(Duration::from_secs(interval_secs));

    info!(
        interval_secs = interval_secs,
        "Data cleanup background job started"
    );

    loop {
        tokio::select! {
            _ = interval.tick() => {
                match cleanup_expired_data(&state.db).await {
                    Ok(deleted) if deleted > 0 => {
                        info!(total_deleted = deleted, "Data cleanup batch complete");
                    }
                    Ok(_) => {
                        // No expired data to clean
                    }
                    Err(e) => {
                        error!(error = %e, "Data cleanup failed");
                    }
                }
            }
            _ = token.cancelled() => {
                info!("Data cleanup shutting down gracefully");
                break;
            }
        }
    }
}
