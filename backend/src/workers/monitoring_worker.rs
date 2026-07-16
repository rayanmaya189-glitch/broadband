use sea_orm::{DatabaseConnection, EntityTrait, QueryFilter, ColumnTrait};
use tracing::{info, warn, error};

use crate::infrastructure::messaging::outbox;
use crate::modules::device::domain::entities::network_device;
use crate::modules::monitoring::domain::entities::metric_record;
use crate::modules::monitoring::domain::entities::monitoring_alert;
use crate::modules::monitoring::domain::rules::monitoring_rules;

/// Background worker for device health monitoring and alerting:
/// - Collect device metrics via adapters
/// - Evaluate alert rules
/// - Create/resolve alerts based on thresholds
/// - Publish monitoring events
pub struct MonitoringWorker {
    db: DatabaseConnection,
}

impl MonitoringWorker {
    pub fn new(db: DatabaseConnection) -> Self {
        Self { db }
    }

    /// Run the full monitoring worker cycle
    pub async fn run_cycle(&self) -> anyhow::Result<()> {
        info!("Monitoring worker: starting cycle");
        self.collect_device_metrics().await?;
        self.evaluate_alert_rules().await?;
        self.cleanup_old_data().await?;
        info!("Monitoring worker: cycle complete");
        Ok(())
    }

    /// Collect metrics from all active devices
    async fn collect_device_metrics(&self) -> anyhow::Result<()> {
        info!("Monitoring worker: collecting device metrics");

        let devices = network_device::Entity::find()
            .filter(network_device::Column::Status.ne("decommissioned"))
            .all(&self.db)
            .await
            .map_err(|e| anyhow::anyhow!("Failed to query devices: {}", e))?;

        let count = devices.len();
        if count == 0 {
            info!("Monitoring worker: no active devices to monitor");
            return Ok(());
        }

        info!(count = count, "Monitoring worker: collecting metrics from devices");

        for device in &devices {
            match self.collect_single_device_metrics(device).await {
                Ok(metrics) => {
                    info!(
                        device_id = device.id,
                        device_name = %device.name,
                        metrics_count = metrics.len(),
                        "Collected device metrics"
                    );
                }
                Err(e) => {
                    warn!(
                        device_id = device.id,
                        device_name = %device.name,
                        error = %e,
                        "Failed to collect device metrics"
                    );
                }
            }
        }

        Ok(())
    }

    /// Collect metrics from a single device
    async fn collect_single_device_metrics(
        &self,
        device: &network_device::Model,
    ) -> anyhow::Result<Vec<serde_json::Value>> {
        use crate::modules::integrations::{DeviceAdapterFactory, DeviceType};

        let device_type = if device.device_model_id >= 101 && device.device_model_id <= 200 {
            DeviceType::Olt
        } else {
            DeviceType::Router
        };

        let adapter = DeviceAdapterFactory::create_for_device(
            &device_type,
            &device.management_ip,
        );

        let mut metrics = Vec::new();

        if let Some(adapter) = adapter {
            // Get health score from adapter
            match adapter.get_health_score().await {
                Ok(health_score) => {
                    let metric = serde_json::json!({
                        "device_id": device.id,
                        "branch_id": device.branch_id,
                        "metric_name": "device_health",
                        "metric_value": health_score,
                        "unit": "score",
                        "recorded_at": chrono::Utc::now(),
                    });

                    // Save metric record
                    let _record = metric_record::Model {
                        id: 0,
                        device_id: device.id,
                        branch_id: device.branch_id,
                        metric_name: "device_health".to_string(),
                        metric_value: health_score as f64,
                        unit: Some("score".to_string()),
                        tags: None,
                        recorded_at: chrono::Utc::now(),
                        created_at: chrono::Utc::now(),
                    };

                    // TODO: Save to database
                    metrics.push(metric);

                    // Check if health score requires alert
                    if health_score < monitoring_rules::HEALTH_SCORE_WARNING {
                        self.create_health_alert(device, health_score).await?;
                    }
                }
                Err(e) => {
                    warn!(
                        device_id = device.id,
                        error = %e,
                        "Failed to get device health score"
                    );
                }
            }
        }

        Ok(metrics)
    }

    /// Create an alert for low health score
    async fn create_health_alert(
        &self,
        device: &network_device::Model,
        health_score: i32,
    ) -> anyhow::Result<()> {
        // Check if there's already an active alert for this device
        let existing_alerts = monitoring_alert::Entity::find()
            .filter(monitoring_alert::Column::DeviceId.eq(device.id))
            .filter(monitoring_alert::Column::Status.is_in(vec!["active", "acknowledged"]))
            .filter(monitoring_alert::Column::AlertType.eq("device_health"))
            .all(&self.db)
            .await
            .map_err(|e| anyhow::anyhow!("Failed to query existing alerts: {}", e))?;

        if !existing_alerts.is_empty() {
            return Ok(()); // Alert already exists
        }

        let severity = if health_score < monitoring_rules::HEALTH_SCORE_CRITICAL {
            "critical"
        } else if health_score < monitoring_rules::HEALTH_SCORE_WARNING {
            "high"
        } else {
            "medium"
        };

        let _alert = monitoring_alert::Model {
            id: 0,
            device_id: device.id,
            branch_id: device.branch_id,
            alert_rule_id: None,
            alert_type: "device_health".to_string(),
            severity: severity.to_string(),
            status: "active".to_string(),
            title: format!("Device health degraded: {}", device.name),
            message: format!(
                "Device {} has health score {} (threshold: {})",
                device.name, health_score, monitoring_rules::HEALTH_SCORE_WARNING
            ),
            metric_name: Some("device_health".to_string()),
            metric_value: Some(health_score as f64),
            threshold_value: Some(monitoring_rules::HEALTH_SCORE_WARNING as f64),
            acknowledged_by: None,
            acknowledged_at: None,
            resolved_by: None,
            resolved_at: None,
            resolution_notes: None,
            created_at: chrono::Utc::now(),
            updated_at: chrono::Utc::now(),
        };

        // TODO: Save alert to database
        info!(
            device_id = device.id,
            health_score,
            severity,
            "Created health alert"
        );

        // Publish alert event
        let payload = serde_json::json!({
            "device_id": device.id,
            "device_name": device.name,
            "health_score": health_score,
            "severity": severity,
            "alert_type": "device_health",
        });

        if let Err(e) = outbox::insert_outbox_event(
            &self.db,
            "monitoring.alert.created",
            "monitoring_alert",
            0, // Will be set by outbox
            payload,
            None,
            None,
            Some(device.branch_id),
        ).await {
            error!(
                device_id = device.id,
                error = %e,
                "Failed to publish monitoring.alert.created event"
            );
        }

        Ok(())
    }

    /// Evaluate alert rules and create/resolve alerts
    async fn evaluate_alert_rules(&self) -> anyhow::Result<()> {
        info!("Monitoring worker: evaluating alert rules");

        // Get all active alerts
        let active_alerts = monitoring_alert::Entity::find()
            .filter(monitoring_alert::Column::Status.is_in(vec!["active", "acknowledged"]))
            .all(&self.db)
            .await
            .map_err(|e| anyhow::anyhow!("Failed to query active alerts: {}", e))?;

        // Check if any alerts should be auto-resolved
        for alert in &active_alerts {
            if let Some(device) = network_device::Entity::find_by_id(alert.device_id)
                .one(&self.db)
                .await
                .map_err(|e| anyhow::anyhow!("Failed to query device: {}", e))?
            {
                // If device is back online and healthy, auto-resolve the alert
                if device.status == "online" {
                    if let Some(health_score) = device.health_score {
                        if health_score >= monitoring_rules::HEALTH_SCORE_GOOD {
                            info!(
                                alert_id = alert.id,
                                device_id = device.id,
                                health_score,
                                "Auto-resolving alert - device healthy"
                            );
                            // TODO: Update alert status to auto_resolved
                        }
                    }
                }
            }
        }

        Ok(())
    }

    /// Cleanup old metrics and alerts
    async fn cleanup_old_data(&self) -> anyhow::Result<()> {
        info!("Monitoring worker: cleaning up old data");

        let metric_cutoff = chrono::Utc::now() - chrono::Duration::days(90);
        let alert_cutoff = chrono::Utc::now() - chrono::Duration::hours(monitoring_rules::ALERT_EXPIRY_HOURS);

        // Delete old metrics
        let metrics_deleted = metric_record::Entity::delete_many()
            .filter(metric_record::Column::RecordedAt.lt(metric_cutoff))
            .exec(&self.db)
            .await
            .map_err(|e| anyhow::anyhow!("Failed to delete old metrics: {}", e))?;

        // Delete expired alerts
        let alerts_deleted = monitoring_alert::Entity::delete_many()
            .filter(monitoring_alert::Column::Status.eq("expired"))
            .filter(monitoring_alert::Column::CreatedAt.lt(alert_cutoff))
            .exec(&self.db)
            .await
            .map_err(|e| anyhow::anyhow!("Failed to delete expired alerts: {}", e))?;

        info!(
            metrics_deleted = metrics_deleted.rows_affected,
            alerts_deleted = alerts_deleted.rows_affected,
            "Monitoring worker: cleanup complete"
        );

        Ok(())
    }
}
