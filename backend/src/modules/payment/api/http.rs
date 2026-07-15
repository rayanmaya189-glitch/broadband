use axum::extract::{Path, State};
use axum::http::{HeaderMap, StatusCode};
use axum::Json;
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use tracing::{info, warn, debug};

use crate::shared::app_state::AppState;
use crate::shared::errors::AppError;
use crate::shared::middleware::auth::UserContext;
use crate::modules::payment::application::services::PaymentService;
use crate::modules::payment::infrastructure::gateway_adapter::{RazorpayAdapter, PayuAdapter, GatewayAdapter};

#[derive(Debug, Serialize)]
pub struct PaymentLinkResponse {
    pub id: i64,
    pub link_id: String,
    pub invoice_id: i64,
    pub amount: String,
    pub currency: String,
    pub gateway_id: String,
    pub payment_url: Option<String>,
    pub status: String,
    pub expires_at: Option<String>,
}

#[derive(Debug, Serialize)]
pub struct GatewayConfigResponse {
    pub id: i64,
    pub gateway_id: String,
    pub name: String,
    pub is_primary: bool,
    pub is_active: bool,
    pub supported_methods: serde_json::Value,
    pub currency: String,
}

#[derive(Debug, Deserialize)]
pub struct CreatePaymentLinkRequest {
    pub invoice_id: i64,
    pub customer_id: i64,
    pub branch_id: i64,
    pub amount: String,
    #[serde(default)]
    pub currency: Option<String>,
    #[serde(default)]
    pub gateway_id: Option<String>,
    #[serde(default)]
    pub idempotency_key: Option<String>,
    #[serde(default)]
    pub metadata: Option<serde_json::Value>,
    #[serde(default)]
    pub expires_in_hours: Option<i64>,
}

#[derive(Debug, Deserialize)]
pub struct ManualPaymentRequest {
    pub invoice_id: i64,
    pub customer_id: i64,
    pub branch_id: i64,
    pub amount: String,
    pub payment_method: String,
    #[serde(default)]
    pub reference_number: Option<String>,
    #[serde(default)]
    pub notes: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct RetryPaymentRequest {
    pub payment_link_id: i64,
    #[serde(default)]
    pub gateway_id: Option<String>,
}

// --- Payment Link Endpoints ---

/// POST /api/v1/payments/create-link
pub async fn create_payment_link(
    State(state): State<Arc<AppState>>,
    _user: UserContext,
    Json(req): Json<CreatePaymentLinkRequest>,
) -> Result<(StatusCode, Json<PaymentLinkResponse>), AppError> {
    let amount: sea_orm::prelude::Decimal = req.amount.parse()
        .map_err(|_| AppError::Validation("Invalid amount".into()))?;
    let currency = req.currency.unwrap_or_else(|| "INR".to_string());
    let gateway_id = req.gateway_id.unwrap_or_else(|| "razorpay".to_string());
    let idempotency_key = req.idempotency_key.unwrap_or_else(|| uuid::Uuid::new_v4().to_string());
    let expires_in_hours = req.expires_in_hours.unwrap_or(24);

    let link = PaymentService::create_payment_link(
        &state.db,
        req.invoice_id,
        req.customer_id,
        req.branch_id,
        amount,
        currency.clone(),
        gateway_id.clone(),
        idempotency_key,
        req.metadata,
        expires_in_hours,
    ).await?;

    Ok((StatusCode::CREATED, Json(PaymentLinkResponse {
        id: link.id,
        link_id: link.link_id,
        invoice_id: link.invoice_id,
        amount: link.amount.to_string(),
        currency: link.currency,
        gateway_id: link.gateway_id,
        payment_url: link.payment_url,
        status: link.status,
        expires_at: link.expires_at.map(|dt| dt.to_rfc3339()),
    })))
}

/// POST /api/v1/payments/manual
pub async fn record_manual_payment(
    State(state): State<Arc<AppState>>,
    user: UserContext,
    Json(req): Json<ManualPaymentRequest>,
) -> Result<(StatusCode, Json<PaymentLinkResponse>), AppError> {
    let amount: sea_orm::prelude::Decimal = req.amount.parse()
        .map_err(|_| AppError::Validation("Invalid amount".into()))?;

    let link = PaymentService::record_manual_payment(
        &state.db,
        req.invoice_id,
        req.customer_id,
        req.branch_id,
        amount,
        req.payment_method,
        req.reference_number,
        req.notes,
        user.user_id,
    ).await?;

    Ok((StatusCode::CREATED, Json(PaymentLinkResponse {
        id: link.id,
        link_id: link.link_id,
        invoice_id: link.invoice_id,
        amount: link.amount.to_string(),
        currency: link.currency,
        gateway_id: link.gateway_id,
        payment_url: link.payment_url,
        status: link.status,
        expires_at: link.expires_at.map(|dt| dt.to_rfc3339()),
    })))
}

/// POST /api/v1/payments/:id/retry
pub async fn retry_payment(
    State(state): State<Arc<AppState>>,
    _user: UserContext,
    Path(id): Path<i64>,
    Json(req): Json<RetryPaymentRequest>,
) -> Result<Json<PaymentLinkResponse>, AppError> {
    use sea_orm::EntityTrait;
    let link = crate::modules::payment::domain::entities::payment_link::Entity::find_by_id(id)
        .one(&state.db)
        .await
        .map_err(|e| AppError::Internal(anyhow::anyhow!("Failed to find payment link: {}", e)))?
        .ok_or_else(|| AppError::NotFound("Payment link not found".to_string()))?;

    if link.status == "completed" {
        return Err(AppError::Validation("Payment already completed".into()));
    }

    // In production, this would re-initiate the payment with the gateway
    let gateway_id = req.gateway_id.unwrap_or(link.gateway_id.clone());
    info!(link_id = %link.link_id, gateway = %gateway_id, "Retrying payment");

    Ok(Json(PaymentLinkResponse {
        id: link.id,
        link_id: link.link_id,
        invoice_id: link.invoice_id,
        amount: link.amount.to_string(),
        currency: link.currency,
        gateway_id: link.gateway_id,
        payment_url: link.payment_url,
        status: link.status,
        expires_at: link.expires_at.map(|dt| dt.to_rfc3339()),
    }))
}

// --- Webhook Endpoints ---

/// POST /api/v1/payments/webhook/razorpay
pub async fn handle_razorpay_webhook(
    State(state): State<Arc<AppState>>,
    headers: HeaderMap,
    body: axum::body::Bytes,
) -> Result<StatusCode, AppError> {
    // 1. Extract signature
    let signature = headers.get("X-Razorpay-Signature")
        .and_then(|v| v.to_str().ok())
        .unwrap_or("");

    // 2. Get gateway config
    let gateway = PaymentService::get_gateway_config(&state.db, "razorpay").await
        .unwrap_or_else(|_| {
            // Fallback for demo
            crate::modules::payment::domain::entities::gateway_config::Model {
                id: 0,
                gateway_id: "razorpay".to_string(),
                name: "Razorpay".to_string(),
                is_primary: true,
                is_active: true,
                credentials: serde_json::json!({}),
                webhook_secret: Some("test_secret".to_string()),
                fee_percentage: sea_orm::prelude::Decimal::from(2),
                fee_fixed: sea_orm::prelude::Decimal::from(0),
                gst_on_fee: sea_orm::prelude::Decimal::from(18),
                supported_methods: serde_json::json!(["upi", "card", "netbanking"]),
                currency: "INR".to_string(),
                created_at: chrono::Utc::now(),
                updated_at: chrono::Utc::now(),
            }
        });

    // 3. Verify signature
    let adapter = RazorpayAdapter {
        key_id: String::new(),
        key_secret: String::new(),
        webhook_secret: gateway.webhook_secret.unwrap_or_default(),
    };

    if !adapter.verify_webhook_signature(&body, signature, &adapter.webhook_secret).map_err(|e| AppError::Internal(anyhow::anyhow!("Signature verification failed: {}", e)))? {
        warn!("Invalid Razorpay webhook signature");
        return Err(AppError::Validation("Invalid webhook signature".into()));
    }

    // 4. Parse payload
    let payload: serde_json::Value = serde_json::from_slice(&body)
        .map_err(|e| AppError::Validation(format!("Invalid JSON: {}", e)))?;

    let webhook = adapter.parse_webhook(payload.clone())
        .map_err(|e| AppError::Internal(anyhow::anyhow!("Failed to parse webhook: {}", e)))?;

    // 5. Idempotency check
    let already_processed = PaymentService::log_webhook(
        &state.db,
        "razorpay",
        &webhook.event_id,
        &webhook.event_type,
        payload,
    ).await?;

    if already_processed {
        return Ok(StatusCode::OK);
    }

    // 6. Process based on event type
    match webhook.event_type.as_str() {
        "payment.captured" | "payment.authorized" => {
            PaymentService::process_successful_payment(
                &state.db,
                "razorpay",
                &webhook.transaction_id,
                webhook.amount,
                webhook.payment_method,
            ).await?;
        }
        "payment.failed" => {
            PaymentService::process_failed_payment(
                &state.db,
                "razorpay",
                &webhook.transaction_id,
                webhook.error_reason,
            ).await?;
        }
        "refund.created" | "refund.processed" => {
            info!(transaction_id = %webhook.transaction_id, "Refund processed");
        }
        _ => {
            debug!(event = %webhook.event_type, "Unhandled Razorpay event");
        }
    }

    // 7. Mark webhook as processed
    PaymentService::mark_webhook_processed(&state.db, "razorpay", &webhook.event_id).await?;

    Ok(StatusCode::OK)
}

/// POST /api/v1/payments/webhook/payu
pub async fn handle_payu_webhook(
    State(state): State<Arc<AppState>>,
    _headers: HeaderMap,
    body: axum::body::Bytes,
) -> Result<StatusCode, AppError> {
    let payload: serde_json::Value = serde_json::from_slice(&body)
        .map_err(|e| AppError::Validation(format!("Invalid JSON: {}", e)))?;

    let adapter = PayuAdapter::from_env();

    let webhook = adapter.parse_webhook(payload.clone())
        .map_err(|e| AppError::Internal(anyhow::anyhow!("Failed to parse webhook: {}", e)))?;

    // Idempotency check
    let already_processed = PaymentService::log_webhook(
        &state.db,
        "payu",
        &webhook.event_id,
        &webhook.event_type,
        payload,
    ).await?;

    if already_processed {
        return Ok(StatusCode::OK);
    }

    // Process based on status
    match webhook.status.as_str() {
        "success" => {
            PaymentService::process_successful_payment(
                &state.db,
                "payu",
                &webhook.transaction_id,
                webhook.amount,
                webhook.payment_method,
            ).await?;
        }
        "failure" | "drop" => {
            PaymentService::process_failed_payment(
                &state.db,
                "payu",
                &webhook.transaction_id,
                webhook.error_reason,
            ).await?;
        }
        _ => {
            debug!(status = %webhook.status, "Unhandled PayU status");
        }
    }

    // Mark webhook as processed
    PaymentService::mark_webhook_processed(&state.db, "payu", &webhook.event_id).await?;

    Ok(StatusCode::OK)
}

// --- Gateway Config Endpoints ---

/// GET /api/v1/payments/gateways
pub async fn list_gateways(
    State(state): State<Arc<AppState>>,
    _user: UserContext,
) -> Result<Json<Vec<GatewayConfigResponse>>, AppError> {
    let gateways = PaymentService::list_gateways(&state.db).await?;
    Ok(Json(gateways.into_iter().map(|g| GatewayConfigResponse {
        id: g.id,
        gateway_id: g.gateway_id,
        name: g.name,
        is_primary: g.is_primary,
        is_active: g.is_active,
        supported_methods: g.supported_methods,
        currency: g.currency,
    }).collect()))
}
