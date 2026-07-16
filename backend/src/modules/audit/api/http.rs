/// Audit API endpoints per §27 Audit Design.
/// Provides entity history search, detail, and rollback endpoints.
use axum::extract::{Path, Query, State};
use axum::Json;
use chrono::NaiveDate;
use serde::Deserialize;
use std::sync::Arc;

use crate::modules::audit::domain::entity_history::{EntityHistoryService, PaginatedResult, HistoryEntry};
use crate::shared::app_state::AppState;
use crate::shared::errors::AppError;
use crate::shared::middleware::auth::UserContext;

#[derive(Debug, Deserialize)]
pub struct HistoryQuery {
    pub entity_id: Option<String>,
    pub action: Option<String>,
    pub user_id: Option<i64>,
    pub from: Option<NaiveDate>,
    pub to: Option<NaiveDate>,
    pub page: Option<i64>,
    pub limit: Option<i64>,
}

#[derive(Debug, Deserialize)]
pub struct RollbackRequest {
    pub history_id: String,
    pub reason: String,
}

/// GET /api/v1/audit/history/:entity_type — Search entity history
pub async fn search_history(
    State(state): State<Arc<AppState>>,
    Path(entity_type): Path<String>,
    Query(query): Query<HistoryQuery>,
    _user: UserContext,
) -> Result<Json<PaginatedResult<HistoryEntry>>, AppError> {
    let page = query.page.unwrap_or(1).max(1);
    let limit = query.limit.unwrap_or(20).min(100);

    let result = EntityHistoryService::search_history(
        &state.db,
        &entity_type,
        query.entity_id,
        query.action.as_deref(),
        query.user_id,
        query.from,
        query.to,
        page,
        limit,
    )
    .await?;

    Ok(Json(result))
}

/// GET /api/v1/audit/history/:entity_type/:history_id — Get specific history entry
pub async fn get_history_entry(
    State(state): State<Arc<AppState>>,
    Path((entity_type, history_id)): Path<(String, String)>,
    _user: UserContext,
) -> Result<Json<HistoryEntry>, AppError> {
    let entry = EntityHistoryService::get_entry(&state.db, &entity_type, &history_id)
        .await?
        .ok_or_else(|| AppError::NotFound("History entry not found".into()))?;

    Ok(Json(entry))
}

/// POST /api/v1/audit/rollback/:entity_type/:entity_id — Rollback entity to previous state
pub async fn rollback_entity(
    State(state): State<Arc<AppState>>,
    Path((entity_type, entity_id)): Path<(String, String)>,
    user: UserContext,
    Json(req): Json<RollbackRequest>,
) -> Result<Json<serde_json::Value>, AppError> {
    // Only super_admin and isp_owner can perform rollbacks
    if !["super_admin", "isp_owner"].contains(&user.role.as_str()) {
        return Err(AppError::Forbidden(
            "Only administrators can perform rollbacks".to_string(),
        ));
    }

    let result = EntityHistoryService::rollback_entity(
        &state.db,
        &entity_type,
        &entity_id,
        &req.history_id,
        user.user_id,
        &req.reason,
    )
    .await?;

    Ok(Json(serde_json::json!({
        "status": "rolled_back",
        "history_id": result.history_id,
        "entity_type": result.entity_type,
        "entity_id": result.entity_id,
        "restored_from": result.restored_from,
    })))
}

/// GET /api/v1/audit/entity-types — List allowed entity types
pub async fn list_entity_types(
    _user: UserContext,
) -> Result<Json<Vec<&'static str>>, AppError> {
    Ok(Json(crate::modules::audit::domain::entity_history::ALLOWED_ENTITY_TYPES.to_vec()))
}
