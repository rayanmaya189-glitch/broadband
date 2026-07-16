use chrono::Utc;
use sea_orm::{DatabaseConnection, EntityTrait, ActiveModelTrait, Set, QueryFilter, ColumnTrait};

use crate::modules::scheduler::domain::entities::{
    job_definition,
    JobDefinition, JobDefinitionActiveModel,
    JobExecutionActiveModel,
};

/// Background worker that checks for due jobs and triggers them.
pub struct SchedulerWorker {
    db: DatabaseConnection,
}

impl SchedulerWorker {
    pub fn new(db: DatabaseConnection) -> Self {
        Self { db }
    }

    /// Single cycle: find due jobs → start execution → complete/fail → update next_run_at
    pub async fn run_cycle(&self) -> Result<(), anyhow::Error> {
        let now = Utc::now();

        // Fetch active jobs whose next_run_at is due
        let due_jobs = JobDefinition::find()
            .filter(job_definition::Column::IsActive.eq(true))
            .filter(job_definition::Column::NextRunAt.lte(now))
            .all(&self.db)
            .await?;

        if due_jobs.is_empty() {
            return Ok(());
        }

        tracing::info!(count = due_jobs.len(), "Scheduler: processing due jobs");

        for job in due_jobs {
            let job_id = job.id;
            let job_name = job.name.clone();
            let target_module = job.target_module.clone();
            let action = job.action.clone();
            let payload = job.payload.clone();

            // Start execution
            let exec = JobExecutionActiveModel {
                job_definition_id: Set(job_id),
                status: Set("running".to_string()),
                input_payload: Set(payload.clone()),
                output_payload: Set(None),
                error_message: Set(None),
                duration_ms: Set(None),
                started_at: Set(now),
                completed_at: Set(None),
                ..Default::default()
            };
            let exec_model = match exec.insert(&self.db).await {
                Ok(e) => e,
                Err(e) => {
                    tracing::error!(job_id, error = %e, "Failed to create job execution");
                    continue;
                }
            };

            tracing::info!(job_id, job_name = %job_name, execution_id = exec_model.id, "Scheduler: executing job");

            // Execute the job
            let start = std::time::Instant::now();
            let result = self.execute_job(&target_module, &action, &payload).await;
            let duration = start.elapsed().as_millis() as i64;

            // Complete or fail the execution
            match result {
                Ok(output) => {
                    let mut active: JobExecutionActiveModel = exec_model.into();
                    active.status = Set("completed".to_string());
                    active.output_payload = Set(Some(output));
                    active.duration_ms = Set(Some(duration));
                    active.completed_at = Set(Some(Utc::now()));
                    if let Err(e) = active.update(&self.db).await {
                        tracing::error!(job_id, error = %e, "Failed to complete execution");
                    }

                    // Update parent job
                    Self::update_job_after_run(&self.db, job_id, "completed").await;
                }
                Err(e) => {
                    tracing::error!(job_id, error = %e, "Scheduler: job execution failed");

                    let mut active: JobExecutionActiveModel = exec_model.into();
                    active.status = Set("failed".to_string());
                    active.error_message = Set(Some(e.to_string()));
                    active.duration_ms = Set(Some(duration));
                    active.completed_at = Set(Some(Utc::now()));
                    if let Err(e2) = active.update(&self.db).await {
                        tracing::error!(job_id, error = %e2, "Failed to fail execution");
                    }

                    Self::update_job_after_run(&self.db, job_id, "failed").await;
                }
            }
        }

        Ok(())
    }

    /// Dispatch job to the target module's worker or event system.
    async fn execute_job(
        &self,
        target_module: &str,
        action: &str,
        _payload: &serde_json::Value,
    ) -> Result<serde_json::Value, anyhow::Error> {
        match target_module {
            "billing" => {
                tracing::info!(action = %action, "Scheduler: running billing worker cycle");
                let worker = crate::workers::billing_worker::BillingWorker::new(self.db.clone());
                worker.run_cycle().await?;
                Ok(serde_json::json!({
                    "module": "billing",
                    "action": action,
                    "status": "completed"
                }))
            }
            "notification" => {
                tracing::info!(action = %action, "Scheduler: running notification worker cycle");
                let worker = crate::workers::notification_worker::NotificationWorker::new(self.db.clone());
                worker.run_cycle().await?;
                Ok(serde_json::json!({
                    "module": "notification",
                    "action": action,
                    "status": "completed"
                }))
            }
            "device_sync" => {
                tracing::info!(action = %action, "Scheduler: running device sync worker cycle");
                let worker = crate::workers::device_sync_worker::DeviceSyncWorker::new(self.db.clone());
                worker.run_cycle().await?;
                Ok(serde_json::json!({
                    "module": "device_sync",
                    "action": action,
                    "status": "completed"
                }))
            }
            "bandwidth" => {
                tracing::info!(action = %action, "Scheduler: running bandwidth worker cycle");
                let worker = crate::workers::bandwidth_worker::BandwidthWorker::new(self.db.clone());
                worker.run_cycle().await?;
                Ok(serde_json::json!({
                    "module": "bandwidth",
                    "action": action,
                    "status": "completed"
                }))
            }
            "monitoring" => {
                tracing::info!(action = %action, "Scheduler: running monitoring worker cycle");
                let worker = crate::workers::monitoring_worker::MonitoringWorker::new(self.db.clone());
                worker.run_cycle().await?;
                Ok(serde_json::json!({
                    "module": "monitoring",
                    "action": action,
                    "status": "completed"
                }))
            }
            "cleanup" => {
                tracing::info!(action = %action, "Scheduler: running cleanup");
                // Cleanup outbox events older than 24 hours
                crate::infrastructure::messaging::outbox::cleanup_published_events(&self.db, 24).await?;
                Ok(serde_json::json!({
                    "module": "cleanup",
                    "action": action,
                    "status": "completed"
                }))
            }
            _ => {
                // Unknown module: publish a trigger event via outbox for other systems to handle
                tracing::info!(target_module = %target_module, action = %action, "Scheduler: publishing trigger event for unknown module");
                let event_payload = serde_json::json!({
                    "target_module": target_module,
                    "action": action,
                });
                crate::infrastructure::messaging::outbox::insert_outbox_event(
                    &self.db,
                    &format!("scheduler.job.triggered.{}", action),
                    "scheduler",
                    0,
                    event_payload,
                    None,
                    None,
                    None,
                ).await?;
                Ok(serde_json::json!({
                    "module": target_module,
                    "action": action,
                    "status": "event_published"
                }))
            }
        }
    }

    async fn update_job_after_run(db: &DatabaseConnection, job_id: i64, status: &str) {
        if let Ok(Some(job)) = JobDefinition::find_by_id(job_id).one(db).await {
            let mut active: JobDefinitionActiveModel = job.into();
            let now = Utc::now();
            active.last_run_at = Set(Some(now));
            active.last_run_status = Set(Some(status.to_string()));
            active.updated_at = Set(now);
            // Schedule next run: default to 1 hour from now if schedule is set
            active.next_run_at = Set(Some(now + chrono::Duration::hours(1)));
            let _ = active.update(db).await;
        }
    }
}
