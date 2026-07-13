use axum::extract::{Json, Path, Query, State};
use crate::app::SharedState;
use crate::common::errors::app_error::AppError;
use crate::modules::audit::request::entity_history_request::*;
use crate::modules::audit::response::entity_history_response::*;
use crate::modules::audit::service::entity_history_service::EntityHistoryService;

#[utoipa::path(
    get,
    path = "/api/v1/audit/entity-history",
    tag = "Audit",
    security(("bearer_auth" = [])),
    params(
        ("entity_type" = Option<String>, Query, description = "Filter by entity type"),
        ("entity_id" = Option<i64>, Query, description = "Filter by entity ID"),
        ("action" = Option<String>, Query, description = "Filter by action"),
        ("user_id" = Option<i64>, Query, description = "Filter by user"),
        ("from" = Option<String>, Query, description = "From date"),
        ("to" = Option<String>, Query, description = "To date"),
        ("page" = Option<i64>, Query, description = "Page number"),
        ("limit" = Option<i64>, Query, description = "Items per page")
    ),
    responses(
        (status = 200, description = "Search results"),
        (status = 401, description = "Unauthorized")
    )
)]
pub async fn search_history(State(state): State<SharedState>, Query(q): Query<EntityHistoryQuery>) -> Result<Json<EntityHistoryListResponse>, AppError> {
    let svc = EntityHistoryService::new(&state.db);
    Ok(Json(svc.search(q).await?))
}

#[utoipa::path(
    get,
    path = "/api/v1/audit/entity-history/{id}",
    tag = "Audit",
    security(("bearer_auth" = [])),
    params(("id" = i64, Path, description = "History entry ID")),
    responses(
        (status = 200, description = "History entry details", body = EntityHistoryResponse),
        (status = 404, description = "Entry not found")
    )
)]
pub async fn get_history_entry(State(state): State<SharedState>, Path(id): Path<i64>) -> Result<Json<EntityHistoryResponse>, AppError> {
    let svc = EntityHistoryService::new(&state.db);
    Ok(Json(svc.get_by_id(id).await?))
}

#[utoipa::path(
    get,
    path = "/api/v1/audit/entity-history/{entity_type}/{entity_id}",
    tag = "Audit",
    security(("bearer_auth" = [])),
    params(("entity_type" = String, Path, description = "Entity type"), ("entity_id" = i64, Path, description = "Entity ID")),
    responses(
        (status = 200, description = "Entity history", body = Vec<EntityHistoryResponse>),
        (status = 401, description = "Unauthorized")
    )
)]
pub async fn get_entity_history(State(state): State<SharedState>, Path((entity_type, entity_id)): Path<(String, i64)>) -> Result<Json<Vec<EntityHistoryResponse>>, AppError> {
    let svc = EntityHistoryService::new(&state.db);
    Ok(Json(svc.get_entity_history(&entity_type, entity_id).await?))
}

#[utoipa::path(
    post,
    path = "/api/v1/audit/entity-history/rollback",
    tag = "Audit",
    security(("bearer_auth" = [])),
    request_body = RollbackRequest,
    responses(
        (status = 200, description = "Rollback completed", body = EntityHistoryResponse),
        (status = 400, description = "Invalid rollback request")
    )
)]
pub async fn rollback(State(state): State<SharedState>, Json(req): Json<RollbackRequest>) -> Result<Json<EntityHistoryResponse>, AppError> {
    let svc = EntityHistoryService::new(&state.db);
    Ok(Json(svc.rollback(req).await?))
}

#[utoipa::path(
    get,
    path = "/api/v1/audit/entity-history/stats",
    tag = "Audit",
    security(("bearer_auth" = [])),
    responses(
        (status = 200, description = "Entity history statistics", body = EntityHistoryStatsResponse),
        (status = 401, description = "Unauthorized")
    )
)]
pub async fn get_stats(State(state): State<SharedState>) -> Result<Json<EntityHistoryStatsResponse>, AppError> {
    let svc = EntityHistoryService::new(&state.db);
    Ok(Json(svc.get_stats().await?))
}
