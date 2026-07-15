use axum::extract::State;
use axum::Json;
use std::sync::Arc;
use crate::shared::app_state::AppState;
use crate::shared::errors::AppError;
use crate::shared::middleware::auth::UserContext;
use crate::modules::audit::application::services::AuditService;

pub async fn list_audit_logs(State(state): State<Arc<AppState>>, _user: UserContext) -> Result<Json<Vec<serde_json::Value>>, AppError> {
    let logs = AuditService::list_logs(&state.db).await?;
    Ok(Json(logs.into_iter().map(|l| serde_json::json!({
        "id": l.id, "action": l.action, "user_id": l.user_id,
        "resource_type": l.resource_type, "result": l.result,
        "created_at": l.created_at.to_rfc3339(),
    })).collect()))
}
