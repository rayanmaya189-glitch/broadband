use chrono::NaiveDate;
use rust_decimal::Decimal;
use serde::Deserialize;
use validator::Validate;

#[derive(Debug, Deserialize, Validate)]
pub struct CreateInvoiceRequest {
    pub customer_id: i64,
    pub branch_id: i64,
    pub subscription_id: i64,
    pub billing_period_start: NaiveDate,
    pub billing_period_end: NaiveDate,
    pub due_date: NaiveDate,
    pub line_items: Vec<CreateLineItemRequest>,
    pub discount_code: Option<String>,
    pub notes: Option<String>,
}

#[derive(Debug, Deserialize, Validate)]
pub struct CreateLineItemRequest {
    pub description: String,
    pub quantity: Option<Decimal>,
    pub unit_price: Decimal,
    pub tax_rate: Option<Decimal>,
}

#[derive(Debug, Deserialize)]
pub struct InvoiceQuery {
    pub status: Option<String>,
    pub customer_id: Option<i64>,
    pub branch_id: Option<i64>,
    pub page: Option<i64>,
    pub per_page: Option<i64>,
}

#[derive(Debug, Deserialize, Validate)]
pub struct RecordPaymentRequest {
    pub invoice_id: i64,
    pub amount: Decimal,
    pub payment_method: String,
    pub payment_gateway: Option<String>,
    pub gateway_transaction_id: Option<String>,
}

#[derive(Debug, Deserialize, Validate)]
pub struct CreateRefundRequest {
    pub payment_id: i64,
    pub amount: Decimal,
    #[validate(length(min = 1))]
    pub reason: String,
}

#[derive(Debug, Deserialize, Validate)]
pub struct CreateDiscountRequest {
    pub name: String,
    pub code: Option<String>,
    #[validate(length(min = 1))]
    pub discount_type: String,
    pub value: Decimal,
    pub max_uses: Option<i32>,
    pub valid_from: NaiveDate,
    pub valid_until: NaiveDate,
}

#[derive(Debug, Deserialize)]
pub struct PaymentQuery {
    pub status: Option<String>,
    pub customer_id: Option<i64>,
    pub branch_id: Option<i64>,
    pub page: Option<i64>,
    pub per_page: Option<i64>,
}

#[derive(Debug, Deserialize)]
pub struct RefundQuery {
    pub status: Option<String>,
    pub page: Option<i64>,
    pub per_page: Option<i64>,
}
