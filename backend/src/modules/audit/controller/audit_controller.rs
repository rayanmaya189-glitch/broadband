use axum::extract::{Json, Path, Query, State};
use crate::app::SharedState;
use crate::common::errors::app_error::AppError;
use crate::modules::audit::request::audit_request::*;
use crate::modules::audit::response::audit_response::*;
use crate::modules::audit::service::audit_service::AuditService;

#[utoipa::path(
    get,
    path = "/api/v1/audit/logs",
    tag = "Audit",
    security(("bearer_auth" = [])),
    params(
        ("user_id" = Option<i64>, Query, description = "Filter by user"),
        ("action" = Option<String>, Query, description = "Filter by action"),
        ("resource_type" = Option<String>, Query, description = "Filter by resource type"),
        ("result" = Option<String>, Query, description = "Filter by result"),
        ("from" = Option<String>, Query, description = "From date"),
        ("to" = Option<String>, Query, description = "To date"),
        ("page" = Option<i64>, Query, description = "Page number"),
        ("per_page" = Option<i64>, Query, description = "Items per page")
    ),
    responses(
        (status = 200, description = "List of audit logs"),
        (status = 401, description = "Unauthorized")
    )
)]
pub async fn list_logs(State(state): State<SharedState>, Query(q): Query<AuditQuery>) -> Result<Json<AuditListResponse>, AppError> {
    let svc = AuditService::new(&state.db);
    Ok(Json(svc.list(q).await?))
}

#[utoipa::path(
    get,
    path = "/api/v1/audit/logs/{id}",
    tag = "Audit",
    security(("bearer_auth" = [])),
    params(("id" = i64, Path, description = "Audit log ID")),
    responses(
        (status = 200, description = "Audit log details", body = AuditLogResponse),
        (status = 404, description = "Log not found")
    )
)]
pub async fn get_log(State(state): State<SharedState>, Path(id): Path<i64>) -> Result<Json<AuditLogResponse>, AppError> {
    let svc = AuditService::new(&state.db);
    Ok(Json(svc.get_by_id(id).await?))
}

#[utoipa::path(
    get,
    path = "/api/v1/audit/user/{user_id}",
    tag = "Audit",
    security(("bearer_auth" = [])),
    params(("user_id" = i64, Path, description = "User ID")),
    responses(
        (status = 200, description = "User activity log"),
        (status = 401, description = "Unauthorized")
    )
)]
pub async fn get_user_activity(State(state): State<SharedState>, Path(user_id): Path<i64>, Query(q): Query<AuditQuery>) -> Result<Json<AuditListResponse>, AppError> {
    let svc = AuditService::new(&state.db);
    let page = q.page.unwrap_or(1);
    let per_page = q.per_page.unwrap_or(50);
    Ok(Json(svc.get_user_activity(user_id, page, per_page).await?))
}

#[utoipa::path(
    get,
    path = "/api/v1/audit/resource/{resource_type}/{resource_id}",
    tag = "Audit",
    security(("bearer_auth" = [])),
    params(("resource_type" = String, Path, description = "Resource type"), ("resource_id" = String, Path, description = "Resource ID")),
    responses(
        (status = 200, description = "Resource history", body = Vec<AuditLogResponse>),
        (status = 401, description = "Unauthorized")
    )
)]
pub async fn get_resource_history(State(state): State<SharedState>, Path((resource_type, resource_id)): Path<(String, String)>) -> Result<Json<Vec<AuditLogResponse>>, AppError> {
    let svc = AuditService::new(&state.db);
    Ok(Json(svc.get_resource_history(&resource_type, &resource_id).await?))
}

#[utoipa::path(
    post,
    path = "/api/v1/audit/export",
    tag = "Audit",
    security(("bearer_auth" = [])),
    request_body = ExportAuditRequest,
    responses(
        (status = 200, description = "Exported audit logs"),
        (status = 401, description = "Unauthorized")
    )
)]
pub async fn export_logs(State(state): State<SharedState>, Json(req): Json<ExportAuditRequest>) -> Result<Json<Vec<AuditLogResponse>>, AppError> {
    let svc = AuditService::new(&state.db);
    Ok(Json(svc.export_logs(req).await?))
}
