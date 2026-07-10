//! SeaORM-based controller for the Inventory domain.

use axum::extract::{Json, Path, Query, State};
use validator::Validate;

use crate::app::SharedState;
use crate::common::errors::app_error::AppError;
use crate::modules::inventory::request::inventory_request::*;
use crate::modules::inventory::response::inventory_response::*;
use crate::modules::inventory::service::inventory_service_seaorm::InventoryServiceSeaorm;

pub async fn list(State(state): State<SharedState>, Query(q): Query<InventoryQuery>) -> Result<Json<Vec<InventoryItemResponse>>, AppError> {
    let svc = InventoryServiceSeaorm::new(&state.db_seaorm);
    let (items, _) = svc.list(q.branch_id, q.status.as_deref(), q.item_type.as_deref(), q.page.unwrap_or(1), q.per_page.unwrap_or(20)).await?;
    Ok(Json(items))
}

pub async fn create(State(state): State<SharedState>, Json(req): Json<CreateInventoryItemRequest>) -> Result<Json<InventoryItemResponse>, AppError> {
    req.validate()?;
    let svc = InventoryServiceSeaorm::new(&state.db_seaorm);
    Ok(Json(svc.create(req).await?))
}

pub async fn update_status(State(state): State<SharedState>, Path(id): Path<i64>, Json(req): Json<UpdateInventoryStatusRequest>) -> Result<Json<InventoryItemResponse>, AppError> {
    let svc = InventoryServiceSeaorm::new(&state.db_seaorm);
    Ok(Json(svc.update_status(id, &req.status).await?))
}

pub async fn assign(State(state): State<SharedState>, Path(id): Path<i64>, Json(req): Json<AssignInventoryRequest>) -> Result<Json<InventoryItemResponse>, AppError> {
    let svc = InventoryServiceSeaorm::new(&state.db_seaorm);
    Ok(Json(svc.assign(id, req.user_id).await?))
}

pub async fn install(State(state): State<SharedState>, Path(id): Path<i64>) -> Result<Json<InventoryItemResponse>, AppError> {
    let svc = InventoryServiceSeaorm::new(&state.db_seaorm);
    Ok(Json(svc.install(id).await?))
}

pub async fn return_item(State(state): State<SharedState>, Path(id): Path<i64>) -> Result<Json<InventoryItemResponse>, AppError> {
    let svc = InventoryServiceSeaorm::new(&state.db_seaorm);
    Ok(Json(svc.return_item(id).await?))
}

pub async fn transfer(State(state): State<SharedState>, Path(id): Path<i64>, Json(req): Json<TransferInventoryRequest>) -> Result<Json<InventoryItemResponse>, AppError> {
    let svc = InventoryServiceSeaorm::new(&state.db_seaorm);
    Ok(Json(svc.transfer(id, req.to_branch_id).await?))
}

pub async fn scrap(State(state): State<SharedState>, Path(id): Path<i64>) -> Result<Json<InventoryItemResponse>, AppError> {
    let svc = InventoryServiceSeaorm::new(&state.db_seaorm);
    Ok(Json(svc.scrap(id).await?))
}
