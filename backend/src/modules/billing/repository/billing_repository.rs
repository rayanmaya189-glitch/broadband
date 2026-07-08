use sqlx::PgPool;

use crate::modules::billing::model::billing::{Invoice, Payment, Refund, Discount};

pub struct BillingRepository<'a> {
    pool: &'a PgPool,
}

impl<'a> BillingRepository<'a> {
    pub fn new(pool: &'a PgPool) -> Self { Self { pool } }
    pub fn pool(&self) -> &'a PgPool { self.pool }

    // ──── Invoices ────
    pub async fn list_invoices(&self, branch_id: Option<i64>, status: Option<&str>, customer_id: Option<i64>, page: i64, per_page: i64) -> Result<(Vec<Invoice>, i64), sqlx::Error> {
        let offset = (page - 1) * per_page;
        let count_row: (i64,) = sqlx::query_as("SELECT COUNT(*) FROM invoices WHERE ($1::bigint IS NULL OR branch_id = $1) AND ($2::text IS NULL OR status = $2) AND ($3::bigint IS NULL OR customer_id = $3)")
            .bind(branch_id).bind(status).bind(customer_id).fetch_one(self.pool).await?;
        let invoices: Vec<Invoice> = sqlx::query_as("SELECT * FROM invoices WHERE ($1::bigint IS NULL OR branch_id = $1) AND ($2::text IS NULL OR status = $2) AND ($3::bigint IS NULL OR customer_id = $3) ORDER BY created_at DESC LIMIT $4 OFFSET $5")
            .bind(branch_id).bind(status).bind(customer_id).bind(per_page).bind(offset).fetch_all(self.pool).await?;
        Ok((invoices, count_row.0))
    }

    pub async fn get_invoice_by_id(&self, id: i64) -> Result<Option<Invoice>, sqlx::Error> {
        sqlx::query_as::<_, Invoice>("SELECT * FROM invoices WHERE id = $1").bind(id).fetch_optional(self.pool).await
    }

    pub async fn create_invoice(&self, invoice_number: &str, customer_id: i64, branch_id: i64, subscription_id: i64, period_start: chrono::NaiveDate, period_end: chrono::NaiveDate, subtotal: rust_decimal::Decimal, discount: rust_decimal::Decimal, tax: rust_decimal::Decimal, total: rust_decimal::Decimal, due_date: chrono::NaiveDate, notes: Option<&str>) -> Result<Invoice, sqlx::Error> {
        sqlx::query_as::<_, Invoice>("INSERT INTO invoices (invoice_number, customer_id, branch_id, subscription_id, billing_period_start, billing_period_end, subtotal, discount_amount, tax_amount, total_amount, due_date, notes) VALUES ($1,$2,$3,$4,$5,$6,$7,$8,$9,$10,$11,$12) RETURNING *")
            .bind(invoice_number).bind(customer_id).bind(branch_id).bind(subscription_id).bind(period_start).bind(period_end).bind(subtotal).bind(discount).bind(tax).bind(total).bind(due_date).bind(notes).fetch_one(self.pool).await
    }

    pub async fn update_invoice_status(&self, id: i64, status: &str) -> Result<Invoice, sqlx::Error> {
        sqlx::query_as::<_, Invoice>("UPDATE invoices SET status = $2, updated_at = NOW(), paid_at = CASE WHEN $2 = 'paid' THEN NOW() ELSE paid_at END WHERE id = $1 RETURNING *")
            .bind(id).bind(status).fetch_one(self.pool).await
    }

    pub async fn generate_invoice_number(&self) -> Result<String, sqlx::Error> {
        let now = chrono::Utc::now();
        let prefix = format!("INV-{}-{:02}", now.format("%Y"), now.format("%m"));
        let row: (i64,) = sqlx::query_as("SELECT COUNT(*) + 1 FROM invoices WHERE invoice_number LIKE $1")
            .bind(format!("{}%", prefix)).fetch_one(self.pool).await?;
        Ok(format!("{}-{:04}", prefix, row.0))
    }

    // ──── Payments ────
    pub async fn list_payments(&self, customer_id: Option<i64>, status: Option<&str>, page: i64, per_page: i64) -> Result<(Vec<Payment>, i64), sqlx::Error> {
        let offset = (page - 1) * per_page;
        let count_row: (i64,) = sqlx::query_as("SELECT COUNT(*) FROM payments WHERE ($1::bigint IS NULL OR customer_id = $1) AND ($2::text IS NULL OR status = $2)")
            .bind(customer_id).bind(status).fetch_one(self.pool).await?;
        let payments: Vec<Payment> = sqlx::query_as("SELECT * FROM payments WHERE ($1::bigint IS NULL OR customer_id = $1) AND ($2::text IS NULL OR status = $2) ORDER BY created_at DESC LIMIT $3 OFFSET $4")
            .bind(customer_id).bind(status).bind(per_page).bind(offset).fetch_all(self.pool).await?;
        Ok((payments, count_row.0))
    }

    pub async fn create_payment(&self, payment_number: &str, invoice_id: i64, customer_id: i64, branch_id: i64, amount: rust_decimal::Decimal, method: &str, gateway: Option<&str>, gateway_tx_id: Option<&str>) -> Result<Payment, sqlx::Error> {
        sqlx::query_as::<_, Payment>("INSERT INTO payments (payment_number, invoice_id, customer_id, branch_id, amount, payment_method, payment_gateway, gateway_transaction_id, status) VALUES ($1,$2,$3,$4,$5,$6,$7,$8,'completed') RETURNING *")
            .bind(payment_number).bind(invoice_id).bind(customer_id).bind(branch_id).bind(amount).bind(method).bind(gateway).bind(gateway_tx_id).fetch_one(self.pool).await
    }

    pub async fn generate_payment_number(&self) -> Result<String, sqlx::Error> {
        let now = chrono::Utc::now();
        let prefix = format!("PAY-{}-{:02}", now.format("%Y"), now.format("%m"));
        let row: (i64,) = sqlx::query_as("SELECT COUNT(*) + 1 FROM payments WHERE payment_number LIKE $1")
            .bind(format!("{}%", prefix)).fetch_one(self.pool).await?;
        Ok(format!("{}-{:04}", prefix, row.0))
    }

    // ──── Refunds ────
    pub async fn list_refunds(&self, status: Option<&str>, page: i64, per_page: i64) -> Result<(Vec<Refund>, i64), sqlx::Error> {
        let offset = (page - 1) * per_page;
        let count_row: (i64,) = sqlx::query_as("SELECT COUNT(*) FROM refunds WHERE ($1::text IS NULL OR status = $1)").bind(status).fetch_one(self.pool).await?;
        let refunds: Vec<Refund> = sqlx::query_as("SELECT * FROM refunds WHERE ($1::text IS NULL OR status = $1) ORDER BY created_at DESC LIMIT $2 OFFSET $3")
            .bind(status).bind(per_page).bind(offset).fetch_all(self.pool).await?;
        Ok((refunds, count_row.0))
    }

    pub async fn create_refund(&self, refund_number: &str, payment_id: i64, invoice_id: i64, customer_id: i64, amount: rust_decimal::Decimal, reason: &str, requested_by: Option<i64>) -> Result<Refund, sqlx::Error> {
        sqlx::query_as::<_, Refund>("INSERT INTO refunds (refund_number, payment_id, invoice_id, customer_id, amount, reason, requested_by) VALUES ($1,$2,$3,$4,$5,$6,$7) RETURNING *")
            .bind(refund_number).bind(payment_id).bind(invoice_id).bind(customer_id).bind(amount).bind(reason).bind(requested_by).fetch_one(self.pool).await
    }

    pub async fn approve_refund(&self, id: i64, approved_by: i64) -> Result<Refund, sqlx::Error> {
        sqlx::query_as::<_, Refund>("UPDATE refunds SET status = 'approved', approved_by = $2, approved_at = NOW() WHERE id = $1 RETURNING *")
            .bind(id).bind(approved_by).fetch_one(self.pool).await
    }

    // ──── Discounts ────
    pub async fn list_discounts(&self, page: i64, per_page: i64) -> Result<(Vec<Discount>, i64), sqlx::Error> {
        let offset = (page - 1) * per_page;
        let count_row: (i64,) = sqlx::query_as("SELECT COUNT(*) FROM discounts").fetch_one(self.pool).await?;
        let discounts: Vec<Discount> = sqlx::query_as("SELECT * FROM discounts ORDER BY created_at DESC LIMIT $1 OFFSET $2")
            .bind(per_page).bind(offset).fetch_all(self.pool).await?;
        Ok((discounts, count_row.0))
    }

    pub async fn create_discount(&self, name: &str, code: Option<&str>, discount_type: &str, value: rust_decimal::Decimal, max_uses: Option<i32>, valid_from: chrono::NaiveDate, valid_until: chrono::NaiveDate) -> Result<Discount, sqlx::Error> {
        sqlx::query_as::<_, Discount>("INSERT INTO discounts (name, code, type, value, max_uses, valid_from, valid_until) VALUES ($1,$2,$3,$4,$5,$6,$7) RETURNING *")
            .bind(name).bind(code).bind(discount_type).bind(value).bind(max_uses).bind(valid_from).bind(valid_until).fetch_one(self.pool).await
    }
}
