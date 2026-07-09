use std::time::Duration;

use tokio_util::sync::CancellationToken;
use tracing::{info, warn, error};

use crate::app::SharedState;

const DEFAULT_INTERVAL_SECS: u64 = 86400;
const RETENTION_POLICIES: &[(&str, i32, &str)] = &[
    ("device_logs", 90, "created_at"), ("device_metrics", 90, "recorded_at"),
    ("notifications", 90, "created_at"), ("events", 365, "published_at"),
    ("bandwidth_usage", 365, "recorded_at"), ("customer_sessions", 90, "created_at"),
    ("audit_logs", 2555, "created_at"),
];

async fn cleanup_expired_data(conn: &mut sqlx::PgConnection) -> Result<u64, sqlx::Error> {
    let mut total = 0u64;
    for (table, days, col) in RETENTION_POLICIES {
        let q = format!("DELETE FROM {} WHERE {} < NOW() - INTERVAL '{} days' LIMIT 10000", table, col, days);
        loop {
            match sqlx::query(&q).execute(&mut *conn).await {
                Ok(r) => { let n = r.rows_affected(); total += n; if n == 0 { break; } info!(table = table, deleted = n, "Deleted expired rows"); }
                Err(e) => { error!(table = table, error = %e, "Failed to delete expired rows"); break; }
            }
        }
    }
    Ok(total)
}

pub async fn run_data_cleanup(state: SharedState, token: CancellationToken) {
    let interval_secs = std::env::var("DATA_CLEANUP_INTERVAL_SECS").ok().and_then(|v| v.parse().ok()).unwrap_or(DEFAULT_INTERVAL_SECS);
    let mut interval = tokio::time::interval(Duration::from_secs(interval_secs));
    info!(interval_secs = interval_secs, "Data cleanup background job started");

    loop {
        tokio::select! {
            _ = interval.tick() => {
                let mut tx = match super::rls_bypass::begin_bypass_transaction(&state.db).await {
                    Ok(t) => t, Err(e) => { warn!(error = %e, "Failed to begin RLS bypass transaction"); continue; }
                };
                match cleanup_expired_data(&mut tx).await {
                    Ok(d) if d > 0 => info!(total_deleted = d, "Data cleanup batch complete"),
                    Ok(_) => {}
                    Err(e) => error!(error = %e, "Data cleanup failed"),
                }
                if let Err(e) = tx.commit().await { error!(error = %e, "Failed to commit RLS bypass transaction"); }
            }
            _ = token.cancelled() => { info!("Data cleanup shutting down gracefully"); break; }
        }
    }
}
