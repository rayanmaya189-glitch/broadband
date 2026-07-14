//! Billing event types.
//!
//! These events are published when billing-related actions occur
//! and are consumed by other modules (notification, accounting).

use chrono::{DateTime, NaiveDate, Utc};
use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};

/// Published when an invoice is created.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct InvoiceCreatedEvent {
    pub invoice_id: i64,
    pub invoice_number: String,
    pub customer_id: i64,
    pub subscription_id: i64,
    pub branch_id: i64,
    pub total_amount: Decimal,
    pub due_date: NaiveDate,
    pub timestamp: DateTime<Utc>,
}

/// Published when an invoice is paid.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct InvoicePaidEvent {
    pub invoice_id: i64,
    pub invoice_number: String,
    pub customer_id: i64,
    pub amount_paid: Decimal,
    pub payment_method: Option<String>,
    pub timestamp: DateTime<Utc>,
}

/// Published when a payment is completed.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PaymentCompletedEvent {
    pub payment_id: i64,
    pub invoice_id: i64,
    pub customer_id: i64,
    pub amount: Decimal,
    pub gateway: Option<String>,
    pub gateway_transaction_id: Option<String>,
    pub timestamp: DateTime<Utc>,
}

/// Published when a payment fails.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PaymentFailedEvent {
    pub invoice_id: i64,
    pub customer_id: i64,
    pub amount: Decimal,
    pub failure_reason: Option<String>,
    pub timestamp: DateTime<Utc>,
}

/// Published when a refund is processed.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RefundProcessedEvent {
    pub refund_id: i64,
    pub payment_id: i64,
    pub invoice_id: i64,
    pub customer_id: i64,
    pub amount: Decimal,
    pub reason: Option<String>,
    pub timestamp: DateTime<Utc>,
}
