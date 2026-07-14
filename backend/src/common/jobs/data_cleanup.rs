//! Data cleanup — removes expired data based on retention policies.
//!
//! Pure SeaORM implementation — zero raw SQL queries.

use std::time::Duration;

use sea_orm::*;
use tokio_util::sync::CancellationToken;
use tracing::{info, warn, error};

use crate::app::SharedState;
use crate::modules::device::model::device_log_entity::{self, Entity as DeviceLogEntity};
use crate::modules::device::model::device_metric_entity::{self, Entity as DeviceMetricEntity};
use crate::modules::notification::model::notification_entity::{self, Entity as NotificationEntity};
use crate::modules::event::model::event_entity::{self, Entity as EventEntity};
use crate::modules::bandwidth::model::bandwidth_usage::{self, Entity as BandwidthUsageEntity};
use crate::modules::network::model::customer_session_entity::{self, Entity as CustomerSessionEntity};
use crate::modules::audit::model::audit_log_entity::{self, Entity as AuditLogEntity};

const DEFAULT_INTERVAL_SECS: u64 = 86400; // 24 hours

/// Generic cleanup helper — deletes rows older than retention_days using pure SeaORM.
async fn cleanup_entity<E: EntityTrait>(
    db: &DatabaseConnection,
    column: E::Column,
    retention_days: i32,
) -> Result<u64, DbErr> {
    let cutoff = chrono::Utc::now() - chrono::Duration::days(retention_days as i64);
    let result = E::delete_many()
        .filter(column.lt(cutoff))
        .exec(db)
        .await?;
    Ok(result.rows_affected)
}

/// Cleanup expired data from all tables with retention policies.
async fn cleanup_expired_data(db: &DatabaseConnection) -> Result<u64, DbErr> {
    let mut total_deleted = 0u64;

    let deleted = cleanup_entity::<DeviceLogEntity>(db, device_log_entity::Column::CreatedAt, 90).await?;
    if deleted > 0 {
        info!(table = "device_logs", deleted = deleted, "Deleted expired rows");
        total_deleted += deleted;
    }

    let deleted = cleanup_entity::<DeviceMetricEntity>(db, device_metric_entity::Column::RecordedAt, 90).await?;
    if deleted > 0 {
        info!(table = "device_metrics", deleted = deleted, "Deleted expired rows");
        total_deleted += deleted;
    }

    let deleted = cleanup_entity::<NotificationEntity>(db, notification_entity::Column::CreatedAt, 90).await?;
    if deleted > 0 {
        info!(table = "notifications", deleted = deleted, "Deleted expired rows");
        total_deleted += deleted;
    }

    let deleted = cleanup_entity::<EventEntity>(db, event_entity::Column::PublishedAt, 365).await?;
    if deleted > 0 {
        info!(table = "events", deleted = deleted, "Deleted expired rows");
        total_deleted += deleted;
    }

    let deleted = cleanup_entity::<BandwidthUsageEntity>(db, bandwidth_usage::Column::RecordedAt, 365).await?;
    if deleted > 0 {
        info!(table = "bandwidth_usage", deleted = deleted, "Deleted expired rows");
        total_deleted += deleted;
    }

    let deleted = cleanup_entity::<CustomerSessionEntity>(db, customer_session_entity::Column::CreatedAt, 90).await?;
    if deleted > 0 {
        info!(table = "customer_sessions", deleted = deleted, "Deleted expired rows");
        total_deleted += deleted;
    }

    let deleted = cleanup_entity::<AuditLogEntity>(db, audit_log_entity::Column::CreatedAt, 2555).await?;
    if deleted > 0 {
        info!(table = "audit_logs", deleted = deleted, "Deleted expired rows");
        total_deleted += deleted;
    }

    Ok(total_deleted)
}

pub async fn run_data_cleanup(state: SharedState, token: CancellationToken) {
    let interval_secs = std::env::var("DATA_CLEANUP_INTERVAL_SECS")
        .ok()
        .and_then(|v| v.parse().ok())
        .unwrap_or(DEFAULT_INTERVAL_SECS);
    let mut interval = tokio::time::interval(Duration::from_secs(interval_secs));
    info!(interval_secs = interval_secs, "Data cleanup background job started");

    loop {
        tokio::select! {
            _ = interval.tick() => {
                if let Err(e) = super::set_rls_bypass(&state.db).await {
                    warn!(error = %e, "Failed to set RLS bypass context");
                    continue;
                }

                match cleanup_expired_data(&state.db).await {
                    Ok(deleted) if deleted > 0 => {
                        info!(total_deleted = deleted, "Data cleanup batch complete");
                    }
                    Ok(_) => {}
                    Err(e) => error!(error = %e, "Data cleanup failed"),
                }
            }
            _ = token.cancelled() => {
                info!("Data cleanup shutting down gracefully");
                break;
            }
        }
    }
}
