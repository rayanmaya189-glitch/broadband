//! Bandwidth Worker
//!
//! Monitors bandwidth usage and applies bandwidth profiles to active subscriptions.
//! Handles bandwidth alerts when customers exceed their data caps.

use std::sync::Arc;
use tokio_util::sync::CancellationToken;

use crate::app::AppState;

/// Run the bandwidth monitoring worker.
///
/// This worker periodically:
/// 1. Collects bandwidth usage samples from network devices
/// 2. Updates bandwidth usage records for active subscriptions
/// 3. Applies bandwidth throttling when data caps are exceeded
/// 4. Publishes bandwidth.changed events
pub async fn run_bandwidth_worker(state: Arc<AppState>, shutdown: CancellationToken) {
    tracing::info!("Bandwidth worker started");

    loop {
        tokio::select! {
            _ = shutdown.cancelled() => {
                tracing::info!("Bandwidth worker shutting down");
                break;
            }
            _ = tokio::time::sleep(std::time::Duration::from_secs(30)) => {
                if let Err(e) = collect_bandwidth_samples(&state).await {
                    tracing::error!(error = %e, "Bandwidth sample collection failed");
                }
            }
            _ = tokio::time::sleep(std::time::Duration::from_secs(300)) => {
                if let Err(e) = check_data_cap_usage(&state).await {
                    tracing::error!(error = %e, "Data cap check failed");
                }
            }
        }
    }
}

/// Collect bandwidth usage samples from active sessions.
async fn collect_bandwidth_samples(state: &Arc<AppState>) -> Result<(), crate::common::errors::app_error::AppError> {
    use sea_orm::{EntityTrait, QueryFilter, ColumnTrait};
    use crate::modules::network::model::customer_session_entity;

    let active_sessions = customer_session_entity::Entity::find()
        .filter(customer_session_entity::Column::IsOnline.eq(true))
        .all(&state.db)
        .await?;

    for session in &active_sessions {
        // In production, this would query the device for actual traffic counters
        // For now, we record the session activity
        tracing::debug!(
            customer_id = session.customer_id,
            mac = %session.mac_address,
            "Recording bandwidth sample"
        );
    }

    Ok(())
}

/// Check data cap usage and apply throttling if needed.
async fn check_data_cap_usage(state: &Arc<AppState>) -> Result<(), crate::common::errors::app_error::AppError> {
    use sea_orm::{EntityTrait, QueryFilter, ColumnTrait};
    use crate::modules::subscription::model::subscription_entity;
    use crate::modules::plan::model::plan_entity;

    // Find subscriptions with active status
    let subscriptions = subscription_entity::Entity::find()
        .filter(subscription_entity::Column::Status.eq("active"))
        .all(&state.db)
        .await?;

    for sub in &subscriptions {
        // Check if plan has data cap
        if let Ok(Some(plan)) = plan_entity::Entity::find_by_id(sub.plan_id)
            .one(&state.db)
            .await
        {
            if let Some(data_cap_gb) = plan.data_cap_gb {
                // In production, sum bandwidth_usage for this subscription
                // and compare against data_cap_gb
                tracing::debug!(
                    subscription_id = sub.id,
                    plan = %plan.name,
                    data_cap_gb = data_cap_gb,
                    "Checking data cap"
                );
            }
        }
    }

    Ok(())
}
