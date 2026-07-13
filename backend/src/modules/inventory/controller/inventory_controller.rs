use axum::extract::{Json, Path, Query, State};
use validator::Validate;
use crate::app::SharedState;
use crate::common::errors::app_error::AppError;
use crate::common::middleware::auth_middleware::UserContext;
use crate::modules::inventory::request::inventory_request::*;
use crate::modules::inventory::response::inventory_response::*;
use crate::modules::inventory::service::inventory_service::InventoryService;

#[utoipa::path(
    get,
    path = "/api/v1/inventory",
    tag = "Inventory",
    security(("bearer_auth" = [])),
    params(
        ("branch_id" = Option<i64>, Query, description = "Filter by branch"),
        ("status" = Option<String>, Query, description = "Filter by status"),
        ("item_type" = Option<String>, Query, description = "Filter by type"),
        ("page" = Option<i64>, Query, description = "Page number"),
        ("limit" = Option<i64>, Query, description = "Items per page")
    ),
    responses(
        (status = 200, description = "List of inventory items"),
        (status = 401, description = "Unauthorized")
    )
)]
pub async fn list_items(State(state): State<SharedState>, Query(q): Query<InventoryQuery>) -> Result<Json<InventoryListResponse>, AppError> {
    let svc = InventoryService::new(&state.db);
    Ok(Json(svc.list(q).await?))
}

#[utoipa::path(
    get,
    path = "/api/v1/inventory/{id}",
    tag = "Inventory",
    security(("bearer_auth" = [])),
    params(("id" = i64, Path, description = "Item ID")),
    responses(
        (status = 200, description = "Inventory item details", body = InventoryItemResponse),
        (status = 404, description = "Item not found")
    )
)]
pub async fn get_item(State(state): State<SharedState>, Path(id): Path<i64>) -> Result<Json<InventoryItemResponse>, AppError> {
    let svc = InventoryService::new(&state.db);
    Ok(Json(svc.get(id).await?))
}

#[utoipa::path(
    post,
    path = "/api/v1/inventory",
    tag = "Inventory",
    security(("bearer_auth" = [])),
    request_body = CreateInventoryItemRequest,
    responses(
        (status = 200, description = "Item created", body = InventoryItemResponse),
        (status = 422, description = "Validation error")
    )
)]
pub async fn create_item(State(state): State<SharedState>, user: UserContext, Json(req): Json<CreateInventoryItemRequest>) -> Result<Json<InventoryItemResponse>, AppError> {
    req.validate()?;
    let svc = InventoryService::new(&state.db);
    Ok(Json(svc.create(req, user.user_id).await?))
}

#[utoipa::path(
    post,
    path = "/api/v1/inventory/{id}/assign",
    tag = "Inventory",
    security(("bearer_auth" = [])),
    params(("id" = i64, Path, description = "Item ID")),
    request_body = AssignInventoryRequest,
    responses(
        (status = 200, description = "Item assigned", body = InventoryItemResponse),
        (status = 404, description = "Item not found")
    )
)]
pub async fn assign_item(State(state): State<SharedState>, user: UserContext, Path(id): Path<i64>, Json(req): Json<AssignInventoryRequest>) -> Result<Json<InventoryItemResponse>, AppError> {
    let svc = InventoryService::new(&state.db);
    Ok(Json(svc.assign(id, req, user.user_id).await?))
}

#[utoipa::path(
    post,
    path = "/api/v1/inventory/{id}/install",
    tag = "Inventory",
    security(("bearer_auth" = [])),
    params(("id" = i64, Path, description = "Item ID")),
    responses(
        (status = 200, description = "Item marked as installed", body = InventoryItemResponse),
        (status = 404, description = "Item not found")
    )
)]
pub async fn install_item(State(state): State<SharedState>, user: UserContext, Path(id): Path<i64>) -> Result<Json<InventoryItemResponse>, AppError> {
    let svc = InventoryService::new(&state.db);
    Ok(Json(svc.install(id, user.user_id).await?))
}

#[utoipa::path(
    post,
    path = "/api/v1/inventory/{id}/return",
    tag = "Inventory",
    security(("bearer_auth" = [])),
    params(("id" = i64, Path, description = "Item ID")),
    responses(
        (status = 200, description = "Item returned", body = InventoryItemResponse),
        (status = 404, description = "Item not found")
    )
)]
pub async fn return_item(State(state): State<SharedState>, user: UserContext, Path(id): Path<i64>) -> Result<Json<InventoryItemResponse>, AppError> {
    let svc = InventoryService::new(&state.db);
    Ok(Json(svc.return_item(id, user.user_id).await?))
}

#[utoipa::path(
    post,
    path = "/api/v1/inventory/{id}/transfer",
    tag = "Inventory",
    security(("bearer_auth" = [])),
    params(("id" = i64, Path, description = "Item ID")),
    request_body = TransferInventoryRequest,
    responses(
        (status = 200, description = "Item transferred", body = InventoryItemResponse),
        (status = 404, description = "Item not found")
    )
)]
pub async fn transfer_item(State(state): State<SharedState>, user: UserContext, Path(id): Path<i64>, Json(req): Json<TransferInventoryRequest>) -> Result<Json<InventoryItemResponse>, AppError> {
    let svc = InventoryService::new(&state.db);
    Ok(Json(svc.transfer(id, req, user.user_id).await?))
}

#[utoipa::path(
    post,
    path = "/api/v1/inventory/{id}/scrap",
    tag = "Inventory",
    security(("bearer_auth" = [])),
    params(("id" = i64, Path, description = "Item ID")),
    responses(
        (status = 200, description = "Item scrapped", body = InventoryItemResponse),
        (status = 404, description = "Item not found")
    )
)]
pub async fn scrap_item(State(state): State<SharedState>, user: UserContext, Path(id): Path<i64>) -> Result<Json<InventoryItemResponse>, AppError> {
    let svc = InventoryService::new(&state.db);
    Ok(Json(svc.scrap(id, user.user_id).await?))
}

#[utoipa::path(
    delete,
    path = "/api/v1/inventory/{id}",
    tag = "Inventory",
    security(("bearer_auth" = [])),
    params(("id" = i64, Path, description = "Item ID")),
    responses(
        (status = 200, description = "Item deleted"),
        (status = 404, description = "Item not found")
    )
)]
pub async fn delete_item(State(state): State<SharedState>, user: UserContext, Path(id): Path<i64>) -> Result<Json<MessageResponse>, AppError> {
    let svc = InventoryService::new(&state.db);
    Ok(Json(svc.delete(id, user.user_id).await?))
}

#[utoipa::path(
    get,
    path = "/api/v1/inventory/{id}/movements",
    tag = "Inventory",
    security(("bearer_auth" = [])),
    params(("id" = i64, Path, description = "Item ID")),
    responses(
        (status = 200, description = "List of movements", body = Vec<InventoryMovementResponse>),
        (status = 404, description = "Item not found")
    )
)]
pub async fn list_movements(State(state): State<SharedState>, Path(id): Path<i64>) -> Result<Json<Vec<InventoryMovementResponse>>, AppError> {
    let svc = InventoryService::new(&state.db);
    Ok(Json(svc.list_movements(id).await?))
}

#[utoipa::path(
    get,
    path = "/api/v1/inventory/reports",
    tag = "Inventory",
    security(("bearer_auth" = [])),
    params(("branch_id" = Option<i64>, Query, description = "Filter by branch")),
    responses(
        (status = 200, description = "Inventory report", body = InventoryReportResponse),
        (status = 401, description = "Unauthorized")
    )
)]
pub async fn get_report(State(state): State<SharedState>, Query(q): Query<InventoryQuery>) -> Result<Json<InventoryReportResponse>, AppError> {
    let svc = InventoryService::new(&state.db);
    Ok(Json(svc.get_report(q.branch_id).await?))
}

#[utoipa::path(
    get,
    path = "/api/v1/inventory/alerts",
    tag = "Inventory",
    security(("bearer_auth" = [])),
    params(("branch_id" = Option<i64>, Query, description = "Filter by branch")),
    responses(
        (status = 200, description = "Warranty alerts", body = Vec<WarrantyAlertResponse>),
        (status = 401, description = "Unauthorized")
    )
)]
pub async fn get_warranty_alerts(State(state): State<SharedState>, Query(q): Query<InventoryQuery>) -> Result<Json<Vec<WarrantyAlertResponse>>, AppError> {
    let svc = InventoryService::new(&state.db);
    Ok(Json(svc.get_warranty_alerts(q.branch_id, None).await?))
}
