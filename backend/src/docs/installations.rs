/// OpenAPI schemas and stub handlers for Installations endpoints.
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

// ── Request / Response types ─────────────────────────────────────────

#[derive(Debug, Serialize, ToSchema)]
pub struct InstallationResponse {
    /// Installation order ID
    pub id: i64,
    /// Customer ID
    pub customer_id: i64,
    /// Order status
    pub status: String,
    /// Scheduled date (YYYY-MM-DD)
    pub scheduled_date: Option<String>,
}

#[derive(Debug, Deserialize, ToSchema)]
pub struct CreateOrderRequest {
    /// Customer ID
    pub customer_id: i64,
    /// Subscription ID (optional)
    #[serde(default)]
    pub subscription_id: Option<i64>,
}

#[derive(Debug, Deserialize, ToSchema)]
pub struct ScheduleRequest {
    /// Scheduled date (YYYY-MM-DD)
    pub scheduled_date: String,
    /// Time slot (optional, e.g. "09:00-12:00")
    #[serde(default)]
    pub scheduled_time_slot: Option<String>,
    /// Technician ID (optional)
    #[serde(default)]
    pub technician_id: Option<i64>,
}

#[derive(Debug, Deserialize, ToSchema)]
pub struct RescheduleRequest {
    /// New date (YYYY-MM-DD)
    pub new_date: String,
    /// New time slot (optional)
    #[serde(default)]
    pub new_time_slot: Option<String>,
}

#[derive(Debug, Serialize, ToSchema)]
pub struct PhotoResponse {
    /// Photo ID
    pub id: i64,
    /// Installation order ID
    pub installation_order_id: i64,
    /// Storage key
    pub storage_key: String,
    /// Storage bucket
    pub storage_bucket: String,
    /// Photo type (e.g. "before", "after", "equipment")
    pub photo_type: String,
    /// User ID of uploader
    pub uploaded_by: Option<i64>,
    /// Notes
    pub notes: Option<String>,
}

#[derive(Debug, Deserialize, ToSchema)]
pub struct AddPhotoRequest {
    /// Storage key
    pub storage_key: String,
    /// Storage bucket
    pub storage_bucket: String,
    /// Photo type
    pub photo_type: String,
    /// Notes (optional)
    #[serde(default)]
    pub notes: Option<String>,
}

#[derive(Debug, Serialize, ToSchema)]
pub struct EquipmentResponse {
    /// Equipment item ID
    pub id: i64,
    /// Installation order ID
    pub installation_order_id: i64,
    /// Equipment type
    pub equipment_type: String,
    /// Model name
    pub model_name: Option<String>,
    /// Serial number
    pub serial_number: Option<String>,
    /// Quantity
    pub quantity: i32,
    /// Status
    pub status: String,
    /// Notes
    pub notes: Option<String>,
}

#[derive(Debug, Deserialize, ToSchema)]
pub struct AddEquipmentRequest {
    /// Equipment type
    pub equipment_type: String,
    /// Model name (optional)
    #[serde(default)]
    pub model_name: Option<String>,
    /// Serial number (optional)
    #[serde(default)]
    pub serial_number: Option<String>,
    /// Quantity (optional, defaults to 1)
    #[serde(default)]
    pub quantity: Option<i32>,
    /// Notes (optional)
    #[serde(default)]
    pub notes: Option<String>,
}

#[derive(Debug, Deserialize, ToSchema)]
pub struct UpdateEquipmentStatusRequest {
    /// New status
    pub status: String,
}

// ── Stub handler functions (for OpenAPI spec only) ───────────────────

/// List all installation orders (paginated)
#[utoipa::path(
    get,
    path = "/api/v1/installations",
    tag = "Installations",
    params(("page" = Option<i64>, Query, description = "Page number"),
           ("limit" = Option<i64>, Query, description = "Items per page")),
    responses(
        (status = 200, description = "Paginated list of installation orders"),
        (status = 403, description = "Forbidden")
    ),
    security(("bearer_auth" = []))
)]
pub async fn list_installations() -> axum::Json<serde_json::Value> {
    unimplemented!()
}

/// Create a new installation order
#[utoipa::path(
    post,
    path = "/api/v1/installations",
    tag = "Installations",
    request_body = CreateOrderRequest,
    responses(
        (status = 201, description = "Installation order created", body = InstallationResponse),
        (status = 403, description = "Forbidden"),
        (status = 422, description = "Validation error")
    ),
    security(("bearer_auth" = []))
)]
pub async fn create_installation() -> axum::Json<serde_json::Value> {
    unimplemented!()
}

/// Get a single installation order by ID
#[utoipa::path(
    get,
    path = "/api/v1/installations/{id}",
    tag = "Installations",
    params(("id" = i64, Path, description = "Installation order ID")),
    responses(
        (status = 200, description = "Installation order details", body = InstallationResponse),
        (status = 403, description = "Forbidden"),
        (status = 404, description = "Order not found")
    ),
    security(("bearer_auth" = []))
)]
pub async fn get_installation() -> axum::Json<serde_json::Value> {
    unimplemented!()
}

/// Reschedule an installation order
#[utoipa::path(
    post,
    path = "/api/v1/installations/{id}/reschedule",
    tag = "Installations",
    params(("id" = i64, Path, description = "Installation order ID")),
    request_body = RescheduleRequest,
    responses(
        (status = 200, description = "Installation rescheduled", body = InstallationResponse),
        (status = 403, description = "Forbidden"),
        (status = 404, description = "Order not found"),
        (status = 422, description = "Validation error")
    ),
    security(("bearer_auth" = []))
)]
pub async fn reschedule_installation() -> axum::Json<serde_json::Value> {
    unimplemented!()
}

/// Start an installation (set status to "in_progress")
#[utoipa::path(
    post,
    path = "/api/v1/installations/{id}/start",
    tag = "Installations",
    params(("id" = i64, Path, description = "Installation order ID")),
    responses(
        (status = 200, description = "Installation started", body = InstallationResponse),
        (status = 403, description = "Forbidden"),
        (status = 404, description = "Order not found")
    ),
    security(("bearer_auth" = []))
)]
pub async fn start_installation() -> axum::Json<serde_json::Value> {
    unimplemented!()
}

/// Complete an installation order
#[utoipa::path(
    post,
    path = "/api/v1/installations/{id}/complete",
    tag = "Installations",
    params(("id" = i64, Path, description = "Installation order ID")),
    responses(
        (status = 200, description = "Installation completed", body = InstallationResponse),
        (status = 403, description = "Forbidden"),
        (status = 404, description = "Order not found")
    ),
    security(("bearer_auth" = []))
)]
pub async fn complete_installation() -> axum::Json<serde_json::Value> {
    unimplemented!()
}

/// Cancel an installation order
#[utoipa::path(
    post,
    path = "/api/v1/installations/{id}/cancel",
    tag = "Installations",
    params(("id" = i64, Path, description = "Installation order ID")),
    responses(
        (status = 200, description = "Installation cancelled"),
        (status = 403, description = "Forbidden"),
        (status = 404, description = "Order not found")
    ),
    security(("bearer_auth" = []))
)]
pub async fn cancel_installation() -> axum::Json<serde_json::Value> {
    unimplemented!()
}

/// Add a photo to an installation
#[utoipa::path(
    post,
    path = "/api/v1/installations/{id}/photos",
    tag = "Installations",
    params(("id" = i64, Path, description = "Installation order ID")),
    request_body = AddPhotoRequest,
    responses(
        (status = 201, description = "Photo added", body = PhotoResponse),
        (status = 403, description = "Forbidden"),
        (status = 404, description = "Order not found"),
        (status = 422, description = "Validation error")
    ),
    security(("bearer_auth" = []))
)]
pub async fn add_installation_photo() -> axum::Json<serde_json::Value> {
    unimplemented!()
}

/// List installation assignments for the current user
#[utoipa::path(
    get,
    path = "/api/v1/installations/my-assignments",
    tag = "Installations",
    params(("page" = Option<i64>, Query, description = "Page number"),
           ("limit" = Option<i64>, Query, description = "Items per page")),
    responses(
        (status = 200, description = "Paginated list of assigned installations"),
        (status = 403, description = "Forbidden")
    ),
    security(("bearer_auth" = []))
)]
pub async fn list_my_installation_assignments() -> axum::Json<serde_json::Value> {
    unimplemented!()
}

/// List equipment for an installation order
#[utoipa::path(
    get,
    path = "/api/v1/installations/{id}/equipment",
    tag = "Installations",
    params(("id" = i64, Path, description = "Installation order ID")),
    responses(
        (status = 200, description = "List of equipment items"),
        (status = 403, description = "Forbidden"),
        (status = 404, description = "Order not found")
    ),
    security(("bearer_auth" = []))
)]
pub async fn list_equipment() -> axum::Json<serde_json::Value> {
    unimplemented!()
}

/// Add equipment to an installation order
#[utoipa::path(
    post,
    path = "/api/v1/installations/{id}/equipment",
    tag = "Installations",
    params(("id" = i64, Path, description = "Installation order ID")),
    request_body = AddEquipmentRequest,
    responses(
        (status = 201, description = "Equipment added", body = EquipmentResponse),
        (status = 403, description = "Forbidden"),
        (status = 404, description = "Order not found"),
        (status = 422, description = "Validation error")
    ),
    security(("bearer_auth" = []))
)]
pub async fn add_equipment() -> axum::Json<serde_json::Value> {
    unimplemented!()
}

/// Update the status of an equipment item
#[utoipa::path(
    put,
    path = "/api/v1/installations/equipment/{equipment_id}/status",
    tag = "Installations",
    params(("equipment_id" = i64, Path, description = "Equipment item ID")),
    request_body = UpdateEquipmentStatusRequest,
    responses(
        (status = 200, description = "Equipment status updated", body = EquipmentResponse),
        (status = 403, description = "Forbidden"),
        (status = 404, description = "Equipment not found"),
        (status = 422, description = "Validation error")
    ),
    security(("bearer_auth" = []))
)]
pub async fn update_equipment_status() -> axum::Json<serde_json::Value> {
    unimplemented!()
}
