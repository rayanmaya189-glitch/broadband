use crate::modules::inventory::application::services::InventoryService;
use crate::shared::app_state::AppState;
use crate::shared::errors::AppError;
use crate::shared::middleware::auth::{require_permission, UserContext};
use axum::extract::{Path, State};
use axum::http::StatusCode;
use axum::Json;
use serde::{Deserialize, Serialize};
use std::sync::Arc;

#[derive(Debug, Serialize)]
pub struct InventoryItemResponse {
    pub id: i64,
    pub item_type: String,
    pub serial_number: Option<String>,
    pub status: String,
}

#[derive(Debug, Deserialize)]
pub struct CreateItemRequest {
    pub item_type: String,
    #[serde(default)]
    pub serial_number: Option<String>,
    #[serde(default)]
    pub barcode: Option<String>,
}

pub async fn list_inventory(
    State(state): State<Arc<AppState>>,
    user: UserContext,
) -> Result<Json<Vec<InventoryItemResponse>>, AppError> {
    require_permission(&user, "inventory.item.view").map_err(|e| AppError::Forbidden(e.1))?;
    let bid = if user.is_company_wide {
        None
    } else {
        user.branch_id
    };
    let items = InventoryService::list_items(&state.db, bid).await?;
    Ok(Json(
        items
            .into_iter()
            .map(|i| InventoryItemResponse {
                id: i.id,
                item_type: i.item_type,
                serial_number: i.serial_number,
                status: i.status,
            })
            .collect(),
    ))
}

pub async fn create_inventory_item(
    State(state): State<Arc<AppState>>,
    user: UserContext,
    Json(req): Json<CreateItemRequest>,
) -> Result<(StatusCode, Json<InventoryItemResponse>), AppError> {
    require_permission(&user, "inventory.item.create").map_err(|e| AppError::Forbidden(e.1))?;
    let i = InventoryService::create_item(
        &state.db,
        user.branch_id.unwrap_or(0),
        req.item_type,
        req.serial_number,
        req.barcode,
    )
    .await?;
    Ok((
        StatusCode::CREATED,
        Json(InventoryItemResponse {
            id: i.id,
            item_type: i.item_type,
            serial_number: i.serial_number,
            status: i.status,
        }),
    ))
}

pub async fn assign_inventory_item(
    State(state): State<Arc<AppState>>,
    user: UserContext,
    Path(id): Path<i64>,
    Json(req): Json<AssignItemRequest>,
) -> Result<Json<InventoryItemResponse>, AppError> {
    require_permission(&user, "inventory.item.assign").map_err(|e| AppError::Forbidden(e.1))?;
    let i = InventoryService::assign_item(&state.db, id, req.assigned_to).await?;
    Ok(Json(InventoryItemResponse {
        id: i.id,
        item_type: i.item_type,
        serial_number: i.serial_number,
        status: i.status,
    }))
}

#[derive(Debug, Deserialize)]
pub struct AssignItemRequest {
    pub assigned_to: i64,
}
