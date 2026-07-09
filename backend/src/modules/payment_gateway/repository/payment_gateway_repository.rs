use sqlx::PgPool;
use crate::modules::payment_gateway::model::payment_gateway::*;

pub struct PaymentGatewayRepository<'a> {
    pool: &'a PgPool,
}

impl<'a> PaymentGatewayRepository<'a> {
    pub fn new(pool: &'a PgPool) -> Self { Self { pool } }

    // ── Gateway Config ────────────────────────────────────

    pub async fn list_gateways(&self) -> Result<Vec<GatewayConfig>, sqlx::Error> {
        sqlx::query_as::<_, GatewayConfig>("SELECT id, gateway_id, name, is_primary, is_active, supported_methods, currency, created_at, updated_at FROM payment_gateways ORDER BY is_primary DESC").fetch_all(self.pool).await
    }

    pub async fn create_gateway(&self, gateway_id: &str, name: &str, is_primary: bool) -> Result<GatewayConfig, sqlx::Error> {
        sqlx::query_as::<_, GatewayConfig>("INSERT INTO payment_gateways (gateway_id, name, is_primary) VALUES ($1,$2,$3) RETURNING id, gateway_id, name, is_primary, is_active, supported_methods, currency, created_at, updated_at")
            .bind(gateway_id).bind(name).bind(is_primary).fetch_one(self.pool).await
    }

    pub async fn update_gateway(&self, id: i64, name: Option<&str>, is_primary: Option<bool>, is_active: Option<bool>) -> Result<GatewayConfig, sqlx::Error> {
        sqlx::query_as::<_, GatewayConfig>("UPDATE payment_gateways SET name = COALESCE($2, name), is_primary = COALESCE($3, is_primary), is_active = COALESCE($4, is_active), updated_at = NOW() WHERE id = $1 RETURNING id, gateway_id, name, is_primary, is_active, supported_methods, currency, created_at, updated_at")
            .bind(id).bind(name).bind(is_primary).bind(is_active).fetch_one(self.pool).await
    }

    // ── Payment Transactions ──────────────────────────────

    pub async fn create_transaction(&self, gateway_id: &str, invoice_id: Option<i64>, customer_id: Option<i64>, amount: rust_decimal::Decimal, payment_method: &str, idempotency_key: Option<&str>) -> Result<PaymentTransaction, sqlx::Error> {
        sqlx::query_as::<_, PaymentTransaction>("INSERT INTO payment_transactions (gateway_id, invoice_id, customer_id, amount, currency, payment_method, idempotency_key, status) VALUES ($1,$2,$3,$4,'INR',$5,$6,'pending') RETURNING id, gateway_id, invoice_id, customer_id, amount, currency, payment_method, gateway_transaction_id, status, idempotency_key, failure_reason, webhook_received_at, created_at, updated_at")
            .bind(gateway_id).bind(invoice_id).bind(customer_id).bind(amount).bind(payment_method).bind(idempotency_key).fetch_one(self.pool).await
    }

    pub async fn find_by_idempotency(&self, key: &str) -> Result<Option<PaymentTransaction>, sqlx::Error> {
        sqlx::query_as::<_, PaymentTransaction>("SELECT id, gateway_id, invoice_id, customer_id, amount, currency, payment_method, gateway_transaction_id, status, idempotency_key, failure_reason, webhook_received_at, created_at, updated_at FROM payment_transactions WHERE idempotency_key = $1").bind(key).fetch_optional(self.pool).await
    }

    pub async fn update_transaction_status(&self, id: i64, status: &str, gateway_txn_id: Option<&str>, failure_reason: Option<&str>) -> Result<PaymentTransaction, sqlx::Error> {
        sqlx::query_as::<_, PaymentTransaction>("UPDATE payment_transactions SET status = $2, gateway_transaction_id = COALESCE($3, gateway_transaction_id), failure_reason = $4, updated_at = NOW() WHERE id = $1 RETURNING id, gateway_id, invoice_id, customer_id, amount, currency, payment_method, gateway_transaction_id, status, idempotency_key, failure_reason, webhook_received_at, created_at, updated_at")
            .bind(id).bind(status).bind(gateway_txn_id).bind(failure_reason).fetch_one(self.pool).await
    }

    pub async fn get_transaction(&self, id: i64) -> Result<Option<PaymentTransaction>, sqlx::Error> {
        sqlx::query_as::<_, PaymentTransaction>("SELECT id, gateway_id, invoice_id, customer_id, amount, currency, payment_method, gateway_transaction_id, status, idempotency_key, failure_reason, webhook_received_at, created_at, updated_at FROM payment_transactions WHERE id = $1").bind(id).fetch_optional(self.pool).await
    }

    pub async fn list_transactions(&self, gateway_id: Option<&str>, status: Option<&str>, page: i64, per_page: i64) -> Result<(Vec<PaymentTransaction>, i64), sqlx::Error> {
        let offset = (page - 1) * per_page;
        let count_row: (i64,) = sqlx::query_as("SELECT COUNT(*) FROM payment_transactions WHERE ($1::text IS NULL OR gateway_id = $1) AND ($2::text IS NULL OR status = $2)")
            .bind(gateway_id).bind(status).fetch_one(self.pool).await?;
        let txns: Vec<PaymentTransaction> = sqlx::query_as("SELECT id, gateway_id, invoice_id, customer_id, amount, currency, payment_method, gateway_transaction_id, status, idempotency_key, failure_reason, webhook_received_at, created_at, updated_at FROM payment_transactions WHERE ($1::text IS NULL OR gateway_id = $1) AND ($2::text IS NULL OR status = $2) ORDER BY created_at DESC LIMIT $3 OFFSET $4")
            .bind(gateway_id).bind(status).bind(per_page).bind(offset).fetch_all(self.pool).await?;
        Ok((txns, count_row.0))
    }

    // ── Payment Links ─────────────────────────────────────

    pub async fn create_payment_link(&self, transaction_id: i64, payment_url: &str, expires_at: chrono::DateTime<chrono::Utc>) -> Result<PaymentLink, sqlx::Error> {
        sqlx::query_as::<_, PaymentLink>("INSERT INTO payment_links (transaction_id, payment_url, expires_at) VALUES ($1,$2,$3) RETURNING id, transaction_id, payment_url, short_url, expires_at, is_used, created_at")
            .bind(transaction_id).bind(payment_url).bind(expires_at).fetch_one(self.pool).await
    }

    // ── Webhook Logs ──────────────────────────────────────

    pub async fn log_webhook(&self, gateway_id: &str, event_type: &str, payload: serde_json::Value, processed: bool, error_message: Option<&str>) -> Result<WebhookLog, sqlx::Error> {
        sqlx::query_as::<_, WebhookLog>("INSERT INTO webhook_logs (gateway_id, event_type, payload, processed, error_message) VALUES ($1,$2,$3,$4,$5) RETURNING id, gateway_id, event_type, payload, processed, error_message, created_at")
            .bind(gateway_id).bind(event_type).bind(payload).bind(processed).bind(error_message).fetch_one(self.pool).await
    }
}
