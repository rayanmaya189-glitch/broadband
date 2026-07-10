use axum::extract::{Json, Path, Query, State};
use validator::Validate;

use crate::app::SharedState;
use crate::common::errors::app_error::AppError;
use crate::common::middleware::auth_middleware::UserContext;
use crate::modules::billing::request::billing_request::*;
use crate::modules::billing::response::billing_response::*;
use crate::modules::billing::service::billing_service_seaorm::BillingServiceSeaorm;

// ── Invoices ────────────────────────────────────────────────

pub async fn list_invoices(State(state): State<SharedState>, Query(query): Query<InvoiceQuery>) -> Result<Json<InvoiceListResponse>, AppError> {
    let svc = BillingServiceSeaorm::new(&state.db_seaorm);
    Ok(Json(svc.list_invoices(query).await?))
}

pub async fn get_invoice(State(state): State<SharedState>, Path(id): Path<i64>) -> Result<Json<InvoiceResponse>, AppError> {
    let svc = BillingServiceSeaorm::new(&state.db_seaorm);
    Ok(Json(svc.get_invoice(id).await?))
}

pub async fn create_invoice(State(state): State<SharedState>, Json(req): Json<CreateInvoiceRequest>) -> Result<Json<InvoiceResponse>, AppError> {
    req.validate()?;
    let svc = BillingServiceSeaorm::new(&state.db_seaorm);
    Ok(Json(svc.create_invoice(req).await?))
}

pub async fn send_invoice(State(state): State<SharedState>, Path(id): Path<i64>) -> Result<Json<InvoiceResponse>, AppError> {
    let svc = BillingServiceSeaorm::new(&state.db_seaorm);
    Ok(Json(svc.send_invoice(id).await?))
}

pub async fn void_invoice(State(state): State<SharedState>, Path(id): Path<i64>) -> Result<Json<InvoiceResponse>, AppError> {
    let svc = BillingServiceSeaorm::new(&state.db_seaorm);
    Ok(Json(svc.void_invoice(id).await?))
}

pub async fn get_line_items(State(state): State<SharedState>, Path(id): Path<i64>) -> Result<Json<Vec<InvoiceLineItemResponse>>, AppError> {
    let svc = BillingServiceSeaorm::new(&state.db_seaorm);
    Ok(Json(svc.get_line_items(id).await?))
}

pub async fn review_invoice(State(state): State<SharedState>, user: UserContext, Path(id): Path<i64>, Json(req): Json<ReviewInvoiceRequest>) -> Result<Json<InvoiceResponse>, AppError> {
    req.validate()?;
    let svc = BillingServiceSeaorm::new(&state.db_seaorm);
    Ok(Json(svc.review_invoice(id, &req.review_status, req.review_notes.as_deref(), user.user_id).await?))
}

// ── Payments ────────────────────────────────────────────────

pub async fn record_payment(State(state): State<SharedState>, Json(req): Json<RecordPaymentRequest>) -> Result<Json<PaymentResponse>, AppError> {
    req.validate()?;
    let svc = BillingServiceSeaorm::new(&state.db_seaorm);
    Ok(Json(svc.record_payment(req).await?))
}

pub async fn list_payments(State(state): State<SharedState>, Query(query): Query<PaymentQuery>) -> Result<Json<PaymentListResponse>, AppError> {
    let svc = BillingServiceSeaorm::new(&state.db_seaorm);
    Ok(Json(svc.list_payments(query).await?))
}

// ── Refunds ─────────────────────────────────────────────────

pub async fn request_refund(State(state): State<SharedState>, Json(req): Json<CreateRefundRequest>) -> Result<Json<RefundResponse>, AppError> {
    req.validate()?;
    let svc = BillingServiceSeaorm::new(&state.db_seaorm);
    Ok(Json(svc.request_refund(req).await?))
}

pub async fn approve_refund(State(state): State<SharedState>, user: UserContext, Path(id): Path<i64>) -> Result<Json<RefundResponse>, AppError> {
    let svc = BillingServiceSeaorm::new(&state.db_seaorm);
    Ok(Json(svc.approve_refund(id, user.user_id).await?))
}

// ── Discounts ───────────────────────────────────────────────

pub async fn list_discounts(State(state): State<SharedState>) -> Result<Json<Vec<DiscountResponse>>, AppError> {
    let svc = BillingServiceSeaorm::new(&state.db_seaorm);
    Ok(Json(svc.list_discounts(1, 100).await?))
}

pub async fn create_discount(State(state): State<SharedState>, Json(req): Json<CreateDiscountRequest>) -> Result<Json<DiscountResponse>, AppError> {
    req.validate()?;
    let svc = BillingServiceSeaorm::new(&state.db_seaorm);
    Ok(Json(svc.create_discount(req).await?))
}

// ── Dunning & Tax Config ────────────────────────────────────

pub async fn get_dunning_config(State(state): State<SharedState>) -> Result<Json<serde_json::Value>, AppError> {
    let svc = BillingServiceSeaorm::new(&state.db_seaorm);
    Ok(Json(svc.get_dunning_config().await?))
}

pub async fn update_dunning_config(State(state): State<SharedState>, Json(req): Json<BillingConfigRequest>) -> Result<Json<MessageResponse>, AppError> {
    let svc = BillingServiceSeaorm::new(&state.db_seaorm);
    Ok(Json(svc.update_dunning_config(req.config).await?))
}

pub async fn get_tax_config(State(state): State<SharedState>) -> Result<Json<serde_json::Value>, AppError> {
    let svc = BillingServiceSeaorm::new(&state.db_seaorm);
    Ok(Json(svc.get_tax_config().await?))
}

pub async fn update_tax_config(State(state): State<SharedState>, Json(req): Json<BillingConfigRequest>) -> Result<Json<MessageResponse>, AppError> {
    let svc = BillingServiceSeaorm::new(&state.db_seaorm);
    Ok(Json(svc.update_tax_config(req.config).await?))
}
