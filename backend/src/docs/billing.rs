/// OpenAPI schemas and stub handlers for Billing endpoints.
use serde::{Deserialize, Serialize};
use utoipa::{IntoParams, ToSchema};

// ── Request / Response types ─────────────────────────────────────────

#[derive(Debug, Serialize, ToSchema)]
pub struct InvoiceResponse {
    pub id: i64,
    pub invoice_number: String,
    pub customer_id: i64,
    pub subscription_id: Option<i64>,
    pub total_amount: String,
    pub status: String,
    pub due_date: String,
}

#[derive(Debug, Deserialize, ToSchema)]
pub struct CreateInvoiceRequest {
    pub customer_id: i64,
    pub branch_id: i64,
    pub subscription_id: i64,
    pub billing_period_start: String,
    pub billing_period_end: String,
    pub total_amount: String,
}

#[derive(Debug, Serialize, ToSchema)]
pub struct PaymentResponse {
    pub id: i64,
    pub payment_number: String,
    pub invoice_id: i64,
    pub amount: String,
    pub payment_method: String,
    pub status: String,
}

#[derive(Debug, Deserialize, ToSchema)]
pub struct RecordPaymentRequest {
    pub invoice_id: i64,
    pub customer_id: i64,
    pub branch_id: i64,
    pub amount: String,
    pub payment_method: String,
}

#[derive(Debug, Deserialize, IntoParams)]
pub struct InvoiceListParams {
    pub page: Option<i64>,
    pub limit: Option<i64>,
    pub status: Option<String>,
    pub customer_id: Option<i64>,
}

#[derive(Debug, Deserialize, IntoParams)]
pub struct PaymentListParams {
    pub page: Option<i64>,
    pub limit: Option<i64>,
    pub invoice_id: Option<i64>,
}

// ── Stub handler functions ───────────────────────────────────────────

/// List all invoices with optional filters
#[utoipa::path(
    get,
    path = "/api/v1/billing/invoices",
    tag = "Billing",
    params(InvoiceListParams),
    responses(
        (status = 200, description = "List of invoices"),
        (status = 403, description = "Insufficient permissions")
    ),
    security(("bearer_auth" = []))
)]
pub async fn list_invoices() -> axum::Json<serde_json::Value> {
    unimplemented!()
}

/// Create a new invoice
#[utoipa::path(
    post,
    path = "/api/v1/billing/invoices",
    tag = "Billing",
    request_body = CreateInvoiceRequest,
    responses(
        (status = 201, description = "Invoice created", body = InvoiceResponse),
        (status = 400, description = "Invalid request")
    ),
    security(("bearer_auth" = []))
)]
pub async fn create_invoice() -> axum::Json<serde_json::Value> {
    unimplemented!()
}

/// List all payments with optional filters
#[utoipa::path(
    get,
    path = "/api/v1/billing/payments",
    tag = "Billing",
    params(PaymentListParams),
    responses(
        (status = 200, description = "List of payments"),
        (status = 403, description = "Insufficient permissions")
    ),
    security(("bearer_auth" = []))
)]
pub async fn list_payments() -> axum::Json<serde_json::Value> {
    unimplemented!()
}

/// Record a payment against an invoice
#[utoipa::path(
    post,
    path = "/api/v1/billing/payments",
    tag = "Billing",
    request_body = RecordPaymentRequest,
    responses(
        (status = 201, description = "Payment recorded", body = PaymentResponse),
        (status = 400, description = "Invalid payment amount")
    ),
    security(("bearer_auth" = []))
)]
pub async fn record_payment() -> axum::Json<serde_json::Value> {
    unimplemented!()
}

/// List all overdue invoices
#[utoipa::path(
    get,
    path = "/api/v1/billing/invoices/overdue",
    tag = "Billing",
    responses(
        (status = 200, description = "List of overdue invoices"),
        (status = 403, description = "Insufficient permissions")
    ),
    security(("bearer_auth" = []))
)]
pub async fn list_overdue_invoices() -> axum::Json<serde_json::Value> {
    unimplemented!()
}

/// Auto-generate invoices for subscriptions due in the current billing cycle
#[utoipa::path(
    post,
    path = "/api/v1/billing/invoices/auto-generate",
    tag = "Billing",
    responses(
        (status = 200, description = "Invoices generated successfully"),
        (status = 403, description = "Insufficient permissions")
    ),
    security(("bearer_auth" = []))
)]
pub async fn auto_generate_invoices() -> axum::Json<serde_json::Value> {
    unimplemented!()
}
