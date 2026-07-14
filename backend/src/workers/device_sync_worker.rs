//! Device Sync Worker
//!
//! Periodically polls network devices to update their online/offline status.
//! Runs as a background task with graceful shutdown support.

use std::sync::Arc;
use tokio_util::sync::CancellationToken;

use crate::app::AppState;

/// Run the device sync worker.
///
/// This worker periodically:
/// 1. Queries all active network devices
/// 2. Checks their connectivity status
/// 3. Updates device status in the database
/// 4. Publishes device.online/device.offline events via NATS
pub async fn run_device_sync_worker(state: Arc<AppState>, shutdown: CancellationToken) {
    tracing::info!("Device sync worker started");

    loop {
        tokio::select! {
            _ = shutdown.cancelled() => {
                tracing::info!("Device sync worker shutting down");
                break;
            }
            _ = tokio::time::sleep(std::time::Duration::from_secs(60)) => {
                if let Err(e) = sync_device_status(&state).await {
                    tracing::error!(error = %e, "Device sync failed");
                }
            }
        }
    }
}

/// Sync device status by checking connectivity of all active devices.
async fn sync_device_status(state: &Arc<AppState>) -> Result<(), crate::common::errors::app_error::AppError> {
    use sea_orm::{EntityTrait, QueryFilter, ColumnTrait, Set, ActiveModelTrait};
    use crate::modules::device::model::network_device_entity;

    let devices = network_device_entity::Entity::find()
        .filter(network_device_entity::Column::Status.eq("active"))
        .all(&state.db)
        .await?;

    let mut online_count = 0u64;
    let mut offline_count = 0u64;

    for device in &devices {
        // Check device connectivity (simplified - in production, use SNMP/ping)
        let is_online = check_device_connectivity(&device.management_ip).await;

        if is_online && device.status != "online" {
            let mut active: network_device_entity::ActiveModel = device.clone().into();
            active.status = Set("online".to_string());
            active.update(&state.db).await?;
            online_count += 1;

            // Publish device.online event
            let _ = state.nats.publish_event("network.device_online", &serde_json::json!({
                "device_id": device.id,
                "device_name": device.name,
                "management_ip": device.management_ip,
            })).await;
        } else if !is_online && device.status != "offline" {
            let mut active: network_device_entity::ActiveModel = device.clone().into();
            active.status = Set("offline".to_string());
            active.update(&state.db).await?;
            offline_count += 1;

            // Publish device.offline event
            let _ = state.nats.publish_event("network.device_offline", &serde_json::json!({
                "device_id": device.id,
                "device_name": device.name,
                "management_ip": device.management_ip,
            })).await;
        }
    }

    if online_count > 0 || offline_count > 0 {
        tracing::info!(online = online_count, offline = offline_count, "Device sync completed");
    }

    Ok(())
}

/// Check if a device is reachable at the given IP.
/// In production, this would use SNMP, ICMP ping, or management API.
async fn check_device_connectivity(ip: &str) -> bool {
    // Simplified connectivity check using TCP connection attempt
    match tokio::net::TcpStream::connect(format!("{ip}:22")).await {
        Ok(_) => true,
        Err(_) => {
            // Try common management ports
            for port in &[80, 443, 161] {
                if tokio::net::TcpStream::connect(format!("{ip}:{port}")).await.is_ok() {
                    return true;
                }
            }
            false
        }
    }
}
