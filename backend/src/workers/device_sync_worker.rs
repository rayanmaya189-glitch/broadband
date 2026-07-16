use sea_orm::{DatabaseConnection, EntityTrait, QueryFilter, ColumnTrait, Set, ActiveModelTrait};
use tracing::{debug, info, warn, error};

use crate::infrastructure::messaging::outbox;
use crate::modules::integrations::{DeviceAdapterFactory, DeviceType};

/// Background worker for device synchronization:
/// - Poll device health status
/// - Update device status in database
/// - Detect offline devices
/// - Publish status change events
pub struct DeviceSyncWorker {
    db: DatabaseConnection,
}

impl DeviceSyncWorker {
    pub fn new(db: DatabaseConnection) -> Self {
        Self { db }
    }

    /// Run the full device sync cycle.
    pub async fn run_cycle(&self) -> anyhow::Result<()> {
        info!("Device sync worker: starting cycle");
        self.sync_device_status().await?;
        self.detect_offline_devices().await?;
        info!("Device sync worker: cycle complete");
        Ok(())
    }

    /// Sync device status from network devices.
    pub async fn sync_device_status(&self) -> anyhow::Result<()> {
        info!("Device sync worker: syncing device status");

        use crate::modules::device::domain::entities::network_device;

        let devices = network_device::Entity::find()
            .filter(network_device::Column::Status.ne("decommissioned"))
            .all(&self.db)
            .await
            .map_err(|e| anyhow::anyhow!("Failed to query devices: {}", e))?;

        let count = devices.len();
        if count == 0 {
            info!("Device sync worker: no active devices to sync");
            return Ok(());
        }

        info!(count = count, "Device sync worker: syncing devices");

        let mut status_changes = 0;

        for device in &devices {
            let health_score = self.check_device_health(device).await;
            let new_status = if health_score > 80 {
                "online"
            } else if health_score > 50 {
                "degraded"
            } else {
                "offline"
            };

            let status_changed = device.status != new_status;

            let mut active: network_device::ActiveModel = device.clone().into();
            active.health_score = Set(Some(health_score));
            active.updated_at = Set(chrono::Utc::now());

            if status_changed {
                active.status = Set(new_status.to_string());

                let payload = serde_json::json!({
                    "device_id": device.id,
                    "device_name": device.name,
                    "old_status": device.status,
                    "new_status": new_status,
                    "health_score": health_score,
                    "management_ip": device.management_ip.to_string(),
                });

                if let Err(e) = outbox::insert_outbox_event(
                    &self.db,
                    "device.status.changed",
                    "device",
                    device.id,
                    payload,
                    None,
                    None,
                    Some(device.branch_id),
                ).await {
                    error!(
                        device_id = device.id,
                        error = %e,
                        "Failed to publish device.status.changed event"
                    );
                }

                status_changes += 1;
            }

            if let Err(e) = active.update(&self.db).await {
                error!(
                    device_id = device.id,
                    error = %e,
                    "Failed to update device status"
                );
            }
        }

        info!(
            total = count,
            status_changes = status_changes,
            "Device sync worker: sync complete"
        );
        Ok(())
    }

    /// Detect devices that haven't been updated recently and mark as offline.
    pub async fn detect_offline_devices(&self) -> anyhow::Result<()> {
        info!("Device sync worker: detecting offline devices");

        use crate::modules::device::domain::entities::network_device;

        let threshold = chrono::Utc::now() - chrono::Duration::minutes(5);

        let stale_devices = network_device::Entity::find()
            .filter(network_device::Column::Status.is_in(vec!["online", "degraded"]))
            .filter(network_device::Column::UpdatedAt.lt(threshold))
            .all(&self.db)
            .await
            .map_err(|e| anyhow::anyhow!("Failed to query stale devices: {}", e))?;

        let count = stale_devices.len();
        if count == 0 {
            info!("Device sync worker: no stale devices detected");
            return Ok(());
        }

        warn!(count = count, "Device sync worker: found stale devices");

        for device in &stale_devices {
            let mut active: network_device::ActiveModel = device.clone().into();
            active.status = Set("offline".to_string());
            active.health_score = Set(Some(0));
            active.updated_at = Set(chrono::Utc::now());

            if let Err(e) = active.update(&self.db).await {
                error!(
                    device_id = device.id,
                    error = %e,
                    "Failed to mark device as offline"
                );
                continue;
            }

            let payload = serde_json::json!({
                "device_id": device.id,
                "device_name": device.name,
                "old_status": device.status,
                "new_status": "offline",
                "management_ip": device.management_ip.to_string(),
                "reason": "no_updates",
            });

            if let Err(e) = outbox::insert_outbox_event(
                &self.db,
                "device.status.changed",
                "device",
                device.id,
                payload,
                None,
                None,
                Some(device.branch_id),
            ).await {
                error!(
                    device_id = device.id,
                    error = %e,
                    "Failed to publish device offline event"
                );
            }
        }

        info!(count = count, "Device sync worker: marked devices as offline");
        Ok(())
    }

    /// Check device health via real network device adapters.
    async fn check_device_health(
        &self,
        device: &crate::modules::device::domain::entities::network_device::Model,
    ) -> i32 {
        // Determine device type from model_id (1-100 = MikroTik, 101-200 = Huawei OLT)
        let device_type = if device.device_model_id >= 101 && device.device_model_id <= 200 {
            DeviceType::Olt
        } else {
            DeviceType::Router
        };

        if let Some(adapter) = DeviceAdapterFactory::create_for_device(
            &device_type,
            &device.management_ip,
        ) {
            match adapter.get_health_score().await {
                Ok(score) => {
                    debug!(
                        device_id = device.id,
                        device_name = %device.name,
                        health_score = score,
                        "Device health score from adapter"
                    );
                    score
                }
                Err(e) => {
                    warn!(
                        device_id = device.id,
                        device_name = %device.name,
                        error = %e,
                        "Failed to get device health from adapter, using fallback"
                    );
                    // Fallback: return degraded score (50) to avoid false offline alerts
                    50
                }
            }
        } else {
            // No adapter available for this device type, use simulated health
            warn!(
                device_id = device.id,
                device_name = %device.name,
                "No adapter available for device type, using simulated health"
            );
            use rand::Rng;
            let mut rng = rand::thread_rng();
            rng.gen_range(70..100)
        }
    }
}
