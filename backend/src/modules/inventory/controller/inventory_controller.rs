use axum::extract::{Json, Path, Query, State};
use validator::Validate;
use crate::app::SharedState;
use crate::common::errors::app_error::AppError;
use crate::common::middleware::auth_middleware::UserContext;
use crate::modules::inventory::request::inventory_request::*;
use crate::modules::inventory::response::inventory_response::*;
use crate::modules::inventory::service::inventory_service::InventoryService;

pub async fn list_items(State(state): State<SharedState>, Query(q): Query<InventoryQuery>) -> Result<Json<InventoryListResponse>, AppError> {
    let svc = InventoryService::new(&state.db);
    Ok(Json(svc.list(q).await?))
}

pub async fn get_item(State(state): State<SharedState>, Path(id): Path<i64>) -> Result<Json<InventoryItemResponse>, AppError> {
    let svc = InventoryService::new(&state.db);
    Ok(Json(svc.get(id).await?))
}

pub async fn create_item(State(state): State<SharedState>, user: UserContext, Json(req): Json<CreateInventoryItemRequest>) -> Result<Json<InventoryItemResponse>, AppError> {
    req.validate()?;
    let svc = InventoryService::new(&state.db);
    Ok(Json(svc.create(req, user.user_id).await?))
}

pub async fn assign_item(State(state): State<SharedState>, user: UserContext, Path(id): Path<i64>, Json(req): Json<AssignInventoryRequest>) -> Result<Json<InventoryItemResponse>, AppError> {
    let svc = InventoryService::new(&state.db);
    Ok(Json(svc.assign(id, req, user.user_id).await?))
}

pub async fn install_item(State(state): State<SharedState>, user: UserContext, Path(id): Path<i64>) -> Result<Json<InventoryItemResponse>, AppError> {
    let svc = InventoryService::new(&state.db);
    Ok(Json(svc.install(id, user.user_id).await?))
}

pub async fn return_item(State(state): State<SharedState>, user: UserContext, Path(id): Path<i64>) -> Result<Json<InventoryItemResponse>, AppError> {
    let svc = InventoryService::new(&state.db);
    Ok(Json(svc.return_item(id, user.user_id).await?))
}

pub async fn transfer_item(State(state): State<SharedState>, user: UserContext, Path(id): Path<i64>, Json(req): Json<TransferInventoryRequest>) -> Result<Json<InventoryItemResponse>, AppError> {
    let svc = InventoryService::new(&state.db);
    Ok(Json(svc.transfer(id, req, user.user_id).await?))
}

pub async fn scrap_item(State(state): State<SharedState>, user: UserContext, Path(id): Path<i64>) -> Result<Json<InventoryItemResponse>, AppError> {
    let svc = InventoryService::new(&state.db);
    Ok(Json(svc.scrap(id, user.user_id).await?))
}

pub async fn delete_item(State(state): State<SharedState>, user: UserContext, Path(id): Path<i64>) -> Result<Json<MessageResponse>, AppError> {
    let svc = InventoryService::new(&state.db);
    Ok(Json(svc.delete(id, user.user_id).await?))
}

pub async fn list_movements(State(state): State<SharedState>, Path(id): Path<i64>) -> Result<Json<Vec<InventoryMovementResponse>>, AppError> {
    let svc = InventoryService::new(&state.db);
    Ok(Json(svc.list_movements(id).await?))
}

pub async fn get_report(State(state): State<SharedState>, Query(q): Query<InventoryQuery>) -> Result<Json<InventoryReportResponse>, AppError> {
    let svc = InventoryService::new(&state.db);
    Ok(Json(svc.get_report(q.branch_id).await?))
}

pub async fn get_warranty_alerts(State(state): State<SharedState>, Query(q): Query<InventoryQuery>) -> Result<Json<Vec<WarrantyAlertResponse>>, AppError> {
    let svc = InventoryService::new(&state.db);
    Ok(Json(svc.get_warranty_alerts(q.branch_id, None).await?))
}
