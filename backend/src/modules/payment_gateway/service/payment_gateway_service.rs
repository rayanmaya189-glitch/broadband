use sqlx::PgPool;
use crate::common::errors::app_error::AppError;
use crate::modules::payment_gateway::repository::payment_gateway_repository::PaymentGatewayRepository;
use crate::modules::payment_gateway::request::payment_gateway_request::*;
use crate::modules::payment_gateway::response::payment_gateway_response::*;

pub struct PaymentGatewayService<'a> {
    repo: PaymentGatewayRepository<'a>,
}

impl<'a> PaymentGatewayService<'a> {
    pub fn new(pool: &'a PgPool) -> Self {
        Self { repo: PaymentGatewayRepository::new(pool) }
    }

    // ── Gateway Config ────────────────────────────────────

    pub async fn list_gateways(&self) -> Result<Vec<GatewayConfigResponse>, AppError> {
        let g = self.repo.list_gateways().await?;
        Ok(g.into_iter().map(|x| GatewayConfigResponse { id: x.id, gateway_id: x.gateway_id, name: x.name, is_primary: x.is_primary, is_active: x.is_active, created_at: x.created_at }).collect())
    }

    pub async fn create_gateway(&self, req: CreateGatewayConfigRequest) -> Result<GatewayConfigResponse, AppError> {
        let g = self.repo.create_gateway(&req.gateway_id, &req.name, req.is_primary.unwrap_or(false)).await?;
        Ok(GatewayConfigResponse { id: g.id, gateway_id: g.gateway_id, name: g.name, is_primary: g.is_primary, is_active: g.is_active, created_at: g.created_at })
    }

    pub async fn update_gateway(&self, id: i64, req: UpdateGatewayRequest) -> Result<GatewayConfigResponse, AppError> {
        let g = self.repo.update_gateway(id, req.name.as_deref(), req.is_primary, req.is_active).await.map_err(|_| AppError::NotFound("Gateway not found".into()))?;
        Ok(GatewayConfigResponse { id: g.id, gateway_id: g.gateway_id, name: g.name, is_primary: g.is_primary, is_active: g.is_active, created_at: g.created_at })
    }

    // ── Payment Links ─────────────────────────────────────

    pub async fn create_payment_link(&self, req: CreatePaymentLinkRequest) -> Result<PaymentLinkResponse, AppError> {
        let gateway_id = req.gateway_id.unwrap_or_else(|| "razorpay".into());
        // Check idempotency - if a transaction with this key already exists, return it
        // For now, generate a unique key
        let idempotency_key = format!("{}-{}-{}", req.invoice_id, req.amount, uuid::Uuid::new_v4());
        let txn = self.repo.create_transaction(&gateway_id, Some(req.invoice_id), req.customer_id, req.amount, &req.payment_method, Some(&idempotency_key)).await?;
        let url = format!("https://pay.aeroxe.in/txn/{}", txn.id);
        let expires_at = chrono::Utc::now() + chrono::Duration::hours(24);
        let _link = self.repo.create_payment_link(txn.id, &url, expires_at).await;
        Ok(PaymentLinkResponse { payment_url: url, transaction_id: txn.id, expires_in: 86400 })
    }

    pub async fn list_transactions(&self, q: TransactionQuery) -> Result<TransactionListResponse, AppError> {
        let page = q.page.unwrap_or(1);
        let per_page = q.per_page.unwrap_or(20);
        let (txns, total) = self.repo.list_transactions(q.gateway_id.as_deref(), q.status.as_deref(), page, per_page).await?;
        Ok(TransactionListResponse {
            transactions: txns.into_iter().map(|t| PaymentTransactionResponse { id: t.id, gateway_id: t.gateway_id, invoice_id: t.invoice_id, customer_id: t.customer_id, amount: t.amount, currency: t.currency, payment_method: t.payment_method, gateway_transaction_id: t.gateway_transaction_id, status: t.status, failure_reason: t.failure_reason, created_at: t.created_at }).collect(),
            total, page, per_page,
        })
    }

    // ── Webhook Processing ────────────────────────────────

    pub async fn process_webhook(&self, req: WebhookPayload) -> Result<WebhookProcessResponse, AppError> {
        // Log the webhook
        self.repo.log_webhook(&req.gateway_id, &req.event_type, req.payload.clone(), true, None).await.ok();

        // Process based on event type
        match req.event_type.as_str() {
            "payment.captured" | "payment.success" => {
                if let Some(txn_id) = req.payload.get("transaction_id").and_then(|v| v.as_i64()) {
                    let _ = self.repo.update_transaction_status(txn_id, "completed", None, None).await;
                    return Ok(WebhookProcessResponse { status: "processed".into(), message: "Payment captured".into(), transaction_id: Some(txn_id) });
                }
                Ok(WebhookProcessResponse { status: "ignored".into(), message: "No transaction_id in payload".into(), transaction_id: None })
            }
            "payment.failed" => {
                if let Some(txn_id) = req.payload.get("transaction_id").and_then(|v| v.as_i64()) {
                    let reason = req.payload.get("failure_reason").and_then(|v| v.as_str()).unwrap_or("Unknown");
                    let _ = self.repo.update_transaction_status(txn_id, "failed", None, Some(reason)).await;
                    return Ok(WebhookProcessResponse { status: "processed".into(), message: "Payment failed recorded".into(), transaction_id: Some(txn_id) });
                }
                Ok(WebhookProcessResponse { status: "ignored".into(), message: "No transaction_id in payload".into(), transaction_id: None })
            }
            _ => Ok(WebhookProcessResponse { status: "ignored".into(), message: format!("Unhandled event type: {}", req.event_type), transaction_id: None }),
        }
    }

    // ── Retry ─────────────────────────────────────────────

    pub async fn retry_payment(&self, req: RetryPaymentRequest) -> Result<PaymentLinkResponse, AppError> {
        let txn = self.repo.get_transaction(req.transaction_id).await?.ok_or_else(|| AppError::NotFound("Transaction not found".into()))?;
        if txn.status == "completed" { return Err(AppError::BadRequest("Payment already completed".into())); }
        let gateway_id = req.gateway_id.unwrap_or_else(|| txn.gateway_id.clone());
        let new_txn = self.repo.create_transaction(&gateway_id, txn.invoice_id, txn.customer_id, txn.amount, &txn.payment_method, None).await?;
        let url = format!("https://pay.aeroxe.in/txn/{}", new_txn.id);
        let expires_at = chrono::Utc::now() + chrono::Duration::hours(24);
        let _link = self.repo.create_payment_link(new_txn.id, &url, expires_at).await?;
        Ok(PaymentLinkResponse { payment_url: url, transaction_id: new_txn.id, expires_in: 86400 })
    }
}
