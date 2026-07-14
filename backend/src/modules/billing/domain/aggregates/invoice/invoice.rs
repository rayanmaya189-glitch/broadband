//! Invoice aggregate root.
//!
//! The Invoice aggregate is the consistency boundary for all billing operations.

use chrono::{DateTime, NaiveDate, Utc};
use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};
use thiserror::Error;

use crate::common::shared::events::EventEnvelope;

/// Invoice aggregate root.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Invoice {
    pub id: i64,
    pub invoice_number: String,
    pub customer_id: i64,
    pub subscription_id: i64,
    pub branch_id: i64,
    pub status: InvoiceStatus,
    pub subtotal: Decimal,
    pub tax_amount: Decimal,
    pub total_amount: Decimal,
    pub discount_amount: Decimal,
    pub due_date: NaiveDate,
    pub paid_at: Option<DateTime<Utc>>,
    pub notes: Option<String>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

/// Invoice line item.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct InvoiceLineItem {
    pub id: i64,
    pub invoice_id: i64,
    pub description: String,
    pub quantity: i32,
    pub unit_price: Decimal,
    pub amount: Decimal,
    pub tax_rate: Decimal,
}

/// Invoice status.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum InvoiceStatus {
    Draft,
    Pending,
    Paid,
    Overdue,
    Cancelled,
    Refunded,
}

impl InvoiceStatus {
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::Draft => "draft",
            Self::Pending => "pending",
            Self::Paid => "paid",
            Self::Overdue => "overdue",
            Self::Cancelled => "cancelled",
            Self::Refunded => "refunded",
        }
    }

    pub fn from_str(s: &str) -> Result<Self, InvoiceError> {
        match s.to_lowercase().as_str() {
            "draft" => Ok(Self::Draft),
            "pending" => Ok(Self::Pending),
            "paid" => Ok(Self::Paid),
            "overdue" => Ok(Self::Overdue),
            "cancelled" => Ok(Self::Cancelled),
            "refunded" => Ok(Self::Refunded),
            _ => Err(InvoiceError::InvalidStatus(s.to_string())),
        }
    }
}

/// Invoice domain events.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum InvoiceEvent {
    Created {
        invoice_id: i64,
        customer_id: i64,
        total_amount: Decimal,
    },
    Paid {
        invoice_id: i64,
        customer_id: i64,
        amount_paid: Decimal,
    },
    Overdue {
        invoice_id: i64,
        customer_id: i64,
    },
    Cancelled {
        invoice_id: i64,
        reason: Option<String>,
    },
}

/// Invoice domain errors.
#[derive(Debug, Error)]
pub enum InvoiceError {
    #[error("Invalid status: {0}")]
    InvalidStatus(String),

    #[error("Invalid status transition: {0}")]
    InvalidTransition(String),

    #[error("Validation error: {0}")]
    Validation(String),

    #[error("Invoice not found")]
    NotFound,
}

impl Invoice {
    /// Create a new invoice.
    pub fn create(
        id: i64,
        invoice_number: String,
        customer_id: i64,
        subscription_id: i64,
        branch_id: i64,
        subtotal: Decimal,
        tax_rate: Decimal,
        due_date: NaiveDate,
    ) -> Result<Self, InvoiceError> {
        let tax_amount = subtotal * tax_rate;
        let total_amount = subtotal + tax_amount;
        let now = Utc::now();

        Ok(Self {
            id,
            invoice_number,
            customer_id,
            subscription_id,
            branch_id,
            status: InvoiceStatus::Draft,
            subtotal,
            tax_amount,
            total_amount,
            discount_amount: Decimal::ZERO,
            due_date,
            paid_at: None,
            notes: None,
            created_at: now,
            updated_at: now,
        })
    }

    /// Mark invoice as pending (ready for payment).
    pub fn finalize(&mut self) -> Result<(), InvoiceError> {
        if self.status != InvoiceStatus::Draft {
            return Err(InvoiceError::InvalidTransition(format!(
                "Cannot finalize from {}",
                self.status.as_str()
            )));
        }
        self.status = InvoiceStatus::Pending;
        self.updated_at = Utc::now();
        Ok(())
    }

    /// Mark invoice as paid.
    pub fn mark_paid(&mut self, amount: Decimal) -> Result<InvoiceEvent, InvoiceError> {
        if self.status != InvoiceStatus::Pending && self.status != InvoiceStatus::Overdue {
            return Err(InvoiceError::InvalidTransition(format!(
                "Cannot mark as paid from {}",
                self.status.as_str()
            )));
        }
        self.status = InvoiceStatus::Paid;
        self.paid_at = Some(Utc::now());
        self.updated_at = Utc::now();

        Ok(InvoiceEvent::Paid {
            invoice_id: self.id,
            customer_id: self.customer_id,
            amount_paid: amount,
        })
    }

    /// Mark invoice as overdue.
    pub fn mark_overdue(&mut self) -> Result<InvoiceEvent, InvoiceError> {
        if self.status != InvoiceStatus::Pending {
            return Err(InvoiceError::InvalidTransition(format!(
                "Cannot mark as overdue from {}",
                self.status.as_str()
            )));
        }
        self.status = InvoiceStatus::Overdue;
        self.updated_at = Utc::now();

        Ok(InvoiceEvent::Overdue {
            invoice_id: self.id,
            customer_id: self.customer_id,
        })
    }

    /// Cancel invoice.
    pub fn cancel(&mut self, reason: Option<&str>) -> Result<InvoiceEvent, InvoiceError> {
        if self.status == InvoiceStatus::Paid || self.status == InvoiceStatus::Refunded {
            return Err(InvoiceError::InvalidTransition(format!(
                "Cannot cancel from {}",
                self.status.as_str()
            )));
        }
        self.status = InvoiceStatus::Cancelled;
        self.updated_at = Utc::now();

        Ok(InvoiceEvent::Cancelled {
            invoice_id: self.id,
            reason: reason.map(|s| s.to_string()),
        })
    }

    /// Apply discount to invoice.
    pub fn apply_discount(&mut self, discount: Decimal) -> Result<(), InvoiceError> {
        if discount < Decimal::ZERO || discount > self.subtotal {
            return Err(InvoiceError::Validation(
                "Discount must be between 0 and subtotal".to_string(),
            ));
        }
        self.discount_amount = discount;
        self.total_amount = self.subtotal + self.tax_amount - discount;
        self.updated_at = Utc::now();
        Ok(())
    }

    /// Check if invoice is overdue based on due date.
    pub fn is_overdue(&self) -> bool {
        self.status == InvoiceStatus::Pending && Utc::now().date_naive() > self.due_date
    }

    /// Create creation event.
    pub fn creation_event(&self) -> EventEnvelope<InvoiceEvent> {
        EventEnvelope::new(
            "billing.invoice.created.v1".to_string(),
            1,
            "billing-service".to_string(),
            InvoiceEvent::Created {
                invoice_id: self.id,
                customer_id: self.customer_id,
                total_amount: self.total_amount,
            },
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn create_test_invoice() -> Invoice {
        Invoice::create(
            1,
            "INV-202607-0001".to_string(),
            100,
            200,
            1,
            Decimal::new(1000, 2),
            Decimal::new(18, 2),
            NaiveDate::from_ymd_opt(2026, 8, 1).unwrap(),
        )
        .unwrap()
    }

    #[test]
    fn test_create_invoice() {
        let invoice = create_test_invoice();
        assert_eq!(invoice.id, 1);
        assert_eq!(invoice.status, InvoiceStatus::Draft);
        assert_eq!(invoice.subtotal, Decimal::new(1000, 2));
    }

    #[test]
    fn test_finalize_invoice() {
        let mut invoice = create_test_invoice();
        invoice.finalize().unwrap();
        assert_eq!(invoice.status, InvoiceStatus::Pending);
    }

    #[test]
    fn test_mark_paid() {
        let mut invoice = create_test_invoice();
        invoice.finalize().unwrap();
        let event = invoice.mark_paid(invoice.total_amount).unwrap();
        assert_eq!(invoice.status, InvoiceStatus::Paid);
        assert!(invoice.paid_at.is_some());
    }

    #[test]
    fn test_mark_overdue() {
        let mut invoice = create_test_invoice();
        invoice.finalize().unwrap();
        let event = invoice.mark_overdue().unwrap();
        assert_eq!(invoice.status, InvoiceStatus::Overdue);
    }

    #[test]
    fn test_cancel_invoice() {
        let mut invoice = create_test_invoice();
        let event = invoice.cancel(Some("Customer request")).unwrap();
        assert_eq!(invoice.status, InvoiceStatus::Cancelled);
    }

    #[test]
    fn test_apply_discount() {
        let mut invoice = create_test_invoice();
        invoice.apply_discount(Decimal::new(500, 2)).unwrap();
        assert_eq!(invoice.discount_amount, Decimal::new(500, 2));
    }
}
