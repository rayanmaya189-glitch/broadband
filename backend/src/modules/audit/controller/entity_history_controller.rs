//! SeaORM-based controller for the EntityHistory domain.

use axum::extract::{Json, Path, Query, State};
use validator::Validate;

use crate::app::SharedState;
use crate::common::errors::app_error::AppError;
use crate::modules::audit::request::entity_history_request::{EntityHistoryQuery, RollbackRequest};
use crate::modules::audit::response::entity_history_response::RollbackResponse;
use crate::modules::audit::service::entity_history_service::EntityHistoryService;

pub async fn search_history(
    State(state): State<SharedState>,
    Query(q): Query<EntityHistoryQuery>,
) -> Result<Json<serde_json::Value>, AppError> {
    let svc = EntityHistoryService::new(&state.db_seaorm);
    let (entries, total) = svc.search(
        q.entity_type.as_deref(), q.entity_id, q.action.as_deref(),
        q.user_id, q.from.as_deref(), q.to.as_deref(),
        q.page.unwrap_or(1), q.per_page.unwrap_or(20),
    ).await?;
    let results: Vec<serde_json::Value> = entries.iter().map(|e| {
        serde_json::json!({
            "id": e.id,
            "entity_type": e.entity_type,
            "entity_id": e.entity_id,
            "action": e.action,
            "old_data": e.old_data,
            "new_data": e.new_data,
            "user_id": e.user_id,
            "reason": e.reason,
            "rollback_reference": e.rollback_reference,
            "created_at": e.created_at,
        })
    }).collect();
    Ok(Json(serde_json::json!({ "data": results, "total": total })))
}

pub async fn get_history_entry(
    State(state): State<SharedState>,
    Path(id): Path<i64>,
) -> Result<Json<serde_json::Value>, AppError> {
    let svc = EntityHistoryService::new(&state.db_seaorm);
    let entry = svc.get_by_id(id).await?;
    Ok(Json(serde_json::json!({
        "id": entry.id,
        "entity_type": entry.entity_type,
        "entity_id": entry.entity_id,
        "action": entry.action,
        "old_data": entry.old_data,
        "new_data": entry.new_data,
        "changed_fields": entry.changed_fields,
        "user_id": entry.user_id,
        "reason": entry.reason,
        "rollback_reference": entry.rollback_reference,
        "created_at": entry.created_at,
    })))
}

pub async fn get_entity_history(
    State(state): State<SharedState>,
    Path((entity_type, entity_id)): Path<(String, i64)>,
) -> Result<Json<serde_json::Value>, AppError> {
    let svc = EntityHistoryService::new(&state.db_seaorm);
    let entries = svc.get_entity_history(&entity_type, entity_id).await?;
    let results: Vec<serde_json::Value> = entries.iter().map(|e| {
        serde_json::json!({
            "id": e.id,
            "entity_type": e.entity_type,
            "entity_id": e.entity_id,
            "action": e.action,
            "old_data": e.old_data,
            "new_data": e.new_data,
            "user_id": e.user_id,
            "reason": e.reason,
            "rollback_reference": e.rollback_reference,
            "created_at": e.created_at,
        })
    }).collect();
    Ok(Json(serde_json::json!({ "data": results })))
}

/// Rollback an entity to a previous state identified by a history entry.
///
/// POST /api/v1/audit/entity-history/:id/rollback
///
/// Safety checks before rollback:
/// - customers: cannot rollback if active subscription exists
/// - invoices: cannot rollback if payment already processed
/// - network_devices: cannot rollback if currently online
/// - plans: cannot rollback if active subscribers exist
/// - subscriptions: cannot rollback if paid invoices exist
pub async fn rollback(
    State(state): State<SharedState>,
    Path(history_id): Path<i64>,
    Json(payload): Json<RollbackRequest>,
) -> Result<Json<RollbackResponse>, AppError> {
    payload.validate()?;
    let svc = EntityHistoryService::new(&state.db_seaorm);
    let result = svc.rollback(history_id, payload.user_id, &payload.reason).await?;
    let message = format!("Successfully rolled back {} #{}", result.entity_type, result.entity_id);
    Ok(Json(RollbackResponse {
        history_id: result.history_id,
        entity_type: result.entity_type,
        entity_id: result.entity_id,
        restored_from: result.restored_from,
        message,
    }))
}
