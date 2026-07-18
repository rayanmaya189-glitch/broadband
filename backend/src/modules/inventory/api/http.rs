use crate::modules::inventory::application::services::InventoryService;
use crate::shared::app_state::AppState;
use crate::shared::errors::AppError;
use crate::shared::middleware::auth::{require_permission, UserContext};
use crate::shared::primitives::PaginationParams;
use axum::extract::{Path, Query, State};
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
    Query(p): Query<PaginationParams>,
) -> Result<Json<serde_json::Value>, AppError> {
    require_permission(&user, "inventory.item.view").map_err(|e| AppError::Forbidden(e.1))?;
    let bid = if user.is_company_wide {
        None
    } else {
        user.branch_id
    };
    let (inv_items, total) =
        InventoryService::list_items(&state.db, bid, p.page(), p.limit()).await?;
    let items: Vec<InventoryItemResponse> = inv_items
        .into_iter()
        .map(|i| InventoryItemResponse {
            id: i.id,
            item_type: i.item_type,
            serial_number: i.serial_number,
            status: i.status,
        })
        .collect();
    Ok(Json(
        serde_json::json!({"items": items, "total": total, "page": p.page(), "limit": p.limit()}),
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
    if let Err(e) = crate::infrastructure::messaging::outbox::insert_outbox_event(
        &state.db,
        "inventory.item.created",
        "inventory_item",
        i.id,
        serde_json::json!({"item_id": i.id, "item_type": i.item_type}),
        None,
        Some(user.user_id),
        user.branch_id,
    )
    .await
    {
        tracing::error!(error = %e, "Failed to publish inventory.item.created event");
    }
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
    if let Err(e) = crate::infrastructure::messaging::outbox::insert_outbox_event(
        &state.db,
        "inventory.item.assigned",
        "inventory_item",
        i.id,
        serde_json::json!({"item_id": i.id, "assigned_to": req.assigned_to}),
        None,
        Some(user.user_id),
        user.branch_id,
    )
    .await
    {
        tracing::error!(error = %e, "Failed to publish inventory.item.assigned event");
    }
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
