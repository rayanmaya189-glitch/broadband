/// OpenAPI schemas and stub handlers for Payments endpoints.
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

// ── Request / Response types ─────────────────────────────────────────

#[derive(Debug, Serialize, ToSchema)]
pub struct PaymentLinkResponse {
    /// Payment link record ID
    pub id: i64,
    /// Unique link ID string
    pub link_id: String,
    /// Associated invoice ID
    pub invoice_id: i64,
    /// Payment amount
    pub amount: String,
    /// Currency code (e.g. INR)
    pub currency: String,
    /// Gateway identifier (e.g. razorpay, payu)
    pub gateway_id: String,
    /// Customer-facing payment URL
    pub payment_url: Option<String>,
    /// Payment status (pending, completed, failed, expired)
    pub status: String,
    /// Link expiry timestamp
    pub expires_at: Option<String>,
}

#[derive(Debug, Serialize, ToSchema)]
pub struct GatewayConfigResponse {
    /// Gateway config record ID
    pub id: i64,
    /// Gateway identifier
    pub gateway_id: String,
    /// Gateway display name
    pub name: String,
    /// Whether this is the primary gateway
    pub is_primary: bool,
    /// Whether gateway is enabled
    pub is_active: bool,
    /// Supported payment methods
    pub supported_methods: serde_json::Value,
    /// Supported currency
    pub currency: String,
}

#[derive(Debug, Deserialize, ToSchema)]
pub struct CreatePaymentLinkRequest {
    /// Invoice ID to link payment to
    pub invoice_id: i64,
    /// Customer ID
    pub customer_id: i64,
    /// Branch ID
    pub branch_id: i64,
    /// Payment amount
    pub amount: String,
    /// Currency code (default INR)
    #[serde(default)]
    pub currency: Option<String>,
    /// Gateway to use (default razorpay)
    #[serde(default)]
    pub gateway_id: Option<String>,
    /// Idempotency key (auto-generated if omitted)
    #[serde(default)]
    pub idempotency_key: Option<String>,
    /// Additional metadata
    #[serde(default)]
    pub metadata: Option<serde_json::Value>,
    /// Link expiry in hours (default 24)
    #[serde(default)]
    pub expires_in_hours: Option<i64>,
}

#[derive(Debug, Deserialize, ToSchema)]
pub struct ManualPaymentRequest {
    /// Invoice ID
    pub invoice_id: i64,
    /// Customer ID
    pub customer_id: i64,
    /// Branch ID
    pub branch_id: i64,
    /// Payment amount
    pub amount: String,
    /// Payment method (cash, cheque, bank_transfer)
    pub payment_method: String,
    /// Reference number
    #[serde(default)]
    pub reference_number: Option<String>,
    /// Payment notes
    #[serde(default)]
    pub notes: Option<String>,
}

#[derive(Debug, Deserialize, ToSchema)]
pub struct RetryPaymentRequest {
    /// Payment link ID to retry
    pub payment_link_id: i64,
    /// Optional gateway override
    #[serde(default)]
    pub gateway_id: Option<String>,
}

#[derive(Debug, Serialize, ToSchema)]
pub struct RazorpayOrderResponse {
    /// Razorpay order ID
    pub order_id: String,
    /// Payment amount in paise
    pub amount: i64,
    /// Currency
    pub currency: String,
    /// Razorpay key ID
    pub key_id: String,
}

#[derive(Debug, Serialize, ToSchema)]
pub struct PaymentVerificationResponse {
    /// Whether payment is valid
    pub verified: bool,
    /// Payment status
    pub status: String,
    /// Associated payment link ID
    pub payment_link_id: Option<i64>,
}

#[derive(Debug, Serialize, ToSchema)]
pub struct PaymentGatewayResponse {
    /// List of configured gateways
    pub gateways: Vec<GatewayConfigResponse>,
}

// ── Stub handler functions (for OpenAPI spec only) ───────────────────

/// Create a payment link via a gateway
#[utoipa::path(
    post,
    path = "/api/v1/payments/create-link",
    tag = "Payments",
    request_body = CreatePaymentLinkRequest,
    responses(
        (status = 201, description = "Payment link created", body = PaymentLinkResponse),
        (status = 403, description = "Insufficient permissions"),
        (status = 422, description = "Validation error")
    ),
    security(("bearer_auth" = []))
)]
pub async fn create_payment_link() -> axum::Json<serde_json::Value> {
    unimplemented!()
}

/// Record a manual (offline) payment
#[utoipa::path(
    post,
    path = "/api/v1/payments/manual",
    tag = "Payments",
    request_body = ManualPaymentRequest,
    responses(
        (status = 201, description = "Manual payment recorded", body = PaymentLinkResponse),
        (status = 403, description = "Insufficient permissions"),
        (status = 422, description = "Validation error")
    ),
    security(("bearer_auth" = []))
)]
pub async fn record_manual_payment() -> axum::Json<serde_json::Value> {
    unimplemented!()
}

/// Retry a failed payment
#[utoipa::path(
    post,
    path = "/api/v1/payments/{id}/retry",
    tag = "Payments",
    params(("id" = i64, Path, description = "Payment link ID")),
    request_body = RetryPaymentRequest,
    responses(
        (status = 200, description = "Payment retried", body = PaymentLinkResponse),
        (status = 403, description = "Insufficient permissions"),
        (status = 404, description = "Payment link not found"),
        (status = 422, description = "Payment already completed")
    ),
    security(("bearer_auth" = []))
)]
pub async fn retry_payment() -> axum::Json<serde_json::Value> {
    unimplemented!()
}

/// Razorpay webhook receiver
#[utoipa::path(
    post,
    path = "/api/v1/payments/webhook/razorpay",
    tag = "Payments",
    request_body = serde_json::Value,
    responses(
        (status = 200, description = "Webhook processed"),
        (status = 400, description = "Invalid webhook signature or payload")
    )
)]
pub async fn handle_razorpay_webhook() -> axum::Json<serde_json::Value> {
    unimplemented!()
}

/// PayU webhook receiver
#[utoipa::path(
    post,
    path = "/api/v1/payments/webhook/payu",
    tag = "Payments",
    request_body = serde_json::Value,
    responses(
        (status = 200, description = "Webhook processed"),
        (status = 400, description = "Invalid webhook payload")
    )
)]
pub async fn handle_payu_webhook() -> axum::Json<serde_json::Value> {
    unimplemented!()
}

/// List configured payment gateways
#[utoipa::path(
    get,
    path = "/api/v1/payments/gateways",
    tag = "Payments",
    responses(
        (status = 200, description = "List of payment gateways", body = Vec<GatewayConfigResponse>),
        (status = 403, description = "Insufficient permissions")
    ),
    security(("bearer_auth" = []))
)]
pub async fn list_gateways() -> axum::Json<serde_json::Value> {
    unimplemented!()
}

/// Get configuration for a specific gateway
#[utoipa::path(
    get,
    path = "/api/v1/payments/gateways/{gateway_id}",
    tag = "Payments",
    params(("gateway_id" = String, Path, description = "Gateway identifier")),
    responses(
        (status = 200, description = "Gateway configuration", body = GatewayConfigResponse),
        (status = 403, description = "Insufficient permissions"),
        (status = 404, description = "Gateway not found")
    ),
    security(("bearer_auth" = []))
)]
pub async fn get_gateway_config() -> axum::Json<serde_json::Value> {
    unimplemented!()
}
