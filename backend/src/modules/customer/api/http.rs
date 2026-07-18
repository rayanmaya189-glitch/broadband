use axum::extract::{Path, Query, State};
use axum::http::StatusCode;
use axum::Json;
use serde::{Deserialize, Serialize};
use std::sync::Arc;

use crate::modules::customer::application::services::CustomerService;
use crate::shared::app_state::AppState;
use crate::shared::errors::AppError;
use crate::shared::middleware::auth::{require_permission, UserContext};
use crate::shared::primitives::PaginationParams;

#[derive(Debug, Serialize)]
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

#[derive(Debug, Deserialize)]
pub struct CreateCustomerRequest {
    pub branch_id: i64,
    pub name: String,
    #[serde(default)]
    pub email: Option<String>,
    pub phone: String,
    #[serde(default)]
    pub alternate_phone: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct UpdateStatusRequest {
    pub status: String,
}

#[derive(Debug, Deserialize)]
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

#[derive(Debug, Serialize)]
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

/// GET /api/v1/customers
pub async fn list_customers(
    State(state): State<Arc<AppState>>,
    user: UserContext,
    Query(params): Query<PaginationParams>,
) -> Result<Json<serde_json::Value>, AppError> {
    require_permission(&user, "customer.account.view").map_err(|e| AppError::Forbidden(e.1))?;
    let branch_id = if user.is_company_wide {
        None
    } else {
        user.branch_id
    };
    let (customers, total) =
        CustomerService::list_customers(&state.db, branch_id, params.page(), params.limit())
            .await?;
    let resp: Vec<CustomerResponse> = customers
        .into_iter()
        .map(|c| CustomerResponse {
            id: c.id,
            customer_code: c.customer_code,
            branch_id: c.branch_id,
            name: c.name,
            email: c.email,
            phone: c.phone,
            status: c.status,
            created_at: c.created_at.to_rfc3339(),
        })
        .collect();
    Ok(Json(
        serde_json::json!({"items": resp, "total": total, "page": params.page(), "limit": params.limit()}),
    ))
}

/// POST /api/v1/customers
pub async fn create_customer(
    State(state): State<Arc<AppState>>,
    _user: UserContext,
    Json(req): Json<CreateCustomerRequest>,
) -> Result<(StatusCode, Json<CustomerResponse>), AppError> {
    let customer = CustomerService::create_customer(
        &state.db,
        req.branch_id,
        req.name,
        req.email,
        req.phone,
        req.alternate_phone,
    )
    .await?;

    // Publish event to outbox
    let payload = serde_json::json!({
        "customer_id": customer.id,
        "customer_code": customer.customer_code,
        "branch_id": customer.branch_id,
        "name": customer.name,
        "phone": customer.phone,
        "status": customer.status,
    });
    if let Err(e) = crate::infrastructure::messaging::outbox::insert_outbox_event(
        &state.db,
        "customer.created",
        "customer",
        customer.id,
        payload,
        None,
        None,
        Some(customer.branch_id),
    )
    .await
    {
        tracing::error!(customer_id = customer.id, error = %e, "Failed to publish customer.created event");
    }

    Ok((
        StatusCode::CREATED,
        Json(CustomerResponse {
            id: customer.id,
            customer_code: customer.customer_code,
            branch_id: customer.branch_id,
            name: customer.name,
            email: customer.email,
            phone: customer.phone,
            status: customer.status,
            created_at: customer.created_at.to_rfc3339(),
        }),
    ))
}

/// GET /api/v1/customers/:id
pub async fn get_customer(
    State(state): State<Arc<AppState>>,
    _user: UserContext,
    Path(id): Path<i64>,
) -> Result<Json<CustomerResponse>, AppError> {
    let customer = CustomerService::get_customer(&state.db, id).await?;
    Ok(Json(CustomerResponse {
        id: customer.id,
        customer_code: customer.customer_code,
        branch_id: customer.branch_id,
        name: customer.name,
        email: customer.email,
        phone: customer.phone,
        status: customer.status,
        created_at: customer.created_at.to_rfc3339(),
    }))
}

/// PUT /api/v1/customers/:id/status
pub async fn update_customer_status(
    State(state): State<Arc<AppState>>,
    _user: UserContext,
    Path(id): Path<i64>,
    Json(req): Json<UpdateStatusRequest>,
) -> Result<Json<CustomerResponse>, AppError> {
    let old_status = CustomerService::get_customer(&state.db, id)
        .await
        .map(|c| c.status)
        .unwrap_or_default();
    let customer = CustomerService::update_customer_status(&state.db, id, &req.status).await?;

    // Publish status change event
    let event_type = match req.status.as_str() {
        "active" => "customer.activated",
        "suspended" => "customer.suspended",
        "terminated" => "customer.terminated",
        _ => "customer.status.changed",
    };
    let payload = serde_json::json!({
        "customer_id": customer.id,
        "old_status": old_status,
        "new_status": customer.status,
    });
    if let Err(e) = crate::infrastructure::messaging::outbox::insert_outbox_event(
        &state.db,
        event_type,
        "customer",
        customer.id,
        payload,
        None,
        None,
        Some(customer.branch_id),
    )
    .await
    {
        tracing::error!(customer_id = customer.id, error = %e, "Failed to publish customer status event");
    }

    Ok(Json(CustomerResponse {
        id: customer.id,
        customer_code: customer.customer_code,
        branch_id: customer.branch_id,
        name: customer.name,
        email: customer.email,
        phone: customer.phone,
        status: customer.status,
        created_at: customer.created_at.to_rfc3339(),
    }))
}

/// GET /api/v1/customers/:id/addresses
pub async fn list_addresses(
    State(state): State<Arc<AppState>>,
    _user: UserContext,
    Path(id): Path<i64>,
) -> Result<Json<Vec<AddressResponse>>, AppError> {
    let addresses = CustomerService::get_addresses(&state.db, id).await?;
    Ok(Json(
        addresses
            .into_iter()
            .map(|a| AddressResponse {
                id: a.id,
                customer_id: a.customer_id,
                address_type: a.address_type,
                line1: a.line1,
                city: a.city,
                state: a.state,
                pincode: a.pincode,
                is_primary: a.is_primary,
            })
            .collect(),
    ))
}

/// POST /api/v1/customers/:id/addresses
pub async fn add_address(
    State(state): State<Arc<AppState>>,
    _user: UserContext,
    Path(id): Path<i64>,
    Json(req): Json<AddAddressRequest>,
) -> Result<(StatusCode, Json<AddressResponse>), AppError> {
    let addr = CustomerService::add_address(
        &state.db,
        id,
        req.address_type
            .unwrap_or_else(|| "installation".to_string()),
        req.line1,
        req.line2,
        req.city,
        req.state,
        req.pincode,
        req.landmark,
    )
    .await?;
    Ok((
        StatusCode::CREATED,
        Json(AddressResponse {
            id: addr.id,
            customer_id: addr.customer_id,
            address_type: addr.address_type,
            line1: addr.line1,
            city: addr.city,
            state: addr.state,
            pincode: addr.pincode,
            is_primary: addr.is_primary,
        }),
    ))
}

/// DELETE /api/v1/customers/:id
pub async fn delete_customer(
    State(state): State<Arc<AppState>>,
    _user: UserContext,
    Path(id): Path<i64>,
) -> Result<StatusCode, AppError> {
    CustomerService::delete_customer(&state.db, id).await?;
    Ok(StatusCode::NO_CONTENT)
}
