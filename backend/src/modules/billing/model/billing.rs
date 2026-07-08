use chrono::{DateTime, NaiveDate, Utc};
use rust_decimal::Decimal;
use sqlx::FromRow;

#[derive(Debug, Clone, FromRow)]
pub struct Invoice {
    pub id: i64,
    pub invoice_number: String,
    pub customer_id: i64,
    pub branch_id: i64,
    pub subscription_id: i64,
    pub billing_period_start: NaiveDate,
    pub billing_period_end: NaiveDate,
    pub subtotal: Decimal,
    pub discount_amount: Decimal,
    pub tax_amount: Decimal,
    pub total_amount: Decimal,
    pub currency: String,
    pub status: String,
    pub due_date: NaiveDate,
    pub paid_at: Option<DateTime<Utc>>,
    pub payment_method: Option<String>,
    pub payment_reference: Option<String>,
    pub created_by: Option<i64>,
    pub notes: Option<String>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Clone, FromRow)]
pub struct InvoiceLineItem {
    pub id: i64,
    pub invoice_id: i64,
    pub description: String,
    pub quantity: Decimal,
    pub unit_price: Decimal,
    pub amount: Decimal,
    pub tax_rate: Decimal,
    pub tax_amount: Decimal,
    pub created_at: DateTime<Utc>,
}

#[derive(Debug, Clone, FromRow)]
pub struct Payment {
    pub id: i64,
    pub payment_number: String,
    pub invoice_id: i64,
    pub customer_id: i64,
    pub branch_id: i64,
    pub amount: Decimal,
    pub currency: String,
    pub payment_method: String,
    pub payment_gateway: Option<String>,
    pub gateway_transaction_id: Option<String>,
    pub status: String,
    pub processed_at: Option<DateTime<Utc>>,
    pub created_at: DateTime<Utc>,
}

#[derive(Debug, Clone, FromRow)]
pub struct Refund {
    pub id: i64,
    pub refund_number: String,
    pub payment_id: i64,
    pub invoice_id: i64,
    pub customer_id: i64,
    pub amount: Decimal,
    pub reason: String,
    pub requested_by: Option<i64>,
    pub approved_by: Option<i64>,
    pub status: String,
    pub processed_at: Option<DateTime<Utc>>,
    pub created_at: DateTime<Utc>,
}

#[derive(Debug, Clone, FromRow)]
pub struct Discount {
    pub id: i64,
    pub name: String,
    pub code: Option<String>,
    pub discount_type: String,
    pub value: Decimal,
    pub max_uses: Option<i32>,
    pub current_uses: i32,
    pub valid_from: NaiveDate,
    pub valid_until: NaiveDate,
    pub is_active: bool,
    pub created_at: DateTime<Utc>,
}
