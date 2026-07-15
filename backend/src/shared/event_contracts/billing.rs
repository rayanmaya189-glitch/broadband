use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct InvoiceGeneratedV1 {
    pub invoice_id: i64,
    pub invoice_number: String,
    pub customer_id: i64,
    pub subscription_id: i64,
    pub total_amount: Decimal,
    pub due_date: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct InvoicePaidV1 {
    pub invoice_id: i64,
    pub payment_id: i64,
    pub amount: Decimal,
    pub payment_method: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct InvoiceOverdueV1 {
    pub invoice_id: i64,
    pub days_overdue: i32,
    pub total_amount: Decimal,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PaymentCompletedV1 {
    pub payment_id: i64,
    pub invoice_id: i64,
    pub amount: Decimal,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PaymentFailedV1 {
    pub payment_id: i64,
    pub invoice_id: i64,
    pub reason: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RefundApprovedV1 {
    pub refund_id: i64,
    pub invoice_id: i64,
    pub amount: Decimal,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RefundProcessedV1 {
    pub refund_id: i64,
    pub invoice_id: i64,
    pub amount: Decimal,
}
