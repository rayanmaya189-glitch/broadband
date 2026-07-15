use axum::extract::State;
use axum::Json;
use std::sync::Arc;
use crate::shared::app_state::AppState;
use crate::shared::errors::AppError;
use crate::shared::middleware::auth::UserContext;

/// GET /api/v1/accounting/accounts
pub async fn list_accounts(State(_state): State<Arc<AppState>>, _user: UserContext)
    -> Result<Json<serde_json::Value>, AppError> {
    Ok(Json(serde_json::json!({"accounts": [], "message": "Accounting module - implementation pending"})))
}
