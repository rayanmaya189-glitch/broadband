use axum::extract::{Json, Path, Query, State};
use validator::Validate;
use crate::app::SharedState;
use crate::common::errors::app_error::AppError;
use crate::modules::inventory::request::inventory_request::*;
use crate::modules::inventory::response::inventory_response::*;
use crate::modules::inventory::service::inventory_service::InventoryService;

pub async fn list_items(State(state): State<SharedState>, Query(q): Query<InventoryQuery>) -> Result<Json<InventoryListResponse>, AppError> { let svc = InventoryService::new(&state.db); Ok(Json(svc.list(q).await?)) }
pub async fn get_item(State(state): State<SharedState>, Path(id): Path<i64>) -> Result<Json<InventoryItemResponse>, AppError> { let svc = InventoryService::new(&state.db); Ok(Json(svc.get(id).await?)) }
pub async fn create_item(State(state): State<SharedState>, Json(req): Json<CreateInventoryItemRequest>) -> Result<Json<InventoryItemResponse>, AppError> { req.validate()?; let svc = InventoryService::new(&state.db); Ok(Json(svc.create(req).await?)) }
pub async fn delete_item(State(state): State<SharedState>, Path(id): Path<i64>) -> Result<Json<MessageResponse>, AppError> { let svc = InventoryService::new(&state.db); Ok(Json(svc.delete(id).await?)) }
