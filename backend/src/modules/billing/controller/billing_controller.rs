use axum::extract::{Json, Path, Query, State};
use validator::Validate;

use crate::app::SharedState;
use crate::common::errors::app_error::AppError;
use crate::common::middleware::auth_middleware::UserContext;
use crate::modules::billing::request::billing_request::*;
use crate::modules::billing::response::billing_response::*;
use crate::modules::billing::service::billing_service::BillingService;

// ── Invoices ────────────────────────────────────────────────

#[utoipa::path(
    get,
    path = "/api/v1/billing/invoices",
    tag = "Billing",
    security(("bearer_auth" = [])),
    params(
        ("page" = Option<i64>, Query, description = "Page number"),
        ("limit" = Option<i64>, Query, description = "Items per page"),
        ("customer_id" = Option<i64>, Query, description = "Filter by customer"),
        ("status" = Option<String>, Query, description = "Filter by status")
    ),
    responses(
        (status = 200, description = "List of invoices"),
        (status = 401, description = "Unauthorized")
    )
)]
pub async fn list_invoices(State(state): State<SharedState>, Query(query): Query<InvoiceQuery>) -> Result<Json<InvoiceListResponse>, AppError> {
    let svc = BillingService::new(&state.db);
    Ok(Json(svc.list_invoices(query).await?))
}

#[utoipa::path(
    get,
    path = "/api/v1/billing/invoices/{id}",
    tag = "Billing",
    security(("bearer_auth" = [])),
    params(("id" = i64, Path, description = "Invoice ID")),
    responses(
        (status = 200, description = "Invoice details", body = InvoiceResponse),
        (status = 404, description = "Invoice not found")
    )
)]
pub async fn get_invoice(State(state): State<SharedState>, Path(id): Path<i64>) -> Result<Json<InvoiceResponse>, AppError> {
    let svc = BillingService::new(&state.db);
    Ok(Json(svc.get_invoice(id).await?))
}

#[utoipa::path(
    post,
    path = "/api/v1/billing/invoices",
    tag = "Billing",
    security(("bearer_auth" = [])),
    request_body = CreateInvoiceRequest,
    responses(
        (status = 200, description = "Invoice created", body = InvoiceResponse),
        (status = 422, description = "Validation error")
    )
)]
pub async fn create_invoice(State(state): State<SharedState>, Json(req): Json<CreateInvoiceRequest>) -> Result<Json<InvoiceResponse>, AppError> {
    req.validate()?;
    let svc = BillingService::new(&state.db);
    Ok(Json(svc.create_invoice(req).await?))
}

#[utoipa::path(
    post,
    path = "/api/v1/billing/invoices/{id}/send",
    tag = "Billing",
    security(("bearer_auth" = [])),
    params(("id" = i64, Path, description = "Invoice ID")),
    responses(
        (status = 200, description = "Invoice sent", body = InvoiceResponse),
        (status = 404, description = "Invoice not found")
    )
)]
pub async fn send_invoice(State(state): State<SharedState>, Path(id): Path<i64>) -> Result<Json<InvoiceResponse>, AppError> {
    let svc = BillingService::new(&state.db);
    Ok(Json(svc.send_invoice(id).await?))
}

#[utoipa::path(
    post,
    path = "/api/v1/billing/invoices/{id}/void",
    tag = "Billing",
    security(("bearer_auth" = [])),
    params(("id" = i64, Path, description = "Invoice ID")),
    responses(
        (status = 200, description = "Invoice voided", body = InvoiceResponse),
        (status = 404, description = "Invoice not found")
    )
)]
pub async fn void_invoice(State(state): State<SharedState>, Path(id): Path<i64>) -> Result<Json<InvoiceResponse>, AppError> {
    let svc = BillingService::new(&state.db);
    Ok(Json(svc.void_invoice(id).await?))
}

#[utoipa::path(
    get,
    path = "/api/v1/billing/invoices/{id}/line-items",
    tag = "Billing",
    security(("bearer_auth" = [])),
    params(("id" = i64, Path, description = "Invoice ID")),
    responses(
        (status = 200, description = "List of line items", body = Vec<InvoiceLineItemResponse>),
        (status = 404, description = "Invoice not found")
    )
)]
pub async fn get_line_items(State(state): State<SharedState>, Path(id): Path<i64>) -> Result<Json<Vec<InvoiceLineItemResponse>>, AppError> {
    let svc = BillingService::new(&state.db);
    Ok(Json(svc.get_line_items(id).await?))
}

#[utoipa::path(
    post,
    path = "/api/v1/billing/invoices/{id}/review",
    tag = "Billing",
    security(("bearer_auth" = [])),
    params(("id" = i64, Path, description = "Invoice ID")),
    request_body = ReviewInvoiceRequest,
    responses(
        (status = 200, description = "Invoice reviewed", body = InvoiceResponse),
        (status = 404, description = "Invoice not found"),
        (status = 422, description = "Validation error")
    )
)]
pub async fn review_invoice(State(state): State<SharedState>, user: UserContext, Path(id): Path<i64>, Json(req): Json<ReviewInvoiceRequest>) -> Result<Json<InvoiceResponse>, AppError> {
    req.validate()?;
    let svc = BillingService::new(&state.db);
    Ok(Json(svc.review_invoice(id, &req.review_status, req.review_notes.as_deref(), user.user_id).await?))
}

// ── Payments ────────────────────────────────────────────────

#[utoipa::path(
    post,
    path = "/api/v1/billing/payments",
    tag = "Billing",
    security(("bearer_auth" = [])),
    request_body = RecordPaymentRequest,
    responses(
        (status = 200, description = "Payment recorded", body = PaymentResponse),
        (status = 422, description = "Validation error")
    )
)]
pub async fn record_payment(State(state): State<SharedState>, Json(req): Json<RecordPaymentRequest>) -> Result<Json<PaymentResponse>, AppError> {
    req.validate()?;
    let svc = BillingService::new(&state.db);
    Ok(Json(svc.record_payment(req).await?))
}

#[utoipa::path(
    get,
    path = "/api/v1/billing/payments",
    tag = "Billing",
    security(("bearer_auth" = [])),
    params(
        ("page" = Option<i64>, Query, description = "Page number"),
        ("limit" = Option<i64>, Query, description = "Items per page"),
        ("invoice_id" = Option<i64>, Query, description = "Filter by invoice"),
        ("customer_id" = Option<i64>, Query, description = "Filter by customer")
    ),
    responses(
        (status = 200, description = "List of payments"),
        (status = 401, description = "Unauthorized")
    )
)]
pub async fn list_payments(State(state): State<SharedState>, Query(query): Query<PaymentQuery>) -> Result<Json<PaymentListResponse>, AppError> {
    let svc = BillingService::new(&state.db);
    Ok(Json(svc.list_payments(query).await?))
}

// ── Refunds ─────────────────────────────────────────────────

#[utoipa::path(
    post,
    path = "/api/v1/billing/refunds",
    tag = "Billing",
    security(("bearer_auth" = [])),
    request_body = CreateRefundRequest,
    responses(
        (status = 200, description = "Refund requested", body = RefundResponse),
        (status = 422, description = "Validation error")
    )
)]
pub async fn request_refund(State(state): State<SharedState>, Json(req): Json<CreateRefundRequest>) -> Result<Json<RefundResponse>, AppError> {
    req.validate()?;
    let svc = BillingService::new(&state.db);
    Ok(Json(svc.request_refund(req).await?))
}

#[utoipa::path(
    post,
    path = "/api/v1/billing/refunds/{id}/approve",
    tag = "Billing",
    security(("bearer_auth" = [])),
    params(("id" = i64, Path, description = "Refund ID")),
    responses(
        (status = 200, description = "Refund approved", body = RefundResponse),
        (status = 404, description = "Refund not found")
    )
)]
pub async fn approve_refund(State(state): State<SharedState>, user: UserContext, Path(id): Path<i64>) -> Result<Json<RefundResponse>, AppError> {
    let svc = BillingService::new(&state.db);
    Ok(Json(svc.approve_refund(id, user.user_id).await?))
}

// ── Discounts ───────────────────────────────────────────────

#[utoipa::path(
    get,
    path = "/api/v1/billing/discounts",
    tag = "Billing",
    security(("bearer_auth" = [])),
    responses(
        (status = 200, description = "List of discounts", body = Vec<DiscountResponse>),
        (status = 401, description = "Unauthorized")
    )
)]
pub async fn list_discounts(State(state): State<SharedState>) -> Result<Json<Vec<DiscountResponse>>, AppError> {
    let svc = BillingService::new(&state.db);
    Ok(Json(svc.list_discounts(1, 100).await?))
}

#[utoipa::path(
    post,
    path = "/api/v1/billing/discounts",
    tag = "Billing",
    security(("bearer_auth" = [])),
    request_body = CreateDiscountRequest,
    responses(
        (status = 200, description = "Discount created", body = DiscountResponse),
        (status = 422, description = "Validation error")
    )
)]
pub async fn create_discount(State(state): State<SharedState>, Json(req): Json<CreateDiscountRequest>) -> Result<Json<DiscountResponse>, AppError> {
    req.validate()?;
    let svc = BillingService::new(&state.db);
    Ok(Json(svc.create_discount(req).await?))
}

// ── Dunning & Tax Config ────────────────────────────────────

#[utoipa::path(
    get,
    path = "/api/v1/billing/dunning/config",
    tag = "Billing",
    security(("bearer_auth" = [])),
    responses(
        (status = 200, description = "Dunning configuration"),
        (status = 401, description = "Unauthorized")
    )
)]
pub async fn get_dunning_config(State(state): State<SharedState>) -> Result<Json<serde_json::Value>, AppError> {
    let svc = BillingService::new(&state.db);
    Ok(Json(svc.get_dunning_config().await?))
}

#[utoipa::path(
    put,
    path = "/api/v1/billing/dunning/config",
    tag = "Billing",
    security(("bearer_auth" = [])),
    request_body = BillingConfigRequest,
    responses(
        (status = 200, description = "Dunning config updated"),
        (status = 422, description = "Validation error")
    )
)]
pub async fn update_dunning_config(State(state): State<SharedState>, Json(req): Json<BillingConfigRequest>) -> Result<Json<MessageResponse>, AppError> {
    let svc = BillingService::new(&state.db);
    Ok(Json(svc.update_dunning_config(req.config).await?))
}

#[utoipa::path(
    get,
    path = "/api/v1/billing/tax/config",
    tag = "Billing",
    security(("bearer_auth" = [])),
    responses(
        (status = 200, description = "Tax configuration"),
        (status = 401, description = "Unauthorized")
    )
)]
pub async fn get_tax_config(State(state): State<SharedState>) -> Result<Json<serde_json::Value>, AppError> {
    let svc = BillingService::new(&state.db);
    Ok(Json(svc.get_tax_config().await?))
}

#[utoipa::path(
    put,
    path = "/api/v1/billing/tax/config",
    tag = "Billing",
    security(("bearer_auth" = [])),
    request_body = BillingConfigRequest,
    responses(
        (status = 200, description = "Tax config updated"),
        (status = 422, description = "Validation error")
    )
)]
pub async fn update_tax_config(State(state): State<SharedState>, Json(req): Json<BillingConfigRequest>) -> Result<Json<MessageResponse>, AppError> {
    let svc = BillingService::new(&state.db);
    Ok(Json(svc.update_tax_config(req.config).await?))
}
