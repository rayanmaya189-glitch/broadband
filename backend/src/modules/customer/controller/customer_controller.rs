use axum::extract::{Json, Path, Query, State};
use validator::Validate;

use crate::app::SharedState;
use crate::common::errors::app_error::AppError;
use crate::modules::customer::request::customer_request::*;
use crate::modules::customer::response::customer_response::*;
use crate::modules::customer::service::customer_service::CustomerService;

// ── Customer CRUD ────────────────────────────────────────────

pub async fn list_customers(State(state): State<SharedState>, Query(query): Query<ListCustomersQuery>) -> Result<Json<crate::common::utils::helpers::PaginatedResponse<CustomerResponse>>, AppError> {
    let svc = CustomerService::new(&state.db);
    Ok(Json(svc.list_customers(&query).await?))
}

pub async fn create_customer(State(state): State<SharedState>, Json(req): Json<CreateCustomerRequest>) -> Result<Json<CustomerResponse>, AppError> {
    req.validate()?;
    let svc = CustomerService::new(&state.db);
    Ok(Json(svc.create_customer(&req).await?))
}

pub async fn get_customer(State(state): State<SharedState>, Path(id): Path<i64>) -> Result<Json<CustomerResponse>, AppError> {
    let svc = CustomerService::new(&state.db);
    Ok(Json(svc.get_customer(id).await?))
}

pub async fn update_customer(State(state): State<SharedState>, Path(id): Path<i64>, Json(req): Json<UpdateCustomerRequest>) -> Result<Json<CustomerResponse>, AppError> {
    req.validate()?;
    let svc = CustomerService::new(&state.db);
    Ok(Json(svc.update_customer(id, &req).await?))
}

pub async fn update_status(State(state): State<SharedState>, Path(id): Path<i64>, Json(req): Json<CustomerStatusTransition>) -> Result<Json<CustomerResponse>, AppError> {
    let svc = CustomerService::new(&state.db);
    Ok(Json(svc.update_status(id, &req).await?))
}

pub async fn delete_customer(State(state): State<SharedState>, Path(id): Path<i64>) -> Result<Json<MessageResponse>, AppError> {
    let svc = CustomerService::new(&state.db);
    Ok(Json(svc.delete_customer(id).await?))
}

// ── Customer Profile (KYC) ──────────────────────────────────

pub async fn get_profile(State(state): State<SharedState>, Path(id): Path<i64>) -> Result<Json<CustomerProfileResponse>, AppError> {
    let svc = CustomerService::new(&state.db);
    Ok(Json(svc.get_profile(id).await?))
}

pub async fn update_profile(State(state): State<SharedState>, Path(id): Path<i64>, Json(req): Json<UpdateCustomerProfileRequest>) -> Result<Json<CustomerProfileResponse>, AppError> {
    let svc = CustomerService::new(&state.db);
    Ok(Json(svc.update_profile(id, &req).await?))
}

pub async fn submit_kyc(State(state): State<SharedState>, Path(id): Path<i64>, Json(req): Json<SubmitKycRequest>) -> Result<Json<MessageResponse>, AppError> {
    let svc = CustomerService::new(&state.db);
    Ok(Json(svc.submit_kyc(id, &req).await?))
}

pub async fn verify_kyc(State(state): State<SharedState>, Path(id): Path<i64>, Json(req): Json<VerifyKycRequest>) -> Result<Json<MessageResponse>, AppError> {
    let svc = CustomerService::new(&state.db);
    Ok(Json(svc.verify_kyc(id, &req).await?))
}

// ── KYC Documents ───────────────────────────────────────────

pub async fn list_kyc_documents(State(state): State<SharedState>, Path(id): Path<i64>) -> Result<Json<Vec<KycDocumentResponse>>, AppError> {
    let svc = CustomerService::new(&state.db);
    Ok(Json(svc.list_kyc_documents(id).await?))
}

pub async fn upload_kyc_document(State(state): State<SharedState>, Path(id): Path<i64>, Json(req): Json<serde_json::Value>) -> Result<Json<KycDocumentResponse>, AppError> {
    let svc = CustomerService::new(&state.db);
    let doc_type = req["document_type"].as_str().ok_or_else(|| AppError::Validation("document_type required".into()))?;
    let doc_url = req["document_url"].as_str().ok_or_else(|| AppError::Validation("document_url required".into()))?;
    let file_name = req["file_name"].as_str();
    let file_size = req["file_size"].as_i64();
    let mime_type = req["mime_type"].as_str();
    Ok(Json(svc.upload_kyc_document(id, doc_type, doc_url, file_name, file_size, mime_type).await?))
}

pub async fn delete_kyc_document(State(state): State<SharedState>, Path((id, doc_id)): Path<(i64, i64)>) -> Result<Json<MessageResponse>, AppError> {
    let svc = CustomerService::new(&state.db);
    Ok(Json(svc.delete_kyc_document(id, doc_id).await?))
}

// ── Customer Addresses ──────────────────────────────────────

pub async fn list_addresses(State(state): State<SharedState>, Path(id): Path<i64>) -> Result<Json<Vec<AddressResponse>>, AppError> {
    let svc = CustomerService::new(&state.db);
    Ok(Json(svc.list_addresses(id).await?))
}

pub async fn create_address(State(state): State<SharedState>, Path(id): Path<i64>, Json(req): Json<CreateAddressRequest>) -> Result<Json<AddressResponse>, AppError> {
    req.validate()?;
    let svc = CustomerService::new(&state.db);
    Ok(Json(svc.create_address(id, &req).await?))
}

pub async fn update_address(State(state): State<SharedState>, Path((id, addr_id)): Path<(i64, i64)>, Json(req): Json<UpdateAddressRequest>) -> Result<Json<AddressResponse>, AppError> {
    let svc = CustomerService::new(&state.db);
    Ok(Json(svc.update_address(id, addr_id, &req).await?))
}

pub async fn delete_address(State(state): State<SharedState>, Path((id, addr_id)): Path<(i64, i64)>) -> Result<Json<MessageResponse>, AppError> {
    let svc = CustomerService::new(&state.db);
    Ok(Json(svc.delete_address(id, addr_id).await?))
}
