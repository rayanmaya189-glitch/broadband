use axum::extract::{Json, Path, Query, State};
use validator::Validate;
use crate::app::SharedState;
use crate::common::errors::app_error::AppError;
use crate::modules::installation::request::installation_request::*;
use crate::modules::installation::response::installation_response::*;
use crate::modules::installation::service::installation_service::InstallationService;

pub async fn list_installations(State(state): State<SharedState>, Query(q): Query<InstallationQuery>) -> Result<Json<InstallationListResponse>, AppError> {
    let svc = InstallationService::new(&state.db);
    Ok(Json(svc.list_installations(q).await?))
}

pub async fn get_installation(State(state): State<SharedState>, Path(id): Path<i64>) -> Result<Json<InstallationResponse>, AppError> {
    let svc = InstallationService::new(&state.db);
    Ok(Json(svc.get_installation(id).await?))
}

pub async fn create_installation(State(state): State<SharedState>, Json(req): Json<CreateInstallationRequest>) -> Result<Json<InstallationResponse>, AppError> {
    req.validate()?;
    let svc = InstallationService::new(&state.db);
    Ok(Json(svc.create_installation(req).await?))
}

pub async fn schedule_installation(State(state): State<SharedState>, Path(id): Path<i64>, Json(req): Json<ScheduleInstallationRequest>) -> Result<Json<InstallationResponse>, AppError> {
    req.validate()?;
    let svc = InstallationService::new(&state.db);
    Ok(Json(svc.schedule_installation(id, req).await?))
}

pub async fn start_installation(State(state): State<SharedState>, Path(id): Path<i64>) -> Result<Json<InstallationResponse>, AppError> {
    let svc = InstallationService::new(&state.db);
    Ok(Json(svc.start_installation(id).await?))
}

pub async fn complete_installation(State(state): State<SharedState>, Path(id): Path<i64>, Json(req): Json<CompleteInstallationRequest>) -> Result<Json<InstallationResponse>, AppError> {
    let svc = InstallationService::new(&state.db);
    Ok(Json(svc.complete_installation(id, req).await?))
}

pub async fn cancel_installation(State(state): State<SharedState>, Path(id): Path<i64>) -> Result<Json<MessageResponse>, AppError> {
    let svc = InstallationService::new(&state.db);
    Ok(Json(svc.cancel_installation(id).await?))
}

pub async fn get_my_assignments(State(state): State<SharedState>, Path(technician_id): Path<i64>) -> Result<Json<Vec<InstallationResponse>>, AppError> {
    let svc = InstallationService::new(&state.db);
    Ok(Json(svc.get_my_assignments(technician_id).await?))
}
