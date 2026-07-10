//! SeaORM-based controller for the Installation domain.

use axum::extract::{Json, Path, Query, State};
use validator::Validate;

use crate::app::SharedState;
use crate::common::errors::app_error::AppError;
use crate::modules::installation::request::installation_request::*;
use crate::modules::installation::response::installation_response::*;
use crate::modules::installation::service::installation_service_seaorm::InstallationServiceSeaorm;

pub async fn list(State(state): State<SharedState>, Query(q): Query<InstallationQuery>) -> Result<Json<Vec<InstallationOrderResponse>>, AppError> {
    let svc = InstallationServiceSeaorm::new(&state.db_seaorm);
    let (orders, _) = svc.list(q.branch_id, q.status.as_deref(), q.page.unwrap_or(1), q.per_page.unwrap_or(20)).await?;
    Ok(Json(orders))
}

pub async fn get_by_id(State(state): State<SharedState>, Path(id): Path<i64>) -> Result<Json<InstallationOrderResponse>, AppError> {
    let svc = InstallationServiceSeaorm::new(&state.db_seaorm);
    Ok(Json(svc.get_by_id(id).await?))
}

pub async fn create(State(state): State<SharedState>, Json(req): Json<CreateInstallationRequest>) -> Result<Json<InstallationOrderResponse>, AppError> {
    req.validate()?;
    let svc = InstallationServiceSeaorm::new(&state.db_seaorm);
    Ok(Json(svc.create(req.customer_id, req.branch_id, req.subscription_id, &req.installation_type).await?))
}

pub async fn schedule(State(state): State<SharedState>, Path(id): Path<i64>, Json(req): Json<ScheduleInstallationRequest>) -> Result<Json<InstallationOrderResponse>, AppError> {
    req.validate()?;
    let svc = InstallationServiceSeaorm::new(&state.db_seaorm);
    Ok(Json(svc.schedule(id, req).await?))
}

pub async fn start(State(state): State<SharedState>, Path(id): Path<i64>) -> Result<Json<InstallationOrderResponse>, AppError> {
    let svc = InstallationServiceSeaorm::new(&state.db_seaorm);
    Ok(Json(svc.start(id).await?))
}

pub async fn complete(State(state): State<SharedState>, Path(id): Path<i64>, Json(req): Json<CompleteInstallationRequest>) -> Result<Json<InstallationOrderResponse>, AppError> {
    let svc = InstallationServiceSeaorm::new(&state.db_seaorm);
    Ok(Json(svc.complete(id, req).await?))
}

pub async fn cancel(State(state): State<SharedState>, Path(id): Path<i64>) -> Result<Json<InstallationOrderResponse>, AppError> {
    let svc = InstallationServiceSeaorm::new(&state.db_seaorm);
    Ok(Json(svc.cancel(id).await?))
}

pub async fn get_my_assignments(State(state): State<SharedState>, Path(technician_id): Path<i64>) -> Result<Json<Vec<InstallationOrderResponse>>, AppError> {
    let svc = InstallationServiceSeaorm::new(&state.db_seaorm);
    Ok(Json(svc.get_my_assignments(technician_id).await?))
}
