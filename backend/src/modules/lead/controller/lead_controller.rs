//! SeaORM-based controller for the Lead domain.

use axum::extract::{Json, Path, Query, State};
use validator::Validate;

use crate::app::SharedState;
use crate::common::errors::app_error::AppError;
use crate::modules::lead::request::lead_request::*;
use crate::modules::lead::response::lead_response::*;
use crate::modules::lead::service::lead_service::LeadService;

pub async fn list(State(state): State<SharedState>, Query(q): Query<LeadQuery>) -> Result<Json<Vec<LeadResponse>>, AppError> {
    let svc = LeadService::new(&state.db_seaorm);
    let (leads, _) = svc.list(q.branch_id, q.status.as_deref(), q.source.as_deref(), q.assigned_to, q.page.unwrap_or(1), q.per_page.unwrap_or(20)).await?;
    Ok(Json(leads))
}

pub async fn get_by_id(State(state): State<SharedState>, Path(id): Path<i64>) -> Result<Json<LeadResponse>, AppError> {
    let svc = LeadService::new(&state.db_seaorm);
    Ok(Json(svc.get_by_id(id).await?))
}

pub async fn create(State(state): State<SharedState>, Json(req): Json<CreateLeadRequest>) -> Result<Json<LeadResponse>, AppError> {
    req.validate()?;
    let svc = LeadService::new(&state.db_seaorm);
    Ok(Json(svc.create(req.branch_id, req).await?))
}

pub async fn update(State(state): State<SharedState>, Path(id): Path<i64>, Json(req): Json<UpdateLeadRequest>) -> Result<Json<LeadResponse>, AppError> {
    let svc = LeadService::new(&state.db_seaorm);
    Ok(Json(svc.update(id, req).await?))
}

pub async fn update_status(State(state): State<SharedState>, Path(id): Path<i64>, Json(req): Json<UpdateLeadStatusRequest>) -> Result<Json<LeadResponse>, AppError> {
    let svc = LeadService::new(&state.db_seaorm);
    Ok(Json(svc.update_status(id, &req.status, req.lost_reason.as_deref()).await?))
}

pub async fn assign(State(state): State<SharedState>, Path(id): Path<i64>, Json(req): Json<AssignLeadRequest>) -> Result<Json<LeadResponse>, AppError> {
    let svc = LeadService::new(&state.db_seaorm);
    Ok(Json(svc.assign(id, req.assigned_to).await?))
}

pub async fn convert(State(state): State<SharedState>, Path(id): Path<i64>, Json(req): Json<ConvertLeadRequest>) -> Result<Json<LeadResponse>, AppError> {
    let svc = LeadService::new(&state.db_seaorm);
    Ok(Json(svc.convert(id, req.customer_id).await?))
}

pub async fn delete(State(state): State<SharedState>, Path(id): Path<i64>) -> Result<Json<MessageResponse>, AppError> {
    let svc = LeadService::new(&state.db_seaorm);
    Ok(Json(svc.delete(id).await?))
}

pub async fn list_activities(State(state): State<SharedState>, Path(id): Path<i64>) -> Result<Json<Vec<LeadActivityResponse>>, AppError> {
    let svc = LeadService::new(&state.db_seaorm);
    Ok(Json(svc.list_activities(id).await?))
}

pub async fn add_activity(State(state): State<SharedState>, Path(id): Path<i64>, Json(req): Json<AddLeadActivityRequest>) -> Result<Json<LeadActivityResponse>, AppError> {
    req.validate()?;
    let svc = LeadService::new(&state.db_seaorm);
    Ok(Json(svc.add_activity(id, req).await?))
}

pub async fn get_pipeline(State(state): State<SharedState>) -> Result<Json<LeadPipelineResponse>, AppError> {
    let svc = LeadService::new(&state.db_seaorm);
    Ok(Json(svc.get_pipeline().await?))
}

pub async fn get_stats(State(state): State<SharedState>) -> Result<Json<serde_json::Value>, AppError> {
    let svc = LeadService::new(&state.db_seaorm);
    Ok(Json(svc.get_stats().await?))
}
