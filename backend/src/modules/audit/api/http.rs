use crate::modules::audit::application::services::AuditService;
use crate::shared::app_state::AppState;
use crate::shared::errors::AppError;
use crate::shared::middleware::auth::{require_permission, UserContext};
use crate::shared::primitives::PaginationParams;
use axum::extract::{Query, State};
use axum::Json;
use std::sync::Arc;

pub async fn list_audit_logs(
    State(state): State<Arc<AppState>>,
    user: UserContext,
    Query(p): Query<PaginationParams>,
) -> Result<Json<serde_json::Value>, AppError> {
    require_permission(&user, "audit.log.view").map_err(|e| AppError::Forbidden(e.1))?;
    let (logs, total) = AuditService::list_logs(&state.db, p.page(), p.limit()).await?;
    let items: Vec<serde_json::Value> = logs.into_iter()
            .map(|l| {
                serde_json::json!({
                    "id": l.id, "action": l.action, "user_id": l.user_id,
                    "resource_type": l.resource_type, "result": l.result,
                    "created_at": l.created_at.to_rfc3339(),
                })
            })
            .collect();
    Ok(Json(serde_json::json!({"items": items, "total": total, "page": p.page(), "limit": p.limit()})))
}
