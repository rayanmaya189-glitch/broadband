/// Partition Management Jobs per §32 docs.
/// Auto-creates monthly partitions for history, audit, notification, and events tables.
/// Runs as a scheduled background job (via scheduler module).

use sea_orm::{ConnectionTrait, DatabaseConnection, Statement};
use chrono::{Utc, Datelike, NaiveDate};
use tracing::{info, warn};

/// Tables that require monthly partitioning per §32 docs.
const PARTITIONED_TABLES: &[&str] = &[
    "customers_history",
    "subscriptions_history",
    "plans_history",
    "invoices_history",
    "refunds_history",
    "journal_entries_history",
    "manual_payments_history",
    "network_devices_history",
    "payment_gateways_history",
    "discounts_history",
    "approval_requests_history",
    "bandwidth_profiles_history",
    "audit_logs",
    "notifications",
    "events",
];

/// Create monthly partitions for all partitioned tables.
/// Should be called monthly (e.g., by scheduler on the 1st of each month).
pub async fn create_monthly_partitions(db: &DatabaseConnection) -> Result<(), anyhow::Error> {
    let now = Utc::now().date_naive();
    let next_month = NaiveDate::from_ymd_opt(
        if now.month() == 12 { now.year() + 1 } else { now.year() },
        if now.month() == 12 { 1 } else { now.month() + 1 },
        1,
    ).unwrap();

    let next_next_month = NaiveDate::from_ymd_opt(
        if next_month.month() == 12 { next_month.year() + 1 } else { next_month.year() },
        if next_month.month() == 12 { 1 } else { next_month.month() + 1 },
        1,
    ).unwrap();

    let partition_name_suffix = format!("{:04}_{:02}", next_month.year(), next_month.month());
    let partition_start = next_month.format("%Y-%m-01").to_string();
    let partition_end = next_next_month.format("%Y-%m-01").to_string();

    for table in PARTITIONED_TABLES {
        let partition_name = format!("{}_{}", table, partition_name_suffix);
        let query = format!(
            "CREATE TABLE IF NOT EXISTS {} PARTITION OF {} FOR VALUES FROM ('{}') TO ('{}')",
            partition_name, table, partition_start, partition_end
        );

        match db.execute(Statement::from_string(
            db.get_database_backend(),
            query,
        )).await {
            Ok(_) => info!(partition = %partition_name, "Created partition"),
            Err(e) => warn!(
                table = table,
                error = %e,
                "Failed to create partition (may not exist yet)"
            ),
        }
    }

    Ok(())
}

/// Run data cleanup based on retention policies per §30 Appendix C.
pub async fn run_cleanup(db: &DatabaseConnection) -> Result<u64, anyhow::Error> {
    let mut total_deleted: u64 = 0;

    // OTP codes (5 minutes)
    let result = db.execute(Statement::from_string(
        db.get_database_backend(),
        "DELETE FROM otp_codes WHERE expires_at < NOW()".to_string(),
    )).await?;
    total_deleted += result.rows_affected();

    // Sessions (24 hours)
    let result = db.execute(Statement::from_string(
        db.get_database_backend(),
        "DELETE FROM user_sessions WHERE expires_at < NOW()".to_string(),
    )).await?;
    total_deleted += result.rows_affected();

    // Refresh tokens (7 days)
    let result = db.execute(Statement::from_string(
        db.get_database_backend(),
        "DELETE FROM refresh_tokens WHERE expires_at < NOW()".to_string(),
    )).await?;
    total_deleted += result.rows_affected();

    // Device metrics (90 days)
    let result = db.execute(Statement::from_string(
        db.get_database_backend(),
        "DELETE FROM device_metrics WHERE recorded_at < NOW() - INTERVAL '90 days'".to_string(),
    )).await?;
    total_deleted += result.rows_affected();

    // Device logs (30 days)
    let result = db.execute(Statement::from_string(
        db.get_database_backend(),
        "DELETE FROM device_logs WHERE recorded_at < NOW() - INTERVAL '30 days'".to_string(),
    )).await?;
    total_deleted += result.rows_affected();

    // Notifications (90 days)
    let result = db.execute(Statement::from_string(
        db.get_database_backend(),
        "DELETE FROM notifications WHERE created_at < NOW() - INTERVAL '90 days'".to_string(),
    )).await?;
    total_deleted += result.rows_affected();

    // Old published outbox events (24 hours)
    let result = db.execute(Statement::from_string(
        db.get_database_backend(),
        "DELETE FROM outbox_events WHERE published = true AND created_at < NOW() - INTERVAL '24 hours'".to_string(),
    )).await?;
    total_deleted += result.rows_affected();

    // History cleanup per retention policies
    let retention_policies: Vec<(&str, i64)> = vec![
        ("customers_history", 2555),
        ("subscriptions_history", 2555),
        ("plans_history", 2555),
        ("invoices_history", 2555),
        ("refunds_history", 2555),
        ("journal_entries_history", 2555),
        ("manual_payments_history", 2555),
        ("network_devices_history", 1095),
        ("payment_gateways_history", 1095),
        ("discounts_history", 1095),
        ("approval_requests_history", 1095),
        ("bandwidth_profiles_history", 730),
    ];

    for (table, retention_days) in retention_policies {
        let query = format!(
            "DELETE FROM {} WHERE created_at < NOW() - INTERVAL '{} days'",
            table, retention_days
        );
        if let Ok(result) = db.execute(Statement::from_string(
            db.get_database_backend(),
            query,
        )).await {
            total_deleted += result.rows_affected();
        }
    }

    info!(total_deleted = total_deleted, "Data cleanup completed");
    Ok(total_deleted)
}
