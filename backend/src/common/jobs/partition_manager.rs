use std::time::Duration;

use chrono::Datelike;
use tokio_util::sync::CancellationToken;
use tracing::{info, warn, error};

use crate::app::SharedState;

const DEFAULT_INTERVAL_SECS: u64 = 86400;
const PARTITIONED_TABLES: &[&str] = &["audit_logs", "device_logs", "notifications", "events", "bandwidth_usage", "customer_sessions"];

async fn create_monthly_partitions(conn: &mut sqlx::PgConnection) -> Result<u32, sqlx::Error> {
    let next_month = chrono::Utc::now().date_naive().with_day(1).unwrap().checked_add_months(chrono::Months::new(1)).unwrap();
    let next_next_month = next_month.checked_add_months(chrono::Months::new(1)).unwrap();
    let suffix = next_month.format("%Y_%m").to_string();
    let start = next_month.format("%Y-%m-01").to_string();
    let end = next_next_month.format("%Y-%m-01").to_string();
    let mut created = 0u32;
    for table in PARTITIONED_TABLES {
        let name = format!("{}_{}", table, suffix);
        let q = format!("CREATE TABLE IF NOT EXISTS {} PARTITION OF {} FOR VALUES FROM ('{}') TO ('{}')", name, table, start, end);            match sqlx::query(&q).execute(&mut *conn).await {
            Ok(_) => { info!(table = table, partition = %name, "Created monthly partition"); created += 1; }
            Err(e) => warn!(table = table, error = %e, "Could not create partition (table may not be partitioned yet)"),
        }
    }
    Ok(created)
}

pub async fn run_partition_manager(state: SharedState, token: CancellationToken) {
    let interval_secs = std::env::var("PARTITION_MANAGER_INTERVAL_SECS").ok().and_then(|v| v.parse().ok()).unwrap_or(DEFAULT_INTERVAL_SECS);
    let mut interval = tokio::time::interval(Duration::from_secs(interval_secs));
    info!(interval_secs = interval_secs, "Partition manager background job started");

    loop {
        tokio::select! {
            _ = interval.tick() => {
                let mut tx = match super::rls_bypass::begin_bypass_transaction(&state.db).await {
                    Ok(t) => t, Err(e) => { warn!(error = %e, "Failed to begin RLS bypass transaction"); continue; }
                };
                match create_monthly_partitions(&mut tx).await {
                    Ok(created) if created > 0 => info!(created = created, "Monthly partitions created successfully"),
                    Ok(_) => {}
                    Err(e) => error!(error = %e, "Failed to create monthly partitions"),
                }
                if let Err(e) = tx.commit().await { error!(error = %e, "Failed to commit RLS bypass transaction"); }
            }
            _ = token.cancelled() => { info!("Partition manager shutting down gracefully"); break; }
        }
    }
}
