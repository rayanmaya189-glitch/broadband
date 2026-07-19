/// Audit API endpoints per §27 Audit Design.
/// Provides entity history search, detail, and rollback endpoints.
use axum::extract::{Path, Query, State};
use axum::Json;
use chrono::NaiveDate;
use serde::Deserialize;
use std::sync::Arc;

use crate::modules::audit::domain::entity_history::{
    EntityHistoryService, HistoryEntry, PaginatedResult,
};
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
pub async fn list_entity_types(_user: UserContext) -> Result<Json<Vec<&'static str>>, AppError> {
    Ok(Json(
        crate::modules::audit::domain::entity_history::ALLOWED_ENTITY_TYPES.to_vec(),
    ))
}

// ── Audit Log Search Endpoints (§27) ──

#[derive(Debug, Deserialize)]
pub struct AuditLogQuery {
    pub user_id: Option<i64>,
    pub action: Option<String>,
    pub resource_type: Option<String>,
    pub result: Option<String>,
    pub page: Option<i64>,
    pub limit: Option<i64>,
}

/// GET /api/v1/audit/logs — Search audit logs with filters
pub async fn search_audit_logs(
    State(state): State<Arc<AppState>>,
    Query(query): Query<AuditLogQuery>,
    _user: UserContext,
) -> Result<Json<serde_json::Value>, AppError> {
    let page = query.page.unwrap_or(1).max(1);
    let limit = query.limit.unwrap_or(20).min(100);
    let offset = (page - 1) * limit;

    let repo = crate::modules::audit::infrastructure::repository::AuditRepository::new(&state.db);
    let logs = repo
        .search_logs(
            query.user_id,
            query.action.as_deref(),
            query.resource_type.as_deref(),
            query.result.as_deref(),
            limit,
            offset,
        )
        .await?;

    let total = repo
        .count_logs(query.user_id, query.action.as_deref())
        .await?;

    Ok(Json(serde_json::json!({
        "data": logs,
        "pagination": {
            "page": page,
            "limit": limit,
            "total": total,
            "total_pages": (total as f64 / limit as f64).ceil() as i64,
        }
    })))
}

/// GET /api/v1/audit/logs/:id — Get specific audit log entry
pub async fn get_audit_log(
    State(state): State<Arc<AppState>>,
    Path(id): Path<i64>,
    _user: UserContext,
) -> Result<Json<crate::modules::audit::domain::entities::audit_log::Model>, AppError> {
    let repo = crate::modules::audit::infrastructure::repository::AuditRepository::new(&state.db);
    let log = repo
        .find_by_id(id)
        .await?
        .ok_or_else(|| AppError::NotFound(format!("Audit log {} not found", id)))?;
    Ok(Json(log))
}

/// GET /api/v1/audit/user/:user_id — Get user activity log
pub async fn get_user_activity(
    State(state): State<Arc<AppState>>,
    Path(user_id): Path<i64>,
    Query(query): Query<AuditLogQuery>,
    _user: UserContext,
) -> Result<Json<Vec<crate::modules::audit::domain::entities::audit_log::Model>>, AppError> {
    let limit = query.limit.unwrap_or(50).min(200);
    let repo = crate::modules::audit::infrastructure::repository::AuditRepository::new(&state.db);
    let logs = repo.get_user_activity(user_id, limit).await?;
    Ok(Json(logs))
}

/// GET /api/v1/audit/export — Export audit logs as JSON
pub async fn export_audit_logs(
    State(state): State<Arc<AppState>>,
    Query(query): Query<AuditLogQuery>,
    _user: UserContext,
) -> Result<Json<serde_json::Value>, AppError> {
    // Only super_admin and isp_owner can export audit logs
    if !["super_admin", "isp_owner"].contains(&_user.role.as_str()) {
        return Err(AppError::Forbidden(
            "Only administrators can export audit logs".to_string(),
        ));
    }

    let repo = crate::modules::audit::infrastructure::repository::AuditRepository::new(&state.db);
    let logs = repo
        .search_logs(
            query.user_id,
            query.action.as_deref(),
            query.resource_type.as_deref(),
            query.result.as_deref(),
            10000, // Max export limit
            0,
        )
        .await?;

    Ok(Json(serde_json::json!({
        "exported_at": chrono::Utc::now().to_rfc3339(),
        "count": logs.len(),
        "data": logs,
    })))
}
