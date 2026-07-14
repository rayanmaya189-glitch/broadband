use axum::extract::{Json, Path, Query, State};

use crate::app::SharedState;
use crate::common::errors::app_error::AppError;
use crate::common::middleware::auth_middleware::UserContext;
use crate::modules::billing::request::billing_request::*;
use crate::modules::billing::response::billing_response::*;
use crate::modules::billing::service::billing_service::BillingService;

/// List all invoices (admin).
pub async fn list_invoices(
    State(state): State<SharedState>,
    Query(query): Query<InvoiceQuery>,
) -> Result<Json<InvoiceListResponse>, AppError> {
    let svc = BillingService::new(&state.db);
    Ok(Json(svc.list_invoices(query).await?))
}

/// Get invoice by ID (admin).
pub async fn get_invoice(
    State(state): State<SharedState>,
    Path(id): Path<i64>,
) -> Result<Json<InvoiceResponse>, AppError> {
    let svc = BillingService::new(&state.db);
    Ok(Json(svc.get_invoice(id).await?))
}

/// Create an invoice (admin).
pub async fn create_invoice(
    State(state): State<SharedState>,
    Json(req): Json<CreateInvoiceRequest>,
) -> Result<Json<InvoiceResponse>, AppError> {
    let svc = BillingService::new(&state.db);
    Ok(Json(svc.create_invoice(req).await?))
}

/// Send an invoice to the customer (admin).
pub async fn send_invoice(
    State(state): State<SharedState>,
    Path(id): Path<i64>,
) -> Result<Json<InvoiceResponse>, AppError> {
    let svc = BillingService::new(&state.db);
    Ok(Json(svc.send_invoice(id).await?))
}

/// Void an invoice (admin).
pub async fn void_invoice(
    State(state): State<SharedState>,
    Path(id): Path<i64>,
) -> Result<Json<InvoiceResponse>, AppError> {
    let svc = BillingService::new(&state.db);
    Ok(Json(svc.void_invoice(id).await?))
}

/// Review an invoice (admin).
pub async fn review_invoice(
    State(state): State<SharedState>,
    user: UserContext,
    Path(id): Path<i64>,
    Json(req): Json<ReviewInvoiceRequest>,
) -> Result<Json<InvoiceResponse>, AppError> {
    let svc = BillingService::new(&state.db);
    Ok(Json(svc.review_invoice(id, &req.review_status, req.review_notes.as_deref(), user.user_id).await?))
}

/// Get line items for an invoice (admin).
pub async fn get_line_items(
    State(state): State<SharedState>,
    Path(id): Path<i64>,
) -> Result<Json<Vec<InvoiceLineItemResponse>>, AppError> {
    let svc = BillingService::new(&state.db);
    Ok(Json(svc.get_line_items(id).await?))
}

/// Record a payment (admin).
pub async fn record_payment(
    State(state): State<SharedState>,
    Json(req): Json<RecordPaymentRequest>,
) -> Result<Json<PaymentResponse>, AppError> {
    let svc = BillingService::new(&state.db);
    Ok(Json(svc.record_payment(req).await?))
}

/// List all payments (admin).
pub async fn list_payments(
    State(state): State<SharedState>,
    Query(query): Query<PaymentQuery>,
) -> Result<Json<PaymentListResponse>, AppError> {
    let svc = BillingService::new(&state.db);
    Ok(Json(svc.list_payments(query).await?))
}

/// Request a refund (admin).
pub async fn request_refund(
    State(state): State<SharedState>,
    Json(req): Json<CreateRefundRequest>,
) -> Result<Json<RefundResponse>, AppError> {
    let svc = BillingService::new(&state.db);
    Ok(Json(svc.request_refund(req).await?))
}

/// Approve a refund (admin).
pub async fn approve_refund(
    State(state): State<SharedState>,
    user: UserContext,
    Path(id): Path<i64>,
) -> Result<Json<RefundResponse>, AppError> {
    let svc = BillingService::new(&state.db);
    Ok(Json(svc.approve_refund(id, user.user_id).await?))
}

/// List discounts (admin).
pub async fn list_discounts(
    State(state): State<SharedState>,
) -> Result<Json<Vec<DiscountResponse>>, AppError> {
    let svc = BillingService::new(&state.db);
    Ok(Json(svc.list_discounts(1, 20).await?))
}

/// Create a discount (admin).
pub async fn create_discount(
    State(state): State<SharedState>,
    Json(req): Json<CreateDiscountRequest>,
) -> Result<Json<DiscountResponse>, AppError> {
    let svc = BillingService::new(&state.db);
    Ok(Json(svc.create_discount(req).await?))
}

/// Get dunning config (admin).
pub async fn get_dunning_config(
    State(state): State<SharedState>,
) -> Result<Json<serde_json::Value>, AppError> {
    let svc = BillingService::new(&state.db);
    Ok(Json(svc.get_dunning_config().await?))
}

/// Update dunning config (admin).
pub async fn update_dunning_config(
    State(state): State<SharedState>,
    Json(req): Json<BillingConfigRequest>,
) -> Result<Json<MessageResponse>, AppError> {
    let svc = BillingService::new(&state.db);
    Ok(Json(svc.update_dunning_config(req.config).await?))
}

/// Get tax config (admin).
pub async fn get_tax_config(
    State(state): State<SharedState>,
) -> Result<Json<serde_json::Value>, AppError> {
    let svc = BillingService::new(&state.db);
    Ok(Json(svc.get_tax_config().await?))
}

/// Update tax config (admin).
pub async fn update_tax_config(
    State(state): State<SharedState>,
    Json(req): Json<BillingConfigRequest>,
) -> Result<Json<MessageResponse>, AppError> {
    let svc = BillingService::new(&state.db);
    Ok(Json(svc.update_tax_config(req.config).await?))
}
