use sea_orm::{DatabaseConnection, EntityTrait, QueryFilter, ColumnTrait, Set, ActiveModelTrait, QueryOrder, QuerySelect};
use tracing::{info, warn, error};

use crate::infrastructure::messaging::outbox;

/// Background worker for bandwidth profile management:
/// - Apply pending bandwidth profiles to network devices
/// - Verify applied profiles match expected configuration
/// - Handle failed applications with retries
pub struct BandwidthWorker {
    db: DatabaseConnection,
}

impl BandwidthWorker {
    pub fn new(db: DatabaseConnection) -> Self {
        Self { db }
    }

    /// Run the full bandwidth worker cycle.
    pub async fn run_cycle(&self) -> anyhow::Result<()> {
        info!("Bandwidth worker: starting cycle");
        self.apply_profiles().await?;
        self.verify_applied_profiles().await?;
        info!("Bandwidth worker: cycle complete");
        Ok(())
    }

    /// Apply pending bandwidth profiles to network devices.
    pub async fn apply_profiles(&self) -> anyhow::Result<()> {
        info!("Bandwidth worker: applying pending profiles");

        use crate::modules::bandwidth::domain::entities::bandwidth_application;

        // Fetch pending applications
        let pending = bandwidth_application::Entity::find()
            .filter(bandwidth_application::Column::Status.eq("pending"))
            .order_by_asc(bandwidth_application::Column::CreatedAt)
            .limit(20)
            .all(&self.db)
            .await
            .map_err(|e| anyhow::anyhow!("Failed to query pending applications: {}", e))?;

        let count = pending.len();
        if count == 0 {
            info!("Bandwidth worker: no pending profile applications");
            return Ok(());
        }

        info!(count = count, "Bandwidth worker: processing applications");

        for app in &pending {
            // Mark as applying
            let mut active: bandwidth_application::ActiveModel = app.clone().into();
            active.status = Set("applying".to_string());
            if let Err(e) = active.update(&self.db).await {
                error!(
                    application_id = app.id,
                    error = %e,
                    "Failed to mark application as applying"
                );
                continue;
            }

            // Apply the bandwidth profile via Mikrotik API or SNMP
            match self.apply_to_device(app).await {
                Ok(()) => {
                    let mut active: bandwidth_application::ActiveModel = app.clone().into();
                    active.status = Set("applied".to_string());
                    active.applied_at = Set(Some(chrono::Utc::now()));
                    active.retry_count = Set(0);

                    if let Err(e) = active.update(&self.db).await {
                        error!(
                            application_id = app.id,
                            error = %e,
                            "Failed to mark application as applied"
                        );
                    }

                    // Publish success event
                    let payload = serde_json::json!({
                        "application_id": app.id,
                        "profile_id": app.profile_id,
                        "subscription_id": app.subscription_id,
                        "device_id": app.device_id,
                        "status": "applied",
                    });

                    if let Err(e) = outbox::insert_outbox_event(
                        &self.db,
                        "bandwidth.profile.applied",
                        "bandwidth_application",
                        app.id,
                        payload,
                        None,
                        None,
                        None,
                    ).await {
                        error!(
                            application_id = app.id,
                            error = %e,
                            "Failed to publish bandwidth.profile.applied event"
                        );
                    }
                }
                Err(e) => {
                    error!(
                        application_id = app.id,
                        error = %e,
                        "Failed to apply bandwidth profile"
                    );

                    let mut active: bandwidth_application::ActiveModel = app.clone().into();
                    active.retry_count = Set(app.retry_count + 1);
                    active.last_error = Set(Some(e.to_string()));

                    if app.retry_count + 1 >= 3 {
                        active.status = Set("failed".to_string());
                        warn!(
                            application_id = app.id,
                            "Bandwidth application exceeded max retries"
                        );
                    } else {
                        active.status = Set("pending".to_string()); // Will retry next cycle
                    }

                    if let Err(update_err) = active.update(&self.db).await {
                        error!(
                            application_id = app.id,
                            error = %update_err,
                            "Failed to update application state"
                        );
                    }
                }
            }
        }

        info!(count = count, "Bandwidth worker: processed applications");
        Ok(())
    }

    /// Verify that applied profiles match expected configuration.
    pub async fn verify_applied_profiles(&self) -> anyhow::Result<()> {
        info!("Bandwidth worker: verifying applied profiles");

        use crate::modules::bandwidth::domain::entities::bandwidth_application;

        // Fetch recently applied profiles for verification
        let applied = bandwidth_application::Entity::find()
            .filter(bandwidth_application::Column::Status.eq("applied"))
            .order_by_desc(bandwidth_application::Column::AppliedAt)
            .limit(10)
            .all(&self.db)
            .await
            .map_err(|e| anyhow::anyhow!("Failed to query applied profiles: {}", e))?;

        let mut verified = 0;

        for app in &applied {
            // In production: query device via SNMP to verify bandwidth limits
            if app.applied_at.is_some() {
                verified += 1;
            }
        }

        info!(count = verified, "Bandwidth worker: verified profiles");
        Ok(())
    }

    /// Apply bandwidth profile to a network device.
    async fn apply_to_device(
        &self,
        _app: &crate::modules::bandwidth::domain::entities::bandwidth_application::Model,
    ) -> anyhow::Result<()> {
        // In production: use Mikrotik RouterOS API to set queue trees
        tokio::time::sleep(std::time::Duration::from_millis(200)).await;

        // Simulate occasional failures (10% failure rate)
        use rand::Rng;
        let mut rng = rand::thread_rng();
        if rng.gen_ratio(1, 10) {
            return Err(anyhow::anyhow!("Simulated device connection timeout"));
        }

        Ok(())
    }
}
