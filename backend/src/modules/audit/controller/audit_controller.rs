use axum::extract::{Json, Path, Query, State};
use crate::app::SharedState;
use crate::common::errors::app_error::AppError;
use crate::modules::audit::request::audit_request::*;
use crate::modules::audit::response::audit_response::*;
use crate::modules::audit::service::audit_service::AuditService;

pub async fn list_logs(State(state): State<SharedState>, Query(q): Query<AuditQuery>) -> Result<Json<AuditListResponse>, AppError> {
    let svc = AuditService::new(&state.db);
    Ok(Json(svc.list(q).await?))
}

pub async fn get_log(State(state): State<SharedState>, Path(id): Path<i64>) -> Result<Json<AuditLogResponse>, AppError> {
    let svc = AuditService::new(&state.db);
    Ok(Json(svc.get_by_id(id).await?))
}

pub async fn get_user_activity(State(state): State<SharedState>, Path(user_id): Path<i64>, Query(q): Query<AuditQuery>) -> Result<Json<AuditListResponse>, AppError> {
    let svc = AuditService::new(&state.db);
    let page = q.page.unwrap_or(1);
    let per_page = q.per_page.unwrap_or(50);
    Ok(Json(svc.get_user_activity(user_id, page, per_page).await?))
}

pub async fn get_resource_history(State(state): State<SharedState>, Path((resource_type, resource_id)): Path<(String, String)>) -> Result<Json<Vec<AuditLogResponse>>, AppError> {
    let svc = AuditService::new(&state.db);
    Ok(Json(svc.get_resource_history(&resource_type, &resource_id).await?))
}

pub async fn export_logs(State(state): State<SharedState>, Json(req): Json<ExportAuditRequest>) -> Result<Json<Vec<AuditLogResponse>>, AppError> {
    let svc = AuditService::new(&state.db);
    Ok(Json(svc.export_logs(req).await?))
}
