/// OpenAPI schemas and stub handlers for Scheduler endpoints.
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

// ── Request / Response types ─────────────────────────────────────────

#[derive(Debug, Deserialize, ToSchema)]
pub struct CreateJobRequest {
    /// Job name
    pub name: String,
    /// Job description
    #[serde(default)]
    pub description: Option<String>,
    /// Job type (e.g. "cron", "once", "interval")
    pub job_type: String,
    /// Schedule expression (cron or interval)
    pub schedule: String,
    /// Target module to execute action on
    pub target_module: String,
    /// Action to perform
    pub action: String,
    /// Optional JSON payload
    #[serde(default)]
    pub payload: Option<serde_json::Value>,
    /// Timeout in seconds
    #[serde(default)]
    pub timeout_seconds: Option<i32>,
}

#[derive(Debug, Deserialize, ToSchema)]
pub struct UpdateJobRequest {
    /// Updated schedule expression
    #[serde(default)]
    pub schedule: Option<String>,
    /// Updated payload
    #[serde(default)]
    pub payload: Option<serde_json::Value>,
    /// Whether the job is active
    #[serde(default)]
    pub is_active: Option<bool>,
    /// Updated timeout in seconds
    #[serde(default)]
    pub timeout_seconds: Option<i32>,
}

#[derive(Debug, Serialize, ToSchema)]
pub struct JobDefinitionResponse {
    /// Job ID
    pub id: i64,
    /// Job name
    pub name: String,
    /// Job description
    pub description: Option<String>,
    /// Job type
    pub job_type: String,
    /// Schedule expression
    pub schedule: String,
    /// Target module
    pub target_module: String,
    /// Action
    pub action: String,
    /// JSON payload
    pub payload: serde_json::Value,
    /// Whether job is active
    pub is_active: bool,
    /// Timeout in seconds
    pub timeout_seconds: Option<i32>,
    /// Created at timestamp
    pub created_at: String,
    /// Updated at timestamp
    pub updated_at: String,
}

#[derive(Debug, Serialize, ToSchema)]
pub struct JobExecutionResponse {
    /// Execution ID
    pub id: i64,
    /// Job definition ID
    pub job_id: i64,
    /// Execution status (e.g. "running", "completed", "failed")
    pub status: String,
    /// Input payload
    pub input: serde_json::Value,
    /// Output result
    pub output: Option<serde_json::Value>,
    /// Error message if failed
    pub error: Option<String>,
    /// Started at timestamp
    pub started_at: String,
    /// Completed at timestamp
    pub completed_at: Option<String>,
}

#[derive(Debug, Serialize, ToSchema)]
pub struct SchedulerStatsResponse {
    /// Total job definitions
    pub total_jobs: i64,
    /// Active jobs
    pub active_jobs: i64,
    /// Total executions
    pub total_executions: i64,
    /// Successful executions
    pub successful_executions: i64,
    /// Failed executions
    pub failed_executions: i64,
}

#[derive(Debug, Deserialize, ToSchema)]
pub struct ExecutionsQuery {
    /// Filter by job ID
    #[serde(default)]
    pub job_id: Option<i64>,
}

// ── Stub handler functions (for OpenAPI spec only) ───────────────────

/// List all scheduler jobs
#[utoipa::path(
    get,
    path = "/api/v1/scheduler/jobs",
    tag = "Scheduler",
    params(("page" = Option<u64>, Query, description = "Page number"),
           ("limit" = Option<u64>, Query, description = "Items per page")),
    responses(
        (status = 200, description = "List of scheduler jobs")
    ),
    security(("bearer_auth" = []))
)]
pub async fn list_jobs() -> axum::Json<serde_json::Value> {
    unimplemented!()
}

/// Get a specific scheduler job by ID
#[utoipa::path(
    get,
    path = "/api/v1/scheduler/jobs/{id}",
    tag = "Scheduler",
    params(("id" = i64, Path, description = "Job ID")),
    responses(
        (status = 200, description = "Job details", body = JobDefinitionResponse),
        (status = 404, description = "Job not found")
    ),
    security(("bearer_auth" = []))
)]
pub async fn get_job() -> axum::Json<serde_json::Value> {
    unimplemented!()
}

/// Create a new scheduler job
#[utoipa::path(
    post,
    path = "/api/v1/scheduler/jobs",
    tag = "Scheduler",
    request_body = CreateJobRequest,
    responses(
        (status = 201, description = "Job created", body = JobDefinitionResponse),
        (status = 403, description = "Forbidden"),
        (status = 422, description = "Validation error")
    ),
    security(("bearer_auth" = []))
)]
pub async fn create_job() -> axum::Json<serde_json::Value> {
    unimplemented!()
}

/// Update a scheduler job
#[utoipa::path(
    put,
    path = "/api/v1/scheduler/jobs/{id}",
    tag = "Scheduler",
    params(("id" = i64, Path, description = "Job ID")),
    request_body = UpdateJobRequest,
    responses(
        (status = 200, description = "Job updated", body = JobDefinitionResponse),
        (status = 403, description = "Forbidden"),
        (status = 404, description = "Job not found")
    ),
    security(("bearer_auth" = []))
)]
pub async fn update_job() -> axum::Json<serde_json::Value> {
    unimplemented!()
}

/// Delete a scheduler job
#[utoipa::path(
    delete,
    path = "/api/v1/scheduler/jobs/{id}",
    tag = "Scheduler",
    params(("id" = i64, Path, description = "Job ID")),
    responses(
        (status = 204, description = "Job deleted"),
        (status = 403, description = "Forbidden"),
        (status = 404, description = "Job not found")
    ),
    security(("bearer_auth" = []))
)]
pub async fn delete_job() -> axum::Json<serde_json::Value> {
    unimplemented!()
}

/// Trigger immediate execution of a scheduler job
#[utoipa::path(
    post,
    path = "/api/v1/scheduler/jobs/{id}/trigger",
    tag = "Scheduler",
    params(("id" = i64, Path, description = "Job ID")),
    responses(
        (status = 201, description = "Job triggered", body = JobExecutionResponse),
        (status = 403, description = "Forbidden"),
        (status = 404, description = "Job not found")
    ),
    security(("bearer_auth" = []))
)]
pub async fn trigger_job() -> axum::Json<serde_json::Value> {
    unimplemented!()
}

/// List job executions
#[utoipa::path(
    get,
    path = "/api/v1/scheduler/executions",
    tag = "Scheduler",
    params(("job_id" = Option<i64>, Query, description = "Filter by job ID")),
    responses(
        (status = 200, description = "List of executions")
    ),
    security(("bearer_auth" = []))
)]
pub async fn list_executions() -> axum::Json<serde_json::Value> {
    unimplemented!()
}

/// Get scheduler statistics
#[utoipa::path(
    get,
    path = "/api/v1/scheduler/stats",
    tag = "Scheduler",
    responses(
        (status = 200, description = "Scheduler statistics", body = SchedulerStatsResponse)
    ),
    security(("bearer_auth" = []))
)]
pub async fn scheduler_stats() -> axum::Json<serde_json::Value> {
    unimplemented!()
}
