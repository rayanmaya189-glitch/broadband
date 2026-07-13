use axum::extract::{Json, Path, Query, State};
use validator::Validate;

use crate::app::SharedState;
use crate::common::errors::app_error::AppError;
use crate::modules::customer::request::customer_request::*;
use crate::modules::customer::response::customer_response::*;
use crate::modules::customer::service::customer_service::CustomerService;

// ── Customer CRUD ────────────────────────────────────────────

#[utoipa::path(
    get,
    path = "/api/v1/customers",
    tag = "Customers",
    security(("bearer_auth" = [])),
    params(
        ("page" = Option<i64>, Query, description = "Page number"),
        ("limit" = Option<i64>, Query, description = "Items per page"),
        ("search" = Option<String>, Query, description = "Search term"),
        ("status" = Option<String>, Query, description = "Filter by status")
    ),
    responses(
        (status = 200, description = "List of customers"),
        (status = 401, description = "Unauthorized")
    )
)]
pub async fn list_customers(State(state): State<SharedState>, Query(query): Query<ListCustomersQuery>) -> Result<Json<crate::common::utils::helpers::PaginatedResponse<CustomerResponse>>, AppError> {
    let svc = CustomerService::new(&state.db);
    Ok(Json(svc.list_customers(&query).await?))
}

#[utoipa::path(
    post,
    path = "/api/v1/customers",
    tag = "Customers",
    security(("bearer_auth" = [])),
    request_body = CreateCustomerRequest,
    responses(
        (status = 200, description = "Customer created", body = CustomerResponse),
        (status = 409, description = "Customer already exists"),
        (status = 422, description = "Validation error")
    )
)]
pub async fn create_customer(State(state): State<SharedState>, Json(req): Json<CreateCustomerRequest>) -> Result<Json<CustomerResponse>, AppError> {
    req.validate()?;
    let svc = CustomerService::new(&state.db);
    Ok(Json(svc.create_customer(&req).await?))
}

#[utoipa::path(
    get,
    path = "/api/v1/customers/{id}",
    tag = "Customers",
    security(("bearer_auth" = [])),
    params(("id" = i64, Path, description = "Customer ID")),
    responses(
        (status = 200, description = "Customer details", body = CustomerResponse),
        (status = 404, description = "Customer not found")
    )
)]
pub async fn get_customer(State(state): State<SharedState>, Path(id): Path<i64>) -> Result<Json<CustomerResponse>, AppError> {
    let svc = CustomerService::new(&state.db);
    Ok(Json(svc.get_customer(id).await?))
}

#[utoipa::path(
    put,
    path = "/api/v1/customers/{id}",
    tag = "Customers",
    security(("bearer_auth" = [])),
    params(("id" = i64, Path, description = "Customer ID")),
    request_body = UpdateCustomerRequest,
    responses(
        (status = 200, description = "Customer updated", body = CustomerResponse),
        (status = 404, description = "Customer not found"),
        (status = 422, description = "Validation error")
    )
)]
pub async fn update_customer(State(state): State<SharedState>, Path(id): Path<i64>, Json(req): Json<UpdateCustomerRequest>) -> Result<Json<CustomerResponse>, AppError> {
    req.validate()?;
    let svc = CustomerService::new(&state.db);
    Ok(Json(svc.update_customer(id, &req).await?))
}

#[utoipa::path(
    put,
    path = "/api/v1/customers/{id}/status",
    tag = "Customers",
    security(("bearer_auth" = [])),
    params(("id" = i64, Path, description = "Customer ID")),
    request_body = CustomerStatusTransition,
    responses(
        (status = 200, description = "Status updated", body = CustomerResponse),
        (status = 404, description = "Customer not found"),
        (status = 422, description = "Invalid status transition")
    )
)]
pub async fn update_status(State(state): State<SharedState>, Path(id): Path<i64>, Json(req): Json<CustomerStatusTransition>) -> Result<Json<CustomerResponse>, AppError> {
    let svc = CustomerService::new(&state.db);
    Ok(Json(svc.update_status(id, &req).await?))
}

#[utoipa::path(
    delete,
    path = "/api/v1/customers/{id}",
    tag = "Customers",
    security(("bearer_auth" = [])),
    params(("id" = i64, Path, description = "Customer ID")),
    responses(
        (status = 200, description = "Customer deleted"),
        (status = 404, description = "Customer not found")
    )
)]
pub async fn delete_customer(State(state): State<SharedState>, Path(id): Path<i64>) -> Result<Json<MessageResponse>, AppError> {
    let svc = CustomerService::new(&state.db);
    Ok(Json(svc.delete_customer(id).await?))
}

// ── Customer Profile (KYC) ──────────────────────────────────

#[utoipa::path(
    get,
    path = "/api/v1/customers/{id}/profile",
    tag = "Customers",
    security(("bearer_auth" = [])),
    params(("id" = i64, Path, description = "Customer ID")),
    responses(
        (status = 200, description = "Customer profile", body = CustomerProfileResponse),
        (status = 404, description = "Customer not found")
    )
)]
pub async fn get_profile(State(state): State<SharedState>, Path(id): Path<i64>) -> Result<Json<CustomerProfileResponse>, AppError> {
    let svc = CustomerService::new(&state.db);
    Ok(Json(svc.get_profile(id).await?))
}

#[utoipa::path(
    put,
    path = "/api/v1/customers/{id}/profile",
    tag = "Customers",
    security(("bearer_auth" = [])),
    params(("id" = i64, Path, description = "Customer ID")),
    request_body = UpdateCustomerProfileRequest,
    responses(
        (status = 200, description = "Profile updated", body = CustomerProfileResponse),
        (status = 404, description = "Customer not found"),
        (status = 422, description = "Validation error")
    )
)]
pub async fn update_profile(State(state): State<SharedState>, Path(id): Path<i64>, Json(req): Json<UpdateCustomerProfileRequest>) -> Result<Json<CustomerProfileResponse>, AppError> {
    let svc = CustomerService::new(&state.db);
    Ok(Json(svc.update_profile(id, &req).await?))
}

#[utoipa::path(
    post,
    path = "/api/v1/customers/{id}/kyc/submit",
    tag = "Customers",
    security(("bearer_auth" = [])),
    params(("id" = i64, Path, description = "Customer ID")),
    request_body = SubmitKycRequest,
    responses(
        (status = 200, description = "KYC submitted"),
        (status = 404, description = "Customer not found")
    )
)]
pub async fn submit_kyc(State(state): State<SharedState>, Path(id): Path<i64>, Json(req): Json<SubmitKycRequest>) -> Result<Json<MessageResponse>, AppError> {
    let svc = CustomerService::new(&state.db);
    Ok(Json(svc.submit_kyc(id, &req).await?))
}

#[utoipa::path(
    post,
    path = "/api/v1/customers/{id}/kyc/verify",
    tag = "Customers",
    security(("bearer_auth" = [])),
    params(("id" = i64, Path, description = "Customer ID")),
    request_body = VerifyKycRequest,
    responses(
        (status = 200, description = "KYC verified"),
        (status = 404, description = "Customer not found")
    )
)]
pub async fn verify_kyc(State(state): State<SharedState>, Path(id): Path<i64>, Json(req): Json<VerifyKycRequest>) -> Result<Json<MessageResponse>, AppError> {
    let svc = CustomerService::new(&state.db);
    Ok(Json(svc.verify_kyc(id, &req).await?))
}

// ── KYC Documents ───────────────────────────────────────────

#[utoipa::path(
    get,
    path = "/api/v1/customers/{id}/kyc/documents",
    tag = "Customers",
    security(("bearer_auth" = [])),
    params(("id" = i64, Path, description = "Customer ID")),
    responses(
        (status = 200, description = "List of KYC documents", body = Vec<KycDocumentResponse>),
        (status = 404, description = "Customer not found")
    )
)]
pub async fn list_kyc_documents(State(state): State<SharedState>, Path(id): Path<i64>) -> Result<Json<Vec<KycDocumentResponse>>, AppError> {
    let svc = CustomerService::new(&state.db);
    Ok(Json(svc.list_kyc_documents(id).await?))
}

#[utoipa::path(
    post,
    path = "/api/v1/customers/{id}/kyc/documents",
    tag = "Customers",
    security(("bearer_auth" = [])),
    params(("id" = i64, Path, description = "Customer ID")),
    request_body = serde_json::Value,
    responses(
        (status = 200, description = "Document uploaded", body = KycDocumentResponse),
        (status = 404, description = "Customer not found"),
        (status = 422, description = "Validation error")
    )
)]
pub async fn upload_kyc_document(State(state): State<SharedState>, Path(id): Path<i64>, Json(req): Json<serde_json::Value>) -> Result<Json<KycDocumentResponse>, AppError> {
    let svc = CustomerService::new(&state.db);
    let doc_type = req["document_type"].as_str().ok_or_else(|| AppError::Validation("document_type required".into()))?;
    let doc_url = req["document_url"].as_str().ok_or_else(|| AppError::Validation("document_url required".into()))?;
    let file_name = req["file_name"].as_str();
    let file_size = req["file_size"].as_i64();
    let mime_type = req["mime_type"].as_str();
    Ok(Json(svc.upload_kyc_document(id, doc_type, doc_url, file_name, file_size, mime_type).await?))
}

#[utoipa::path(
    delete,
    path = "/api/v1/customers/{id}/kyc/documents/{doc_id}",
    tag = "Customers",
    security(("bearer_auth" = [])),
    params(("id" = i64, Path, description = "Customer ID"), ("doc_id" = i64, Path, description = "Document ID")),
    responses(
        (status = 200, description = "Document deleted"),
        (status = 404, description = "Document not found")
    )
)]
pub async fn delete_kyc_document(State(state): State<SharedState>, Path((id, doc_id)): Path<(i64, i64)>) -> Result<Json<MessageResponse>, AppError> {
    let svc = CustomerService::new(&state.db);
    Ok(Json(svc.delete_kyc_document(id, doc_id).await?))
}

// ── Customer Addresses ──────────────────────────────────────

#[utoipa::path(
    get,
    path = "/api/v1/customers/{id}/addresses",
    tag = "Customers",
    security(("bearer_auth" = [])),
    params(("id" = i64, Path, description = "Customer ID")),
    responses(
        (status = 200, description = "List of addresses", body = Vec<AddressResponse>),
        (status = 404, description = "Customer not found")
    )
)]
pub async fn list_addresses(State(state): State<SharedState>, Path(id): Path<i64>) -> Result<Json<Vec<AddressResponse>>, AppError> {
    let svc = CustomerService::new(&state.db);
    Ok(Json(svc.list_addresses(id).await?))
}

#[utoipa::path(
    post,
    path = "/api/v1/customers/{id}/addresses",
    tag = "Customers",
    security(("bearer_auth" = [])),
    params(("id" = i64, Path, description = "Customer ID")),
    request_body = CreateAddressRequest,
    responses(
        (status = 200, description = "Address created", body = AddressResponse),
        (status = 404, description = "Customer not found"),
        (status = 422, description = "Validation error")
    )
)]
pub async fn create_address(State(state): State<SharedState>, Path(id): Path<i64>, Json(req): Json<CreateAddressRequest>) -> Result<Json<AddressResponse>, AppError> {
    req.validate()?;
    let svc = CustomerService::new(&state.db);
    Ok(Json(svc.create_address(id, &req).await?))
}

#[utoipa::path(
    put,
    path = "/api/v1/customers/{id}/addresses/{addr_id}",
    tag = "Customers",
    security(("bearer_auth" = [])),
    params(("id" = i64, Path, description = "Customer ID"), ("addr_id" = i64, Path, description = "Address ID")),
    request_body = UpdateAddressRequest,
    responses(
        (status = 200, description = "Address updated", body = AddressResponse),
        (status = 404, description = "Address not found")
    )
)]
pub async fn update_address(State(state): State<SharedState>, Path((id, addr_id)): Path<(i64, i64)>, Json(req): Json<UpdateAddressRequest>) -> Result<Json<AddressResponse>, AppError> {
    let svc = CustomerService::new(&state.db);
    Ok(Json(svc.update_address(id, addr_id, &req).await?))
}

#[utoipa::path(
    delete,
    path = "/api/v1/customers/{id}/addresses/{addr_id}",
    tag = "Customers",
    security(("bearer_auth" = [])),
    params(("id" = i64, Path, description = "Customer ID"), ("addr_id" = i64, Path, description = "Address ID")),
    responses(
        (status = 200, description = "Address deleted"),
        (status = 404, description = "Address not found")
    )
)]
pub async fn delete_address(State(state): State<SharedState>, Path((id, addr_id)): Path<(i64, i64)>) -> Result<Json<MessageResponse>, AppError> {
    let svc = CustomerService::new(&state.db);
    Ok(Json(svc.delete_address(id, addr_id).await?))
}
