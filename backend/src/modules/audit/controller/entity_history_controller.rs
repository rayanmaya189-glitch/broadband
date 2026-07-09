use axum::extract::{Json, Path, Query, State};
use crate::app::SharedState;
use crate::common::errors::app_error::AppError;
use crate::modules::audit::request::entity_history_request::*;
use crate::modules::audit::response::entity_history_response::*;
use crate::modules::audit::service::entity_history_service::EntityHistoryService;

pub async fn search_history(State(state): State<SharedState>, Query(q): Query<EntityHistoryQuery>) -> Result<Json<EntityHistoryListResponse>, AppError> {
    let svc = EntityHistoryService::new(&state.db);
    Ok(Json(svc.search(q).await?))
}

pub async fn get_history_entry(State(state): State<SharedState>, Path(id): Path<i64>) -> Result<Json<EntityHistoryResponse>, AppError> {
    let svc = EntityHistoryService::new(&state.db);
    Ok(Json(svc.get_by_id(id).await?))
}

pub async fn get_entity_history(State(state): State<SharedState>, Path((entity_type, entity_id)): Path<(String, i64)>) -> Result<Json<Vec<EntityHistoryResponse>>, AppError> {
    let svc = EntityHistoryService::new(&state.db);
    Ok(Json(svc.get_entity_history(&entity_type, entity_id).await?))
}

pub async fn rollback(State(state): State<SharedState>, Json(req): Json<RollbackRequest>) -> Result<Json<EntityHistoryResponse>, AppError> {
    let svc = EntityHistoryService::new(&state.db);
    Ok(Json(svc.rollback(req).await?))
}

pub async fn get_stats(State(state): State<SharedState>) -> Result<Json<EntityHistoryStatsResponse>, AppError> {
    let svc = EntityHistoryService::new(&state.db);
    Ok(Json(svc.get_stats().await?))
}
