//! SeaORM-based controller for the EntityHistory domain.

use axum::extract::{Json, Path, Query, State};

use crate::app::SharedState;
use crate::common::errors::app_error::AppError;
use crate::modules::audit::service::audit_service::AuditService;

pub async fn search_history(
    State(state): State<SharedState>,
) -> Result<Json<serde_json::Value>, AppError> {
    let svc = AuditService::new(&state.db_seaorm);
    let repo = crate::modules::audit::repository::entity_history_repository::EntityHistoryRepository::new(&state.db_seaorm);
    let (entries, total) = repo.search(None, None, None, None, None, None, 1, 100).await?;
    let results: Vec<serde_json::Value> = entries.iter().map(|e| {
        serde_json::json!({
            "id": e.id,
            "entity_type": e.entity_type,
            "entity_id": e.entity_id,
            "action": e.action,
            "user_id": e.user_id,
            "created_at": e.created_at,
        })
    }).collect();
    Ok(Json(serde_json::json!({ "data": results, "total": total })))
}

pub async fn get_history_entry(
    State(state): State<SharedState>,
    Path(id): Path<i64>,
) -> Result<Json<serde_json::Value>, AppError> {
    let repo = crate::modules::audit::repository::entity_history_repository::EntityHistoryRepository::new(&state.db_seaorm);
    let entry = repo.get_by_id(id).await?.ok_or_else(|| AppError::NotFound("History entry not found".into()))?;
    Ok(Json(serde_json::json!({
        "id": entry.id,
        "entity_type": entry.entity_type,
        "entity_id": entry.entity_id,
        "action": entry.action,
        "old_data": entry.old_data,
        "new_data": entry.new_data,
        "user_id": entry.user_id,
        "created_at": entry.created_at,
    })))
}

pub async fn get_entity_history(
    State(state): State<SharedState>,
    Path((entity_type, entity_id)): Path<(String, i64)>,
) -> Result<Json<serde_json::Value>, AppError> {
    let repo = crate::modules::audit::repository::entity_history_repository::EntityHistoryRepository::new(&state.db_seaorm);
    let entries = repo.get_entity_history(&entity_type, entity_id).await?;
    let results: Vec<serde_json::Value> = entries.iter().map(|e| {
        serde_json::json!({
            "id": e.id,
            "entity_type": e.entity_type,
            "entity_id": e.entity_id,
            "action": e.action,
            "user_id": e.user_id,
            "created_at": e.created_at,
        })
    }).collect();
    Ok(Json(serde_json::json!({ "data": results })))
}
