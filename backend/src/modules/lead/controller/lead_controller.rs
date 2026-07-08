use axum::extract::{Json, Path, Query, State};
use validator::Validate;

use crate::app::SharedState;
use crate::common::errors::app_error::AppError;
use crate::modules::lead::request::lead_request::*;
use crate::modules::lead::response::lead_response::*;
use crate::modules::lead::service::lead_service::LeadService;

pub async fn list_leads(State(state): State<SharedState>, Query(query): Query<LeadQuery>) -> Result<Json<LeadListResponse>, AppError> {
    let svc = LeadService::new(&state.db);
    Ok(Json(svc.list_leads(query).await?))
}

pub async fn create_lead(State(state): State<SharedState>, Json(req): Json<CreateLeadRequest>) -> Result<Json<LeadResponse>, AppError> {
    req.validate()?;
    let svc = LeadService::new(&state.db);
    Ok(Json(svc.create_lead(req).await?))
}

pub async fn get_lead(State(state): State<SharedState>, Path(id): Path<i64>) -> Result<Json<LeadResponse>, AppError> {
    let svc = LeadService::new(&state.db);
    Ok(Json(svc.get_lead(id).await?))
}

pub async fn update_lead(State(state): State<SharedState>, Path(id): Path<i64>, Json(req): Json<UpdateLeadRequest>) -> Result<Json<LeadResponse>, AppError> {
    let svc = LeadService::new(&state.db);
    Ok(Json(svc.update_lead(id, req).await?))
}

pub async fn update_status(State(state): State<SharedState>, Path(id): Path<i64>, Json(req): Json<LeadStatusRequest>) -> Result<Json<LeadResponse>, AppError> {
    let svc = LeadService::new(&state.db);
    Ok(Json(svc.update_status(id, req).await?))
}

pub async fn assign_lead(State(state): State<SharedState>, Path(id): Path<i64>, Json(req): Json<AssignLeadRequest>) -> Result<Json<LeadResponse>, AppError> {
    let svc = LeadService::new(&state.db);
    Ok(Json(svc.assign_lead(id, req).await?))
}

pub async fn add_activity(State(state): State<SharedState>, Path(id): Path<i64>, Json(req): Json<AddActivityRequest>) -> Result<Json<LeadActivityResponse>, AppError> {
    req.validate()?;
    let svc = LeadService::new(&state.db);
    Ok(Json(svc.add_activity(id, 1, req).await?))
}

pub async fn get_activities(State(state): State<SharedState>, Path(id): Path<i64>) -> Result<Json<Vec<LeadActivityResponse>>, AppError> {
    let svc = LeadService::new(&state.db);
    Ok(Json(svc.get_activities(id).await?))
}

pub async fn convert_lead(State(state): State<SharedState>, Path(id): Path<i64>, Json(req): Json<ConvertLeadRequest>) -> Result<Json<LeadResponse>, AppError> {
    let svc = LeadService::new(&state.db);
    Ok(Json(svc.convert_lead(id, req).await?))
}

pub async fn delete_lead(State(state): State<SharedState>, Path(id): Path<i64>) -> Result<Json<MessageResponse>, AppError> {
    let svc = LeadService::new(&state.db);
    Ok(Json(svc.delete_lead(id).await?))
}

pub async fn get_pipeline(State(state): State<SharedState>) -> Result<Json<LeadPipelineResponse>, AppError> {
    let svc = LeadService::new(&state.db);
    Ok(Json(svc.get_pipeline().await?))
}

pub async fn get_stats(State(state): State<SharedState>) -> Result<Json<LeadStatsResponse>, AppError> {
    let svc = LeadService::new(&state.db);
    Ok(Json(svc.get_stats().await?))
}
