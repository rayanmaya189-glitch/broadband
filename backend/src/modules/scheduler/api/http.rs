use axum::extract::{Path, Query, State};
use axum::http::StatusCode;
use axum::Json;
use serde::Deserialize;
use std::sync::Arc;

use crate::modules::scheduler::application::services::SchedulerService;
use crate::shared::app_state::AppState;
use crate::shared::errors::AppError;
use crate::shared::middleware::auth::{require_permission, UserContext};
use crate::shared::primitives::PaginationParams;

// ── Request Types ──

#[derive(Debug, Deserialize)]
pub struct CreateJobRequest {
    pub name: String,
    pub description: Option<String>,
    pub job_type: String,
    pub schedule: String,
    pub target_module: String,
    pub action: String,
    pub payload: Option<serde_json::Value>,
    pub timeout_seconds: Option<i32>,
}

#[derive(Debug, Deserialize)]
pub struct UpdateJobRequest {
    pub schedule: Option<String>,
    pub payload: Option<serde_json::Value>,
    pub is_active: Option<bool>,
    pub timeout_seconds: Option<i32>,
}

#[derive(Debug, Deserialize)]
pub struct ExecutionsQuery {
    pub job_id: Option<i64>,
}

// ── Handlers ──

/// GET /api/v1/scheduler/jobs
pub async fn list_jobs(
    State(state): State<Arc<AppState>>,
    _user: UserContext,
    Query(_p): Query<PaginationParams>,
) -> Result<Json<serde_json::Value>, AppError> {
    let jobs = SchedulerService::list_job_definitions(&state.db).await?;
    Ok(Json(serde_json::json!({ "items": jobs, "total": jobs.len() })))
}

/// GET /api/v1/scheduler/jobs/:id
pub async fn get_job(
    State(state): State<Arc<AppState>>,
    _user: UserContext,
    Path(id): Path<i64>,
) -> Result<Json<serde_json::Value>, AppError> {
    let job = SchedulerService::get_job_definition(&state.db, id).await?;
    Ok(Json(serde_json::to_value(job).unwrap_or_default()))
}

/// POST /api/v1/scheduler/jobs
pub async fn create_job(
    State(state): State<Arc<AppState>>,
    user: UserContext,
    Json(req): Json<CreateJobRequest>,
) -> Result<(StatusCode, Json<serde_json::Value>), AppError> {
    require_permission(&user, "scheduler.job.create").map_err(|e| AppError::Forbidden(e.1))?;
    let job = SchedulerService::create_job_definition(
        &state.db,
        req.name,
        req.description,
        req.job_type,
        req.schedule,
        req.target_module,
        req.action,
        req.payload.unwrap_or(serde_json::json!({})),
        req.timeout_seconds,
    ).await?;
    Ok((StatusCode::CREATED, Json(serde_json::to_value(job).unwrap_or_default())))
}

/// PUT /api/v1/scheduler/jobs/:id
pub async fn update_job(
    State(state): State<Arc<AppState>>,
    user: UserContext,
    Path(id): Path<i64>,
    Json(req): Json<UpdateJobRequest>,
) -> Result<Json<serde_json::Value>, AppError> {
    require_permission(&user, "scheduler.job.update").map_err(|e| AppError::Forbidden(e.1))?;
    let job = SchedulerService::update_job_definition(
        &state.db, id, req.schedule, req.payload, req.is_active, req.timeout_seconds,
    ).await?;
    Ok(Json(serde_json::to_value(job).unwrap_or_default()))
}

/// DELETE /api/v1/scheduler/jobs/:id
pub async fn delete_job(
    State(state): State<Arc<AppState>>,
    user: UserContext,
    Path(id): Path<i64>,
) -> Result<StatusCode, AppError> {
    require_permission(&user, "scheduler.job.delete").map_err(|e| AppError::Forbidden(e.1))?;
    SchedulerService::delete_job_definition(&state.db, id).await?;
    Ok(StatusCode::NO_CONTENT)
}

/// POST /api/v1/scheduler/jobs/:id/trigger
pub async fn trigger_job(
    State(state): State<Arc<AppState>>,
    user: UserContext,
    Path(id): Path<i64>,
) -> Result<(StatusCode, Json<serde_json::Value>), AppError> {
    require_permission(&user, "scheduler.job.trigger").map_err(|e| AppError::Forbidden(e.1))?;
    let job = SchedulerService::get_job_definition(&state.db, id).await?;
    let execution = SchedulerService::start_execution(
        &state.db, job.id, serde_json::json!({ "triggered_by": user.user_id }),
    ).await?;
    Ok((StatusCode::CREATED, Json(serde_json::to_value(execution).unwrap_or_default())))
}

/// GET /api/v1/scheduler/executions
pub async fn list_executions(
    State(state): State<Arc<AppState>>,
    _user: UserContext,
    Query(q): Query<ExecutionsQuery>,
) -> Result<Json<serde_json::Value>, AppError> {
    let jobs = SchedulerService::list_executions(&state.db, q.job_id).await?;
    Ok(Json(serde_json::json!({ "items": jobs, "total": jobs.len() })))
}

/// GET /api/v1/scheduler/stats
pub async fn scheduler_stats(
    State(state): State<Arc<AppState>>,
    _user: UserContext,
) -> Result<Json<serde_json::Value>, AppError> {
    let stats = SchedulerService::get_scheduler_stats(&state.db).await?;
    Ok(Json(stats))
}
