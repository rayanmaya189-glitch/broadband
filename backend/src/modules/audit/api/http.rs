/// Audit API endpoints per §27 Audit Design.
/// Provides entity history search, detail, and rollback endpoints.
use axum::extract::{Path, Query, State};
use axum::Json;
use chrono::NaiveDate;
use sea_orm::{ActiveModelTrait, ColumnTrait, EntityTrait, PaginatorTrait, QueryFilter, QueryOrder, QuerySelect};
use serde::Deserialize;
use std::sync::Arc;

use crate::infrastructure::messaging::outbox_entity::{self, Entity as OutboxEventEntity};
use crate::modules::audit::domain::entity_history::{
    EntityHistoryService, HistoryEntry, PaginatedResult,
};
use crate::shared::app_state::AppState;
use crate::shared::errors::AppError;
use crate::shared::middleware::auth::{require_permission, UserContext};
use crate::shared::primitives::PaginationParams;

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

#[derive(Debug, Deserialize)]
pub struct CompareQuery {
    pub entity_type: String,
    pub entity_id: String,
    pub version_a: String,
    pub version_b: String,
}

/// GET /api/v1/audit/history/compare — Compare two versions of an entity
pub async fn compare_history(
    State(state): State<Arc<AppState>>,
    Query(query): Query<CompareQuery>,
    user: UserContext,
) -> Result<Json<crate::modules::audit::domain::entity_history::HistoryDiff>, AppError> {
    require_permission(&user, "audit.history.view")
        .map_err(|e| AppError::Forbidden(e.1))?;

    let diff = EntityHistoryService::compare_history(
        &state.db,
        &query.entity_type,
        &query.entity_id,
        &query.version_a,
        &query.version_b,
    )
    .await?;

    Ok(Json(diff))
}

#[derive(Debug, Deserialize)]
pub struct ExportHistoryQuery {
    pub entity_type: String,
    pub entity_id: String,
    pub format: Option<String>,
}

/// GET /api/v1/audit/history/export — Export entity history
pub async fn export_history(
    State(state): State<Arc<AppState>>,
    Query(query): Query<ExportHistoryQuery>,
    user: UserContext,
) -> Result<Json<serde_json::Value>, AppError> {
    require_permission(&user, "audit.history.export")
        .map_err(|e| AppError::Forbidden(e.1))?;

    let export = EntityHistoryService::export_history(
        &state.db,
        &query.entity_type,
        &query.entity_id,
    )
    .await?;

    Ok(Json(serde_json::json!({
        "entity_type": export.entity_type,
        "entity_id": export.entity_id,
        "exported_at": export.exported_at,
        "count": export.count,
        "format": query.format.as_deref().unwrap_or("json"),
        "data": export.entries,
    })))
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

// ── Events / Outbox Endpoints ──

#[derive(Debug, Deserialize)]
pub struct EventQuery {
    pub event_type: Option<String>,
    pub aggregate_type: Option<String>,
    pub status: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct EventExportQuery {
    pub event_type: Option<String>,
    pub from: Option<chrono::DateTime<chrono::Utc>>,
    pub to: Option<chrono::DateTime<chrono::Utc>>,
    pub format: Option<String>,
}

/// GET /api/v1/audit/events — View event log from outbox
pub async fn list_events(
    State(state): State<Arc<AppState>>,
    Query(pagination): Query<PaginationParams>,
    Query(query): Query<EventQuery>,
    user: UserContext,
) -> Result<Json<serde_json::Value>, AppError> {
    require_permission(&user, "audit.event.view")
        .map_err(|e| AppError::Forbidden(e.1))?;

    let page = pagination.page();
    let limit = pagination.limit();
    let offset = pagination.offset();

    let mut stmt = OutboxEventEntity::find();

    if let Some(ref event_type) = query.event_type {
        stmt = stmt.filter(outbox_entity::Column::EventType.eq(event_type.as_str()));
    }
    if let Some(ref agg_type) = query.aggregate_type {
        stmt = stmt.filter(outbox_entity::Column::AggregateType.eq(agg_type.as_str()));
    }
    if let Some(ref status) = query.status {
        match status.as_str() {
            "published" => {
                stmt = stmt.filter(outbox_entity::Column::Published.eq(true));
            }
            "pending" => {
                stmt = stmt.filter(outbox_entity::Column::Published.eq(false));
                stmt = stmt.filter(outbox_entity::Column::DeadLetter.eq(false));
            }
            "dead_letter" => {
                stmt = stmt.filter(outbox_entity::Column::DeadLetter.eq(true));
            }
            _ => {}
        }
    }

    let total = stmt.clone().count(&state.db).await.map_err(|e| {
        AppError::Internal(anyhow::anyhow!("Failed to count outbox events: {}", e))
    })?;

    let events = stmt
        .order_by_desc(outbox_entity::Column::CreatedAt)
        .offset(offset)
        .limit(limit)
        .all(&state.db)
        .await
        .map_err(|e| {
            AppError::Internal(anyhow::anyhow!("Failed to fetch outbox events: {}", e))
        })?;

    Ok(Json(serde_json::json!({
        "data": events,
        "pagination": {
            "page": page,
            "limit": limit,
            "total": total,
            "total_pages": (total as f64 / limit as f64).ceil() as i64,
        }
    })))
}

/// GET /api/v1/audit/events/export — Export events matching criteria
pub async fn export_events(
    State(state): State<Arc<AppState>>,
    Query(query): Query<EventExportQuery>,
    user: UserContext,
) -> Result<Json<serde_json::Value>, AppError> {
    require_permission(&user, "audit.event.export")
        .map_err(|e| AppError::Forbidden(e.1))?;

    let mut stmt = OutboxEventEntity::find();

    if let Some(ref event_type) = query.event_type {
        stmt = stmt.filter(outbox_entity::Column::EventType.eq(event_type.as_str()));
    }
    if let Some(from) = query.from {
        stmt = stmt.filter(outbox_entity::Column::CreatedAt.gte(from));
    }
    if let Some(to) = query.to {
        stmt = stmt.filter(outbox_entity::Column::CreatedAt.lte(to));
    }

    let events = stmt
        .order_by_desc(outbox_entity::Column::CreatedAt)
        .limit(10000)
        .all(&state.db)
        .await
        .map_err(|e| {
            AppError::Internal(anyhow::anyhow!("Failed to export outbox events: {}", e))
        })?;

    Ok(Json(serde_json::json!({
        "exported_at": chrono::Utc::now().to_rfc3339(),
        "count": events.len(),
        "data": events,
    })))
}

/// POST /api/v1/audit/events/:id/replay — Replay a single event
pub async fn replay_event(
    State(state): State<Arc<AppState>>,
    Path(id): Path<i64>,
    user: UserContext,
) -> Result<Json<serde_json::Value>, AppError> {
    require_permission(&user, "audit.event.replay")
        .map_err(|e| AppError::Forbidden(e.1))?;

    let event = OutboxEventEntity::find()
        .filter(outbox_entity::Column::Id.eq(id))
        .one(&state.db)
        .await
        .map_err(|e| {
            AppError::Internal(anyhow::anyhow!("Failed to find outbox event: {}", e))
        })?
        .ok_or_else(|| AppError::NotFound(format!("Outbox event {} not found", id)))?;

    let mut active: outbox_entity::ActiveModel = event.into();
    active.published = sea_orm::Set(false);
    active.retry_count = sea_orm::Set(0);
    active.dead_letter = sea_orm::Set(false);
    active.dead_letter_at = sea_orm::Set(None);
    active.last_error = sea_orm::Set(None);
    active.updated_at = sea_orm::Set(chrono::Utc::now());
    active.update(&state.db).await.map_err(|e| {
        AppError::Internal(anyhow::anyhow!("Failed to replay outbox event: {}", e))
    })?;

    Ok(Json(serde_json::json!({
        "status": "replayed",
        "event_id": id,
    })))
}
