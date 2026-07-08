use axum::extract::{Json, Path, Query, State};
use validator::Validate;

use crate::app::SharedState;
use crate::common::errors::app_error::AppError;
use crate::modules::billing::request::billing_request::*;
use crate::modules::billing::response::billing_response::*;
use crate::modules::billing::service::billing_service::BillingService;

pub async fn list_invoices(State(state): State<SharedState>, Query(query): Query<InvoiceQuery>) -> Result<Json<InvoiceListResponse>, AppError> {
    let svc = BillingService::new(&state.db);
    Ok(Json(svc.list_invoices(query).await?))
}

pub async fn get_invoice(State(state): State<SharedState>, Path(id): Path<i64>) -> Result<Json<InvoiceResponse>, AppError> {
    let svc = BillingService::new(&state.db);
    Ok(Json(svc.get_invoice(id).await?))
}

pub async fn create_invoice(State(state): State<SharedState>, Json(req): Json<CreateInvoiceRequest>) -> Result<Json<InvoiceResponse>, AppError> {
    req.validate()?;
    let svc = BillingService::new(&state.db);
    Ok(Json(svc.create_invoice(req).await?))
}

pub async fn record_payment(State(state): State<SharedState>, Json(req): Json<RecordPaymentRequest>) -> Result<Json<PaymentResponse>, AppError> {
    req.validate()?;
    let svc = BillingService::new(&state.db);
    Ok(Json(svc.record_payment(req).await?))
}

pub async fn list_payments(State(state): State<SharedState>, Query(query): Query<PaymentQuery>) -> Result<Json<PaymentListResponse>, AppError> {
    let svc = BillingService::new(&state.db);
    Ok(Json(svc.list_payments(query).await?))
}

pub async fn request_refund(State(state): State<SharedState>, Json(req): Json<CreateRefundRequest>) -> Result<Json<RefundResponse>, AppError> {
    req.validate()?;
    let svc = BillingService::new(&state.db);
    Ok(Json(svc.request_refund(req).await?))
}

pub async fn approve_refund(State(state): State<SharedState>, Path(id): Path<i64>) -> Result<Json<RefundResponse>, AppError> {
    let svc = BillingService::new(&state.db);
    Ok(Json(svc.approve_refund(id, 1).await?))
}

pub async fn list_discounts(State(state): State<SharedState>) -> Result<Json<Vec<DiscountResponse>>, AppError> {
    let svc = BillingService::new(&state.db);
    Ok(Json(svc.list_discounts(1, 100).await?))
}

pub async fn create_discount(State(state): State<SharedState>, Json(req): Json<CreateDiscountRequest>) -> Result<Json<DiscountResponse>, AppError> {
    req.validate()?;
    let svc = BillingService::new(&state.db);
    Ok(Json(svc.create_discount(req).await?))
}
