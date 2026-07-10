use chrono::{DateTime, NaiveDate, Utc};
use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;


#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct InvoiceResponse {
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
    pub review_status: Option<String>,
    pub notes: Option<String>,
    pub created_at: DateTime<Utc>,
    pub customer_name: Option<String>,
    pub branch_name: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct InvoiceLineItemResponse {
    pub id: i64,
    pub invoice_id: i64,
    pub description: String,
    pub quantity: Decimal,
    pub unit_price: Decimal,
    pub amount: Decimal,
    pub tax_rate: Decimal,
    pub tax_amount: Decimal,
}

#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct InvoiceListResponse {
    pub invoices: Vec<InvoiceResponse>,
    pub total: i64,
    pub page: i64,
    pub per_page: i64,
    pub total_pages: i64,
}

#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct PaymentResponse {
    pub id: i64,
    pub payment_number: String,
    pub invoice_id: i64,
    pub customer_id: i64,
    pub amount: Decimal,
    pub currency: String,
    pub payment_method: String,
    pub payment_gateway: Option<String>,
    pub status: String,
    pub processed_at: Option<DateTime<Utc>>,
    pub created_at: DateTime<Utc>,
}

#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct PaymentListResponse {
    pub payments: Vec<PaymentResponse>,
    pub total: i64,
    pub page: i64,
    pub per_page: i64,
    pub total_pages: i64,
}

#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct RefundResponse {
    pub id: i64,
    pub refund_number: String,
    pub payment_id: i64,
    pub invoice_id: i64,
    pub customer_id: i64,
    pub amount: Decimal,
    pub reason: String,
    pub status: String,
    pub created_at: DateTime<Utc>,
}

#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct DiscountResponse {
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

impl InvoiceResponse {
    pub fn from_model(m: crate::modules::billing::model::invoice_entity::Model) -> Self {
        Self {
            id: m.id, invoice_number: m.invoice_number, customer_id: m.customer_id,
            branch_id: m.branch_id, subscription_id: m.subscription_id,
            billing_period_start: m.billing_period_start, billing_period_end: m.billing_period_end,
            subtotal: m.subtotal, discount_amount: m.discount_amount, tax_amount: m.tax_amount,
            total_amount: m.total_amount, currency: m.currency, status: m.status,
            due_date: m.due_date, paid_at: m.paid_at.map(|v| v.into()),
            payment_method: m.payment_method, review_status: m.review_status, notes: m.notes,
            created_at: m.created_at.into(), customer_name: None, branch_name: None,
        }
    }
}

impl InvoiceLineItemResponse {
    pub fn from_model(m: crate::modules::billing::model::invoice_line_item_entity::Model) -> Self {
        Self {
            id: m.id, invoice_id: m.invoice_id, description: m.description,
            quantity: m.quantity, unit_price: m.unit_price, amount: m.amount,
            tax_rate: m.tax_rate, tax_amount: m.tax_amount,
        }
    }
}

impl PaymentResponse {
    pub fn from_model(m: crate::modules::billing::model::payment_entity::Model) -> Self {
        Self {
            id: m.id, payment_number: m.payment_number, invoice_id: m.invoice_id,
            customer_id: m.customer_id, amount: m.amount, currency: m.currency,
            payment_method: m.payment_method, payment_gateway: m.payment_gateway,
            status: m.status, processed_at: m.processed_at.map(|v| v.into()),
            created_at: m.created_at.into(),
        }
    }
}

impl RefundResponse {
    pub fn from_model(m: crate::modules::billing::model::refund_entity::Model) -> Self {
        Self {
            id: m.id, refund_number: m.refund_number, payment_id: m.payment_id,
            invoice_id: m.invoice_id, customer_id: m.customer_id, amount: m.amount,
            reason: m.reason, status: m.status, created_at: m.created_at.into(),
        }
    }
}

impl DiscountResponse {
    pub fn from_model(m: crate::modules::billing::model::discount_entity::Model) -> Self {
        Self {
            id: m.id, name: m.name, code: m.code, discount_type: m.discount_type,
            value: m.value, max_uses: m.max_uses, current_uses: m.current_uses,
            valid_from: m.valid_from, valid_until: m.valid_until, is_active: m.is_active,
            created_at: m.created_at.into(),
        }
    }
}

#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct MessageResponse {
    pub message: String,
}
