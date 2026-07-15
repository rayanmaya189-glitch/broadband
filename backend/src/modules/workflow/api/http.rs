use axum::extract::State;
use axum::http::StatusCode;
use axum::Json;
use serde::{Deserialize, Serialize};
use std::sync::Arc;

use crate::shared::app_state::AppState;
use crate::shared::errors::AppError;
use crate::shared::middleware::auth::UserContext;
use crate::modules::workflow::application::services::WorkflowService;

#[derive(Debug, Serialize)]
pub struct WorkflowInstanceResponse {
    pub id: i64,
    pub workflow_type: String,
    pub reference_type: String,
    pub reference_id: i64,
    pub status: String,
    pub current_step: i32,
    pub total_steps: i32,
    pub error_message: Option<String>,
    pub started_at: String,
    pub completed_at: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct StartWorkflowRequest {
    pub workflow_type: String,
    pub reference_type: String,
    pub reference_id: i64,
    pub input_data: serde_json::Value,
    pub total_steps: i32,
}

#[derive(Debug, Deserialize)]
pub struct AdvanceWorkflowRequest {
    pub step_output: serde_json::Value,
}

#[derive(Debug, Deserialize)]
pub struct FailWorkflowRequest {
    pub error_message: String,
}

#[derive(Debug, Serialize)]
pub struct WorkflowStepResponse {
    pub id: i64,
    pub step_name: String,
    pub step_order: i32,
    pub target_module: String,
    pub action: String,
    pub status: String,
    pub retry_count: i32,
    pub error_message: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct AddStepRequest {
    pub step_name: String,
    pub step_order: i32,
    pub target_module: String,
    pub action: String,
    pub input_payload: serde_json::Value,
    #[serde(default)]
    pub compensation_action: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct CompleteStepRequest {
    pub output_payload: serde_json::Value,
}

#[derive(Debug, Deserialize)]
pub struct FailStepRequest {
    pub error_message: String,
}

pub async fn list_workflows(
    State(state): State<Arc<AppState>>,
    _user: UserContext,
) -> Result<Json<Vec<WorkflowInstanceResponse>>, AppError> {
    let instances = WorkflowService::list_instances(&state.db).await?;
    Ok(Json(instances.into_iter().map(|i| WorkflowInstanceResponse {
        id: i.id, workflow_type: i.workflow_type, reference_type: i.reference_type,
        reference_id: i.reference_id, status: i.status, current_step: i.current_step,
        total_steps: i.total_steps, error_message: i.error_message,
        started_at: i.started_at.to_rfc3339(),
        completed_at: i.completed_at.map(|c| c.to_rfc3339()),
    }).collect()))
}

pub async fn get_workflow(
    State(state): State<Arc<AppState>>,
    axum::extract::Path(id): axum::extract::Path<i64>,
    _user: UserContext,
) -> Result<Json<WorkflowInstanceResponse>, AppError> {
    let i = WorkflowService::get_instance(&state.db, id).await?;
    Ok(Json(WorkflowInstanceResponse {
        id: i.id, workflow_type: i.workflow_type, reference_type: i.reference_type,
        reference_id: i.reference_id, status: i.status, current_step: i.current_step,
        total_steps: i.total_steps, error_message: i.error_message,
        started_at: i.started_at.to_rfc3339(),
        completed_at: i.completed_at.map(|c| c.to_rfc3339()),
    }))
}

pub async fn start_workflow(
    State(state): State<Arc<AppState>>,
    user: UserContext,
    Json(req): Json<StartWorkflowRequest>,
) -> Result<(StatusCode, Json<WorkflowInstanceResponse>), AppError> {
    let i = WorkflowService::start_workflow(
        &state.db, req.workflow_type, req.reference_type, req.reference_id,
        req.input_data, req.total_steps, Some(user.id), None,
    ).await?;
    Ok((StatusCode::CREATED, Json(WorkflowInstanceResponse {
        id: i.id, workflow_type: i.workflow_type, reference_type: i.reference_type,
        reference_id: i.reference_id, status: i.status, current_step: i.current_step,
        total_steps: i.total_steps, error_message: i.error_message,
        started_at: i.started_at.to_rfc3339(),
        completed_at: i.completed_at.map(|c| c.to_rfc3339()),
    })))
}

pub async fn advance_workflow(
    State(state): State<Arc<AppState>>,
    axum::extract::Path(id): axum::extract::Path<i64>,
    _user: UserContext,
    Json(req): Json<AdvanceWorkflowRequest>,
) -> Result<Json<WorkflowInstanceResponse>, AppError> {
    let i = WorkflowService::advance_workflow(&state.db, id, req.step_output).await?;
    Ok(Json(WorkflowInstanceResponse {
        id: i.id, workflow_type: i.workflow_type, reference_type: i.reference_type,
        reference_id: i.reference_id, status: i.status, current_step: i.current_step,
        total_steps: i.total_steps, error_message: i.error_message,
        started_at: i.started_at.to_rfc3339(),
        completed_at: i.completed_at.map(|c| c.to_rfc3339()),
    }))
}

pub async fn fail_workflow(
    State(state): State<Arc<AppState>>,
    axum::extract::Path(id): axum::extract::Path<i64>,
    _user: UserContext,
    Json(req): Json<FailWorkflowRequest>,
) -> Result<Json<WorkflowInstanceResponse>, AppError> {
    let i = WorkflowService::fail_workflow(&state.db, id, req.error_message).await?;
    Ok(Json(WorkflowInstanceResponse {
        id: i.id, workflow_type: i.workflow_type, reference_type: i.reference_type,
        reference_id: i.reference_id, status: i.status, current_step: i.current_step,
        total_steps: i.total_steps, error_message: i.error_message,
        started_at: i.started_at.to_rfc3339(),
        completed_at: i.completed_at.map(|c| c.to_rfc3339()),
    }))
}

pub async fn cancel_workflow(
    State(state): State<Arc<AppState>>,
    axum::extract::Path(id): axum::extract::Path<i64>,
    _user: UserContext,
) -> Result<StatusCode, AppError> {
    WorkflowService::cancel_workflow(&state.db, id).await?;
    Ok(StatusCode::OK)
}

pub async fn compensate_workflow(
    State(state): State<Arc<AppState>>,
    axum::extract::Path(id): axum::extract::Path<i64>,
    _user: UserContext,
) -> Result<Json<WorkflowInstanceResponse>, AppError> {
    let i = WorkflowService::start_compensation(&state.db, id).await?;
    Ok(Json(WorkflowInstanceResponse {
        id: i.id, workflow_type: i.workflow_type, reference_type: i.reference_type,
        reference_id: i.reference_id, status: i.status, current_step: i.current_step,
        total_steps: i.total_steps, error_message: i.error_message,
        started_at: i.started_at.to_rfc3339(),
        completed_at: i.completed_at.map(|c| c.to_rfc3339()),
    }))
}

// ── Steps ──

pub async fn list_steps(
    State(state): State<Arc<AppState>>,
    axum::extract::Path(workflow_id): axum::extract::Path<i64>,
    _user: UserContext,
) -> Result<Json<Vec<WorkflowStepResponse>>, AppError> {
    let steps = WorkflowService::get_steps_for_workflow(&state.db, workflow_id).await?;
    Ok(Json(steps.into_iter().map(|s| WorkflowStepResponse {
        id: s.id, step_name: s.step_name, step_order: s.step_order,
        target_module: s.target_module, action: s.action, status: s.status,
        retry_count: s.retry_count, error_message: s.error_message,
    }).collect()))
}

pub async fn add_step(
    State(state): State<Arc<AppState>>,
    axum::extract::Path(workflow_id): axum::extract::Path<i64>,
    _user: UserContext,
    Json(req): Json<AddStepRequest>,
) -> Result<(StatusCode, Json<WorkflowStepResponse>), AppError> {
    let s = WorkflowService::add_step(
        &state.db, workflow_id, req.step_name, req.step_order,
        req.target_module, req.action, req.input_payload, req.compensation_action,
    ).await?;
    Ok((StatusCode::CREATED, Json(WorkflowStepResponse {
        id: s.id, step_name: s.step_name, step_order: s.step_order,
        target_module: s.target_module, action: s.action, status: s.status,
        retry_count: s.retry_count, error_message: s.error_message,
    })))
}

pub async fn complete_step(
    State(state): State<Arc<AppState>>,
    axum::extract::Path(step_id): axum::extract::Path<i64>,
    _user: UserContext,
    Json(req): Json<CompleteStepRequest>,
) -> Result<Json<WorkflowStepResponse>, AppError> {
    let s = WorkflowService::complete_step(&state.db, step_id, req.output_payload).await?;
    Ok(Json(WorkflowStepResponse {
        id: s.id, step_name: s.step_name, step_order: s.step_order,
        target_module: s.target_module, action: s.action, status: s.status,
        retry_count: s.retry_count, error_message: s.error_message,
    }))
}

pub async fn fail_step(
    State(state): State<Arc<AppState>>,
    axum::extract::Path(step_id): axum::extract::Path<i64>,
    _user: UserContext,
    Json(req): Json<FailStepRequest>,
) -> Result<Json<WorkflowStepResponse>, AppError> {
    let s = WorkflowService::fail_step(&state.db, step_id, req.error_message).await?;
    Ok(Json(WorkflowStepResponse {
        id: s.id, step_name: s.step_name, step_order: s.step_order,
        target_module: s.target_module, action: s.action, status: s.status,
        retry_count: s.retry_count, error_message: s.error_message,
    }))
}

pub async fn get_workflow_stats(
    State(state): State<Arc<AppState>>,
    _user: UserContext,
) -> Result<Json<serde_json::Value>, AppError> {
    Ok(Json(WorkflowService::get_instance_stats(&state.db).await?))
}
