/// OpenAPI schemas and stub handlers for Inventory endpoints.
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

// ── Request / Response types ─────────────────────────────────────────

#[derive(Debug, Serialize, ToSchema)]
pub struct InventoryItemResponse {
    /// Inventory item ID
    pub id: i64,
    /// Item type (e.g. "router", "cable", "ont")
    pub item_type: String,
    /// Serial number (if applicable)
    pub serial_number: Option<String>,
    /// Item status (e.g. "available", "assigned", "deployed")
    pub status: String,
}

#[derive(Debug, Deserialize, ToSchema)]
pub struct CreateInventoryItemRequest {
    /// Item type
    pub item_type: String,
    /// Serial number (optional)
    #[serde(default)]
    pub serial_number: Option<String>,
    /// Barcode (optional)
    #[serde(default)]
    pub barcode: Option<String>,
}

#[derive(Debug, Deserialize, ToSchema)]
pub struct UpdateStockRequest {
    /// Status to set
    pub status: String,
}

#[derive(Debug, Deserialize, ToSchema)]
pub struct AssignItemRequest {
    /// User ID to assign the item to
    pub assigned_to: i64,
}

// ── Stub handler functions (for OpenAPI spec only) ───────────────────

/// List all inventory items (paginated)
#[utoipa::path(
    get,
    path = "/api/v1/inventory",
    tag = "Inventory",
    params(("page" = Option<i64>, Query, description = "Page number"),
           ("limit" = Option<i64>, Query, description = "Items per page")),
    responses(
        (status = 200, description = "Paginated list of inventory items"),
        (status = 403, description = "Forbidden")
    ),
    security(("bearer_auth" = []))
)]
pub async fn list_inventory() -> axum::Json<serde_json::Value> {
    unimplemented!()
}

/// Create a new inventory item
#[utoipa::path(
    post,
    path = "/api/v1/inventory",
    tag = "Inventory",
    request_body = CreateInventoryItemRequest,
    responses(
        (status = 201, description = "Inventory item created", body = InventoryItemResponse),
        (status = 403, description = "Forbidden"),
        (status = 422, description = "Validation error")
    ),
    security(("bearer_auth" = []))
)]
pub async fn create_inventory_item() -> axum::Json<serde_json::Value> {
    unimplemented!()
}

/// Assign an inventory item to a user
#[utoipa::path(
    post,
    path = "/api/v1/inventory/{id}/assign",
    tag = "Inventory",
    params(("id" = i64, Path, description = "Inventory item ID")),
    request_body = AssignItemRequest,
    responses(
        (status = 200, description = "Item assigned", body = InventoryItemResponse),
        (status = 403, description = "Forbidden"),
        (status = 404, description = "Item not found"),
        (status = 422, description = "Validation error")
    ),
    security(("bearer_auth" = []))
)]
pub async fn assign_inventory_item() -> axum::Json<serde_json::Value> {
    unimplemented!()
}
