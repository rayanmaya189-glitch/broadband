/// OpenAPI schemas and stub handlers for Customer endpoints.
use serde::{Deserialize, Serialize};
use utoipa::{IntoParams, ToSchema};

// ── Request / Response types ─────────────────────────────────────────

#[derive(Debug, Serialize, ToSchema)]
pub struct CustomerResponse {
    pub id: i64,
    pub customer_code: String,
    pub branch_id: i64,
    pub name: String,
    pub email: Option<String>,
    pub phone: String,
    pub status: String,
    pub created_at: String,
}

#[derive(Debug, Deserialize, ToSchema)]
pub struct CreateCustomerRequest {
    pub branch_id: i64,
    pub name: String,
    #[serde(default)]
    pub email: Option<String>,
    pub phone: String,
    #[serde(default)]
    pub alternate_phone: Option<String>,
}

#[derive(Debug, Deserialize, ToSchema)]
pub struct UpdateStatusRequest {
    pub status: String,
}

#[derive(Debug, Serialize, ToSchema)]
pub struct AddressResponse {
    pub id: i64,
    pub customer_id: i64,
    pub address_type: String,
    pub line1: String,
    pub city: String,
    pub state: String,
    pub pincode: String,
    pub is_primary: bool,
}

#[derive(Debug, Deserialize, ToSchema)]
pub struct AddAddressRequest {
    #[serde(default)]
    pub address_type: Option<String>,
    pub line1: String,
    #[serde(default)]
    pub line2: Option<String>,
    pub city: String,
    pub state: String,
    pub pincode: String,
    #[serde(default)]
    pub landmark: Option<String>,
}

#[derive(Debug, Deserialize, IntoParams)]
pub struct CustomerListParams {
    pub page: Option<i64>,
    pub limit: Option<i64>,
    pub status: Option<String>,
    pub search: Option<String>,
    pub branch_id: Option<i64>,
}

// ── Stub handler functions ───────────────────────────────────────────

/// List all customers with optional filters
#[utoipa::path(
    get,
    path = "/api/v1/customers",
    tag = "Customers",
    params(CustomerListParams),
    responses(
        (status = 200, description = "List of customers"),
        (status = 403, description = "Insufficient permissions")
    ),
    security(("bearer_auth" = []))
)]
pub async fn list_customers() -> axum::Json<serde_json::Value> {
    unimplemented!()
}

/// Create a new customer
#[utoipa::path(
    post,
    path = "/api/v1/customers",
    tag = "Customers",
    request_body = CreateCustomerRequest,
    responses(
        (status = 201, description = "Customer created", body = CustomerResponse),
        (status = 409, description = "Customer code already exists"),
        (status = 422, description = "Validation error")
    ),
    security(("bearer_auth" = []))
)]
pub async fn create_customer() -> axum::Json<serde_json::Value> {
    unimplemented!()
}

/// Get customer by ID
#[utoipa::path(
    get,
    path = "/api/v1/customers/{id}",
    tag = "Customers",
    params(("id" = i64, Path, description = "Customer ID")),
    responses(
        (status = 200, description = "Customer details", body = CustomerResponse),
        (status = 404, description = "Customer not found")
    ),
    security(("bearer_auth" = []))
)]
pub async fn get_customer() -> axum::Json<serde_json::Value> {
    unimplemented!()
}

/// Update customer status (activate, suspend, deactivate)
#[utoipa::path(
    put,
    path = "/api/v1/customers/{id}/status",
    tag = "Customers",
    params(("id" = i64, Path, description = "Customer ID")),
    request_body = UpdateStatusRequest,
    responses(
        (status = 200, description = "Status updated"),
        (status = 403, description = "Invalid status transition"),
        (status = 404, description = "Customer not found")
    ),
    security(("bearer_auth" = []))
)]
pub async fn update_customer_status() -> axum::Json<serde_json::Value> {
    unimplemented!()
}

/// Delete a customer (soft delete)
#[utoipa::path(
    delete,
    path = "/api/v1/customers/{id}",
    tag = "Customers",
    params(("id" = i64, Path, description = "Customer ID")),
    responses(
        (status = 204, description = "Customer deleted"),
        (status = 404, description = "Customer not found")
    ),
    security(("bearer_auth" = []))
)]
pub async fn delete_customer() -> axum::Json<serde_json::Value> {
    unimplemented!()
}
