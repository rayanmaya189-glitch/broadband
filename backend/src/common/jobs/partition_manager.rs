use std::time::Duration;

use chrono::Datelike;
use sqlx::PgPool;
use tokio_util::sync::CancellationToken;
use tracing::{info, warn, error};

use crate::app::SharedState;

/// Default check interval: daily at 02:00 UTC (once per day).
const DEFAULT_INTERVAL_SECS: u64 = 86400;

/// Tables that need monthly partition creation.
const PARTITIONED_TABLES: &[&str] = &[
    "audit_logs",
    "device_logs",
    "notifications",
    "events",
    "bandwidth_usage",
    "customer_sessions",
];

/// Create next month's partitions for all partitioned tables.
async fn create_monthly_partitions(pool: &PgPool) -> Result<u32, sqlx::Error> {
    let next_month = chrono::Utc::now()
        .date_naive()
        .with_day(1)
        .unwrap()
        .checked_add_months(chrono::Months::new(1))
        .unwrap();

    let next_next_month = next_month
        .checked_add_months(chrono::Months::new(1))
        .unwrap();

    let partition_suffix = next_month.format("%Y_%m").to_string();
    let partition_start = next_month.format("%Y-%m-01").to_string();
    let partition_end = next_next_month.format("%Y-%m-01").to_string();

    let mut created = 0u32;

    for table in PARTITIONED_TABLES {
        let partition_name = format!("{}_{}", table, partition_suffix);

        let query = format!(
            "CREATE TABLE IF NOT EXISTS {} PARTITION OF {} FOR VALUES FROM ('{}') TO ('{}')",
            partition_name, table, partition_start, partition_end
        );

        match sqlx::query(&query).execute(pool).await {
            Ok(_) => {
                info!(table = table, partition = %partition_name, "Created monthly partition");
                created += 1;
            }
            Err(e) => {
                // Table may not be partitioned yet — log and continue
                warn!(
                    table = table,
                    error = %e,
                    "Could not create partition (table may not be partitioned yet)"
                );
            }
        }
    }

    Ok(created)
}

/// Main partition management loop.
pub async fn run_partition_manager(state: SharedState, token: CancellationToken) {
    let interval_secs = std::env::var("PARTITION_MANAGER_INTERVAL_SECS")
        .ok()
        .and_then(|v| v.parse().ok())
        .unwrap_or(DEFAULT_INTERVAL_SECS);

    let mut interval = tokio::time::interval(Duration::from_secs(interval_secs));

    info!(
        interval_secs = interval_secs,
        "Partition manager background job started"
    );

    loop {
        tokio::select! {
            _ = interval.tick() => {
                match create_monthly_partitions(&state.db).await {
                    Ok(created) if created > 0 => {
                        info!(created = created, "Monthly partitions created successfully");
                    }
                    Ok(_) => {
                        // All partitions already exist
                    }
                    Err(e) => {
                        error!(error = %e, "Failed to create monthly partitions");
                    }
                }
            }
            _ = token.cancelled() => {
                info!("Partition manager shutting down gracefully");
                break;
            }
        }
    }
}
