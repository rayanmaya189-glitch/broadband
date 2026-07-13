use axum::extract::{Json, Path, Query, State};
use axum::http::HeaderMap;
use bytes::Bytes;
use validator::Validate;
use crate::app::SharedState;
use crate::common::errors::app_error::AppError;
use crate::common::utils::webhook_verify;
use crate::modules::payment_gateway::request::payment_gateway_request::*;
use crate::modules::payment_gateway::response::payment_gateway_response::*;
use crate::modules::payment_gateway::service::payment_gateway_service::PaymentGatewayService;

#[utoipa::path(
    get,
    path = "/api/v1/payments/gateways",
    tag = "Payment Gateway",
    security(("bearer_auth" = [])),
    responses(
        (status = 200, description = "List of gateways", body = Vec<GatewayConfigResponse>),
        (status = 401, description = "Unauthorized")
    )
)]
pub async fn list_gateways(State(state): State<SharedState>) -> Result<Json<Vec<GatewayConfigResponse>>, AppError> {
    let svc = PaymentGatewayService::new(&state.db);
    Ok(Json(svc.list_gateways().await?))
}

#[utoipa::path(
    post,
    path = "/api/v1/payments/gateways",
    tag = "Payment Gateway",
    security(("bearer_auth" = [])),
    request_body = CreateGatewayConfigRequest,
    responses(
        (status = 200, description = "Gateway created", body = GatewayConfigResponse),
        (status = 422, description = "Validation error")
    )
)]
pub async fn create_gateway(State(state): State<SharedState>, Json(req): Json<CreateGatewayConfigRequest>) -> Result<Json<GatewayConfigResponse>, AppError> {
    req.validate()?;
    let svc = PaymentGatewayService::new(&state.db);
    Ok(Json(svc.create_gateway(req).await?))
}

#[utoipa::path(
    put,
    path = "/api/v1/payments/gateways/{id}",
    tag = "Payment Gateway",
    security(("bearer_auth" = [])),
    params(("id" = i64, Path, description = "Gateway ID")),
    request_body = UpdateGatewayRequest,
    responses(
        (status = 200, description = "Gateway updated", body = GatewayConfigResponse),
        (status = 404, description = "Gateway not found")
    )
)]
pub async fn update_gateway(State(state): State<SharedState>, Path(id): Path<i64>, Json(req): Json<UpdateGatewayRequest>) -> Result<Json<GatewayConfigResponse>, AppError> {
    let svc = PaymentGatewayService::new(&state.db);
    Ok(Json(svc.update_gateway(id, req).await?))
}

#[utoipa::path(
    post,
    path = "/api/v1/payments/create-link",
    tag = "Payment Gateway",
    security(("bearer_auth" = [])),
    request_body = CreatePaymentLinkRequest,
    responses(
        (status = 200, description = "Payment link created", body = PaymentLinkResponse),
        (status = 422, description = "Validation error")
    )
)]
pub async fn create_payment_link(State(state): State<SharedState>, Json(req): Json<CreatePaymentLinkRequest>) -> Result<Json<PaymentLinkResponse>, AppError> {
    req.validate()?;
    let svc = PaymentGatewayService::new(&state.db);
    Ok(Json(svc.create_payment_link(req).await?))
}

#[utoipa::path(
    get,
    path = "/api/v1/payments",
    tag = "Payment Gateway",
    security(("bearer_auth" = [])),
    params(
        ("page" = Option<i64>, Query, description = "Page number"),
        ("per_page" = Option<i64>, Query, description = "Items per page"),
        ("gateway_id" = Option<String>, Query, description = "Filter by gateway"),
        ("status" = Option<String>, Query, description = "Filter by status")
    ),
    responses(
        (status = 200, description = "List of transactions"),
        (status = 401, description = "Unauthorized")
    )
)]
pub async fn list_transactions(State(state): State<SharedState>, Query(q): Query<TransactionQuery>) -> Result<Json<TransactionListResponse>, AppError> {
    let svc = PaymentGatewayService::new(&state.db);
    Ok(Json(svc.list_transactions(q).await?))
}

/// Razorpay webhook handler with signature verification.
// Note: Not annotated with #[utoipa::path] because it accepts raw Bytes
// which doesn't implement ToSchema. Documented manually in openapi.rs.
pub async fn process_webhook_razorpay(
    State(state): State<SharedState>,
    headers: HeaderMap,
    body: Bytes,
) -> Result<Json<WebhookProcessResponse>, AppError> {
    let signature = headers
        .get("X-Razorpay-Signature")
        .and_then(|v| v.to_str().ok())
        .ok_or_else(|| AppError::Validation("Missing X-Razorpay-Signature header".into()))?;

    let secret = get_webhook_secret(&state, "razorpay").await?;
    webhook_verify::verify_razorpay(&body, signature, &secret)
        .map_err(|e| AppError::Forbidden(format!("Webhook verification failed: {}", e)))?;

    let raw: serde_json::Value = serde_json::from_slice(&body)
        .map_err(|e| AppError::Validation(format!("Invalid webhook payload: {}", e)))?;

    let event_type = raw.get("event").and_then(|v| v.as_str()).unwrap_or("unknown");
    let payload_data = raw.get("payload").cloned().unwrap_or(raw.clone());

    let svc = PaymentGatewayService::new(&state.db);
    let webhook_req = WebhookPayload {
        event_type: event_type.to_string(),
        gateway_id: "razorpay".into(),
        payload: payload_data,
        signature: Some(signature.to_string()),
    };
    Ok(Json(svc.process_webhook(webhook_req).await?))
}

/// PayU webhook handler with signature verification.
// Note: Not annotated with #[utoipa::path] because it accepts raw Bytes
// which doesn't implement ToSchema. Documented manually in openapi.rs.
pub async fn process_webhook_payu(
    State(state): State<SharedState>,
    headers: HeaderMap,
    body: Bytes,
) -> Result<Json<WebhookProcessResponse>, AppError> {
    let signature = headers
        .get("X-PayU-Signature")
        .and_then(|v| v.to_str().ok())
        .ok_or_else(|| AppError::Validation("Missing X-PayU-Signature header".into()))?;

    let secret = get_webhook_secret(&state, "payu").await?;

    webhook_verify::verify_payu(&body, signature, &secret)
        .map_err(|e| AppError::Forbidden(format!("Webhook verification failed: {}", e)))?;

    let raw: serde_json::Value = serde_json::from_slice(&body)
        .map_err(|e| AppError::Validation(format!("Invalid webhook payload: {}", e)))?;

    let status = raw.get("status").and_then(|v| v.as_str()).unwrap_or("unknown");
    let event_type = match status {
        "success" => "payment.captured",
        "failure" => "payment.failed",
        _ => "payment.unknown",
    };

    let svc = PaymentGatewayService::new(&state.db);
    let webhook_req = WebhookPayload {
        event_type: event_type.to_string(),
        gateway_id: "payu".into(),
        payload: raw,
        signature: Some(signature.to_string()),
    };
    Ok(Json(svc.process_webhook(webhook_req).await?))
}

/// InstaMojo webhook handler with Svix-based signature verification.
// Note: Not annotated with #[utoipa::path] because it accepts raw Bytes
// which doesn't implement ToSchema. Documented manually in openapi.rs.
pub async fn process_webhook_instamojo(
    State(state): State<SharedState>,
    headers: HeaderMap,
    body: Bytes,
) -> Result<Json<WebhookProcessResponse>, AppError> {
    let svix_id = headers
        .get("Svix-Id")
        .and_then(|v| v.to_str().ok())
        .ok_or_else(|| AppError::Validation("Missing Svix-Id header".into()))?;

    let svix_timestamp = headers
        .get("Svix-Timestamp")
        .and_then(|v| v.to_str().ok())
        .ok_or_else(|| AppError::Validation("Missing Svix-Timestamp header".into()))?;

    let svix_signature = headers
        .get("Svix-Signature")
        .and_then(|v| v.to_str().ok())
        .ok_or_else(|| AppError::Validation("Missing Svix-Signature header".into()))?;

    let secret = get_webhook_secret(&state, "instamojo").await?;

    webhook_verify::verify_instamojo(&body, svix_id, svix_timestamp, svix_signature, &secret)
        .map_err(|e| AppError::Forbidden(format!("Webhook verification failed: {}", e)))?;

    let raw: serde_json::Value = serde_json::from_slice(&body)
        .map_err(|e| AppError::Validation(format!("Invalid webhook payload: {}", e)))?;

    let event_type = raw.get("type").and_then(|v| v.as_str()).unwrap_or("unknown");
    let payload_data = raw.get("data").cloned().unwrap_or(raw.clone());

    let svc = PaymentGatewayService::new(&state.db);
    let webhook_req = WebhookPayload {
        event_type: event_type.to_string(),
        gateway_id: "instamojo".into(),
        payload: payload_data,
        signature: Some(svix_signature.to_string()),
    };
    Ok(Json(svc.process_webhook(webhook_req).await?))
}

#[utoipa::path(
    post,
    path = "/api/v1/payments/{id}/retry",
    tag = "Payment Gateway",
    security(("bearer_auth" = [])),
    params(("id" = i64, Path, description = "Payment ID")),
    responses(
        (status = 200, description = "Payment retried", body = PaymentLinkResponse),
        (status = 404, description = "Payment not found")
    )
)]
pub async fn retry_payment(State(state): State<SharedState>, Json(req): Json<RetryPaymentRequest>) -> Result<Json<PaymentLinkResponse>, AppError> {
    let svc = PaymentGatewayService::new(&state.db);
    Ok(Json(svc.retry_payment(req).await?))
}

/// Retrieve the webhook signing secret for a given gateway from the database.
async fn get_webhook_secret(state: &SharedState, gateway_id: &str) -> Result<String, AppError> {
    let result = sqlx::query_scalar::<_, Option<String>>(
        "SELECT webhook_secret FROM payment_gateways WHERE gateway_id = $1 AND is_active = true"
    )
    .bind(gateway_id)
    .fetch_optional(&state.db)
    .await
    .map_err(|e| AppError::External(format!("Database error: {}", e)))?;

    if let Some(Some(secret)) = result {
        return Ok(secret);
    }

    let env_key = format!("{}_WEBHOOK_SECRET", gateway_id.to_uppercase());
    std::env::var(&env_key)
        .map_err(|_| AppError::External(format!(
            "Webhook secret not configured for gateway '{}'. Set {} in environment or database.",
            gateway_id, env_key
        )))
}
