//! SeaORM-based controller for the Audit domain.

use axum::extract::{Json, Path, Query, State};

use crate::app::SharedState;
use crate::common::errors::app_error::AppError;
use crate::modules::audit::service::audit_service::AuditService;

pub async fn list_logs(
    State(state): State<SharedState>,
) -> Result<Json<serde_json::Value>, AppError> {
    let svc = AuditService::new(&state.db_seaorm);
    let (logs, total) = svc.list(None, None, None, None, None, None, 1, 100).await?;
    let results: Vec<serde_json::Value> = logs.iter().map(|l| {
        serde_json::json!({
            "id": l.id,
            "user_id": l.user_id,
            "user_email": l.user_email,
            "action": l.action,
            "resource_type": l.resource_type,
            "result": l.result,
            "created_at": l.created_at,
        })
    }).collect();
    Ok(Json(serde_json::json!({ "data": results, "total": total })))
}

pub async fn get_log(
    State(state): State<SharedState>,
    Path(id): Path<i64>,
) -> Result<Json<serde_json::Value>, AppError> {
    let svc = AuditService::new(&state.db_seaorm);
    let log = svc.get_by_id(id).await?;
    Ok(Json(serde_json::json!({
        "id": log.id,
        "user_id": log.user_id,
        "user_email": log.user_email,
        "action": log.action,
        "resource_type": log.resource_type,
        "result": log.result,
        "created_at": log.created_at,
    })))
}
