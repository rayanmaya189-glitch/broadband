use chrono::Utc;
use sea_orm::{ActiveModelTrait, ColumnTrait, DatabaseConnection, EntityTrait, QueryFilter, Set};
use tokio::sync::Semaphore;
use tokio::task::JoinSet;

use crate::modules::scheduler::domain::entities::{
    job_definition, JobDefinition, JobDefinitionActiveModel, JobExecutionActiveModel,
};
use crate::modules::scheduler::domain::value_objects::Schedule;

/// Default timeout for worker execution (5 minutes)
const WORKER_TIMEOUT_SECS: u64 = 300;

/// Maximum concurrent workers to avoid overwhelming the DB pool
const MAX_CONCURRENT_WORKERS: usize = 5;

/// Background worker that checks for due jobs and triggers them concurrently.
pub struct SchedulerWorker {
    db: DatabaseConnection,
    semaphore: std::sync::Arc<Semaphore>,
}

impl SchedulerWorker {
    pub fn new(db: DatabaseConnection) -> Self {
        Self {
            db,
            semaphore: std::sync::Arc::new(Semaphore::new(MAX_CONCURRENT_WORKERS)),
        }
    }

    /// Single cycle: find due jobs → spawn concurrent executions → collect results
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

        tracing::info!(
            count = due_jobs.len(),
            "Scheduler: processing due jobs concurrently"
        );

        let mut join_set = JoinSet::new();

        for job in due_jobs {
            let job_id = job.id;
            let job_name = job.name.clone();
            let target_module = job.target_module.clone();
            let action = job.action.clone();
            let payload = job.payload.clone();

            // Start execution record
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

            tracing::info!(job_id, job_name = %job_name, execution_id = exec_model.id, "Scheduler: spawning job");

            // Clone for the spawned task
            let db = self.db.clone();
            let semaphore = self.semaphore.clone();

            // Spawn the job execution concurrently with timeout + semaphore
            join_set.spawn(async move {
                // Acquire semaphore permit to limit concurrency
                let _permit = semaphore.acquire().await.expect("semaphore closed");

                let start = std::time::Instant::now();

                // Wrap execution with timeout
                let result = tokio::time::timeout(
                    std::time::Duration::from_secs(WORKER_TIMEOUT_SECS),
                    execute_job(&db, &target_module, &action, &payload),
                ).await;

                let duration = start.elapsed().as_millis() as i64;

                match result {
                    Ok(Ok(output)) => {
                        let mut active: JobExecutionActiveModel = exec_model.into();
                        active.status = Set("completed".to_string());
                        active.output_payload = Set(Some(output));
                        active.duration_ms = Set(Some(duration));
                        active.completed_at = Set(Some(Utc::now()));
                        if let Err(e) = active.update(&db).await {
                            tracing::error!(job_id, error = %e, "Failed to complete execution");
                        }

                        update_job_after_run(&db, job_id, "completed").await;
                        tracing::info!(job_id, duration_ms = duration, "Scheduler: job completed");
                    }
                    Ok(Err(e)) => {
                        tracing::error!(job_id, error = %e, duration_ms = duration, "Scheduler: job execution failed");

                        let mut active: JobExecutionActiveModel = exec_model.into();
                        active.status = Set("failed".to_string());
                        active.error_message = Set(Some(e.to_string()));
                        active.duration_ms = Set(Some(duration));
                        active.completed_at = Set(Some(Utc::now()));
                        if let Err(e2) = active.update(&db).await {
                            tracing::error!(job_id, error = %e2, "Failed to fail execution");
                        }

                        update_job_after_run(&db, job_id, "failed").await;
                    }
                    Err(_timeout) => {
                        tracing::error!(job_id, timeout_secs = WORKER_TIMEOUT_SECS, "Scheduler: job timed out");

                        let mut active: JobExecutionActiveModel = exec_model.into();
                        active.status = Set("failed".to_string());
                        active.error_message = Set(Some(format!("Execution timed out after {} seconds", WORKER_TIMEOUT_SECS)));
                        active.duration_ms = Set(Some(duration));
                        active.completed_at = Set(Some(Utc::now()));
                        if let Err(e) = active.update(&db).await {
                            tracing::error!(job_id, error = %e, "Failed to record timeout");
                        }

                        update_job_after_run(&db, job_id, "failed").await;
                    }
                }

                job_id
            });
        }

        // Wait for all spawned tasks to complete
        while let Some(result) = join_set.join_next().await {
            if let Err(e) = result {
                tracing::error!(error = %e, "Scheduler: spawned task panicked");
            }
        }

        tracing::info!("Scheduler: all concurrent jobs completed");
        Ok(())
    }
}

/// Dispatch job to the target module's worker or event system.
async fn execute_job(
    db: &DatabaseConnection,
    target_module: &str,
    action: &str,
    _payload: &serde_json::Value,
) -> Result<serde_json::Value, anyhow::Error> {
    match target_module {
        "billing" => {
            tracing::info!(action = %action, "Scheduler: running billing worker cycle");
            let worker = crate::workers::billing_worker::BillingWorker::new(db.clone());
            worker.run_cycle().await?;
            Ok(serde_json::json!({
                "module": "billing",
                "action": action,
                "status": "completed"
            }))
        }
        "notification" => {
            tracing::info!(action = %action, "Scheduler: running notification worker cycle");
            let worker = crate::workers::notification_worker::NotificationWorker::new(db.clone());
            worker.run_cycle().await?;
            Ok(serde_json::json!({
                "module": "notification",
                "action": action,
                "status": "completed"
            }))
        }
        "device_sync" => {
            tracing::info!(action = %action, "Scheduler: running device sync worker cycle");
            let worker = crate::workers::device_sync_worker::DeviceSyncWorker::new(db.clone());
            worker.run_cycle().await?;
            Ok(serde_json::json!({
                "module": "device_sync",
                "action": action,
                "status": "completed"
            }))
        }
        "bandwidth" => {
            tracing::info!(action = %action, "Scheduler: running bandwidth worker cycle");
            let worker = crate::workers::bandwidth_worker::BandwidthWorker::new(db.clone());
            worker.run_cycle().await?;
            Ok(serde_json::json!({
                "module": "bandwidth",
                "action": action,
                "status": "completed"
            }))
        }
        "monitoring" => {
            tracing::info!(action = %action, "Scheduler: running monitoring worker cycle");
            let worker = crate::workers::monitoring_worker::MonitoringWorker::new(db.clone());
            worker.run_cycle().await?;
            Ok(serde_json::json!({
                "module": "monitoring",
                "action": action,
                "status": "completed"
            }))
        }
        "cleanup" => {
            tracing::info!(action = %action, "Scheduler: running cleanup");
            crate::infrastructure::messaging::outbox::cleanup_published_events(db, 24).await?;
            Ok(serde_json::json!({
                "module": "cleanup",
                "action": action,
                "status": "completed"
            }))
        }
        _ => {
            tracing::info!(target_module = %target_module, action = %action, "Scheduler: publishing trigger event for unknown module");
            let event_payload = serde_json::json!({
                "target_module": target_module,
                "action": action,
            });
            crate::infrastructure::messaging::outbox::insert_outbox_event(
                db,
                &format!("scheduler.job.triggered.{}", action),
                "scheduler",
                0,
                event_payload,
                None,
                None,
                None,
            )
            .await?;
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
        // Clone fields before moving job into ActiveModel
        let job_type = job.job_type.clone();
        let schedule_str = job.schedule.clone();
        let is_active = job.is_active;

        let mut active: JobDefinitionActiveModel = job.into();
        let now = Utc::now();
        active.last_run_at = Set(Some(now));
        active.last_run_status = Set(Some(status.to_string()));
        active.updated_at = Set(now);

        // Calculate next_run_at based on the job's schedule
        let next_run = if is_active {
            match Schedule::parse(&job_type, &schedule_str) {
                Ok(schedule) => schedule.next_run_after(now),
                Err(e) => {
                    tracing::warn!(
                        job_id,
                        job_type = %job_type,
                        schedule = %schedule_str,
                        error = %e,
                        "Scheduler: failed to parse schedule, disabling job"
                    );
                    active.is_active = Set(false);
                    None
                }
            }
        } else {
            None
        };
        active.next_run_at = Set(next_run);

        let _ = active.update(db).await;
    }
}
