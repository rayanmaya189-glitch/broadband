use axum::extract::{Query, State};
use crate::app::SharedState;
use crate::common::errors::app_error::AppError;
use crate::modules::audit::request::audit_request::*;
use crate::modules::audit::response::audit_response::*;
use crate::modules::audit::service::audit_service::AuditService;

pub async fn list_logs(State(state): State<SharedState>, Query(q): Query<AuditQuery>) -> Result<axum::Json<AuditListResponse>, AppError> {
    let svc = AuditService::new(&state.db);
    Ok(axum::Json(svc.list(q).await?))
}
