# AeroXe Backend — Payment Gateway Module

> **Req Ref:** §8B Payment Gateway Integration

---

## 1. Overview

Gateway-agnostic payment processing supporting Razorpay, PayU, InstaMojo, and CCAvenue. Handles payment link generation, webhook processing, idempotency, and gateway failover.

## 2. Supported Gateways

| Gateway | UPI | Cards | Net Banking | Auto-debit |
|---------|-----|-------|-------------|------------|
| Razorpay | ✅ | ✅ | ✅ | ✅ (UPI AutoPay) |
| PayU | ✅ | ✅ | ✅ | ✅ (NACH) |
| InstaMojo | ✅ | ✅ | ❌ | ❌ |
| CCAvenue | ✅ | ✅ | ✅ | ❌ |

## 3. API Endpoints

| Method | Path | Required Role | Description |
|--------|------|--------------|-------------|
| POST | `/api/v1/payments/create-link` | billing_operator+ | Generate payment link |
| GET | `/api/v1/payments/links` | billing_ops | List payment links |
| POST | `/api/v1/payments/webhook/razorpay` | No (webhook) | Razorpay webhook |
| POST | `/api/v1/payments/webhook/payu` | No (webhook) | PayU webhook |
| POST | `/api/v1/payments/webhook/instamojo` | No (webhook) | InstaMojo webhook |
| GET | `/api/v1/payments/gateways` | finance_manager+ | List gateway configs |
| POST | `/api/v1/payments/gateways` | finance_manager+ | Configure gateway |
| PUT | `/api/v1/payments/gateways/:id` | finance_manager+ | Update gateway config |
| POST | `/api/v1/payments/:id/retry` | billing_operator+ | Retry failed payment |

## 4. Payment Link Flow

```
1. Staff creates payment link for invoice
2. Backend selects primary gateway (Razorpay)
3. Call gateway API to create payment link/order
4. Return payment URL to staff
5. Staff sends link to customer (SMS/Email/WhatsApp)
6. Customer clicks link → gateway checkout page
7. Customer completes payment
8. Gateway sends webhook to backend
9. Backend verifies webhook signature
10. Backend updates payment status
11. Backend updates invoice status
12. Publish invoice.paid event
```

## 5. Gateway Configuration

```json
{
  "gateway_id": "razorpay",
  "name": "Razorpay",
  "is_primary": true,
  "is_active": true,
  "credentials": {
    "key_id": "encrypted:...",
    "key_secret": "encrypted:..."
  },
  "webhook_secret": "encrypted:...",
  "fee_structure": {
    "percentage": 2.0,
    "fixed": 0,
    "gst_on_fee": 18.0
  },
  "supported_methods": ["upi", "card", "netbanking", "wallet"],
  "currency": "INR"
}
```

## 6. Webhook Processing

```rust
// Razorpay webhook handler
async fn handle_razorpay_webhook(
    State(state): State<SharedState>,
    headers: HeaderMap,
    body: Bytes,
) -> Result<StatusCode, AppError> {
    // 1. Verify webhook signature
    let signature = headers.get("X-Razorpay-Signature").unwrap();
    verify_signature(&body, signature, &gateway.webhook_secret)?;

    // 2. Parse webhook payload
    let payload: RazorpayWebhook = serde_json::from_slice(&body)?;

    // 3. Idempotency check
    if state.redis.exists(&format!("webhook:processed:{}", payload.event_id)).await? {
        return Ok(StatusCode::OK);
    }

    // 4. Process based on event type
    match payload.event.as_str() {
        "payment.captured" => process_successful_payment(&state, &payload).await?,
        "payment.failed" => process_failed_payment(&state, &payload).await?,
        "refund.created" => process_refund(&state, &payload).await?,
        _ => {}
    }

    // 5. Mark as processed
    state.redis.setex(
        &format!("webhook:processed:{}", payload.event_id),
        "1", 86400
    ).await?;

    Ok(StatusCode::OK)
}
```

## 7. Idempotency

- Each payment request includes an `idempotency_key`
- Stored in Redis with 24h TTL
- Duplicate requests return the original response
- Gateway transaction IDs used for deduplication

```rust
pub struct IdempotencyKey(pub String);

impl FromRequestParts<AppState> for IdempotencyKey {
    async fn from_request_parts(parts: &mut Parts, _state: &AppState) -> Result<Self> {
        parts.headers
            .get("X-Idempotency-Key")
            .and_then(|v| v.to_str().ok())
            .map(|v| IdempotencyKey(v.to_string()))
            .ok_or_else(|| AppError::Validation("Missing idempotency key".into()))
    }
}
```

## 8. Gateway Failover

```
1. Primary gateway fails
2. Retry with primary gateway (1 attempt)
3. If still fails → switch to secondary gateway
4. If secondary fails → queue for manual processing
5. Notify finance team
```

## 9. RBAC Permissions

```
payment.gateway.view
payment.gateway.configure
payment.link.create
payment.webhook.receive
```
