//! Payment aggregate root.
//!
//! The Payment aggregate tracks payment transactions and their status.

use chrono::{DateTime, Utc};
use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};
use thiserror::Error;

use crate::common::shared::events::EventEnvelope;

/// Payment aggregate root.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Payment {
    pub id: i64,
    pub invoice_id: i64,
    pub customer_id: i64,
    pub amount: Decimal,
    pub currency: String,
    pub payment_method: PaymentMethod,
    pub status: PaymentStatus,
    pub gateway_transaction_id: Option<String>,
    pub gateway_response: Option<String>,
    pub refunded_amount: Decimal,
    pub notes: Option<String>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

/// Payment methods.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum PaymentMethod {
    Cash,
    BankTransfer,
    Upi,
    CreditCard,
    DebitCard,
    NetBanking,
    Wallet,
    Cheque,
}

impl PaymentMethod {
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::Cash => "cash",
            Self::BankTransfer => "bank_transfer",
            Self::Upi => "upi",
            Self::CreditCard => "credit_card",
            Self::DebitCard => "debit_card",
            Self::NetBanking => "net_banking",
            Self::Wallet => "wallet",
            Self::Cheque => "cheque",
        }
    }

    pub fn from_str(s: &str) -> Result<Self, PaymentError> {
        match s.to_lowercase().as_str() {
            "cash" => Ok(Self::Cash),
            "bank_transfer" => Ok(Self::BankTransfer),
            "upi" => Ok(Self::Upi),
            "credit_card" => Ok(Self::CreditCard),
            "debit_card" => Ok(Self::DebitCard),
            "net_banking" => Ok(Self::NetBanking),
            "wallet" => Ok(Self::Wallet),
            "cheque" => Ok(Self::Cheque),
            _ => Err(PaymentError::InvalidPaymentMethod(s.to_string())),
        }
    }
}

/// Payment status.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum PaymentStatus {
    Pending,
    Processing,
    Completed,
    Failed,
    Refunded,
    PartiallyRefunded,
}

impl PaymentStatus {
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::Pending => "pending",
            Self::Processing => "processing",
            Self::Completed => "completed",
            Self::Failed => "failed",
            Self::Refunded => "refunded",
            Self::PartiallyRefunded => "partially_refunded",
        }
    }
}

/// Payment domain events.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum PaymentEvent {
    Completed {
        payment_id: i64,
        invoice_id: i64,
        customer_id: i64,
        amount: Decimal,
    },
    Failed {
        payment_id: i64,
        invoice_id: i64,
        reason: Option<String>,
    },
    Refunded {
        payment_id: i64,
        invoice_id: i64,
        amount: Decimal,
    },
}

/// Payment domain errors.
#[derive(Debug, Error)]
pub enum PaymentError {
    #[error("Invalid payment method: {0}")]
    InvalidPaymentMethod(String),

    #[error("Invalid status transition: {0}")]
    InvalidTransition(String),

    #[error("Validation error: {0}")]
    Validation(String),

    #[error("Insufficient refund amount")]
    InsufficientRefundAmount,

    #[error("Payment not found")]
    NotFound,
}

impl Payment {
    /// Create a new payment.
    pub fn create(
        id: i64,
        invoice_id: i64,
        customer_id: i64,
        amount: Decimal,
        currency: String,
        payment_method: PaymentMethod,
    ) -> Result<Self, PaymentError> {
        if amount <= Decimal::ZERO {
            return Err(PaymentError::Validation(
                "Payment amount must be positive".to_string(),
            ));
        }

        let now = Utc::now();

        Ok(Self {
            id,
            invoice_id,
            customer_id,
            amount,
            currency,
            payment_method,
            status: PaymentStatus::Pending,
            gateway_transaction_id: None,
            gateway_response: None,
            refunded_amount: Decimal::ZERO,
            notes: None,
            created_at: now,
            updated_at: now,
        })
    }

    /// Mark payment as processing.
    pub fn start_processing(&mut self) -> Result<(), PaymentError> {
        if self.status != PaymentStatus::Pending {
            return Err(PaymentError::InvalidTransition(format!(
                "Cannot start processing from {}",
                self.status.as_str()
            )));
        }
        self.status = PaymentStatus::Processing;
        self.updated_at = Utc::now();
        Ok(())
    }

    /// Complete payment.
    pub fn complete(
        &mut self,
        gateway_transaction_id: Option<String>,
        gateway_response: Option<String>,
    ) -> Result<PaymentEvent, PaymentError> {
        if self.status != PaymentStatus::Processing {
            return Err(PaymentError::InvalidTransition(format!(
                "Cannot complete from {}",
                self.status.as_str()
            )));
        }
        self.status = PaymentStatus::Completed;
        self.gateway_transaction_id = gateway_transaction_id;
        self.gateway_response = gateway_response;
        self.updated_at = Utc::now();

        Ok(PaymentEvent::Completed {
            payment_id: self.id,
            invoice_id: self.invoice_id,
            customer_id: self.customer_id,
            amount: self.amount,
        })
    }

    /// Fail payment.
    pub fn fail(&mut self, reason: Option<String>) -> Result<PaymentEvent, PaymentError> {
        if self.status != PaymentStatus::Processing {
            return Err(PaymentError::InvalidTransition(format!(
                "Cannot fail from {}",
                self.status.as_str()
            )));
        }
        self.status = PaymentStatus::Failed;
        self.updated_at = Utc::now();

        Ok(PaymentEvent::Failed {
            payment_id: self.id,
            invoice_id: self.invoice_id,
            reason,
        })
    }

    /// Refund payment.
    pub fn refund(&mut self, amount: Decimal) -> Result<PaymentEvent, PaymentError> {
        if self.status != PaymentStatus::Completed {
            return Err(PaymentError::InvalidTransition(format!(
                "Cannot refund from {}",
                self.status.as_str()
            )));
        }

        let remaining = self.amount - self.refunded_amount;
        if amount > remaining {
            return Err(PaymentError::InsufficientRefundAmount);
        }

        self.refunded_amount += amount;
        if self.refunded_amount >= self.amount {
            self.status = PaymentStatus::Refunded;
        } else {
            self.status = PaymentStatus::PartiallyRefunded;
        }
        self.updated_at = Utc::now();

        Ok(PaymentEvent::Refunded {
            payment_id: self.id,
            invoice_id: self.invoice_id,
            amount,
        })
    }

    /// Create completion event.
    pub fn completion_event(&self) -> EventEnvelope<PaymentEvent> {
        EventEnvelope::new(
            "billing.payment.completed.v1".to_string(),
            1,
            "billing-service".to_string(),
            PaymentEvent::Completed {
                payment_id: self.id,
                invoice_id: self.invoice_id,
                customer_id: self.customer_id,
                amount: self.amount,
            },
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn create_test_payment() -> Payment {
        Payment::create(
            1,
            100,
            200,
            Decimal::new(1000, 2),
            "INR".to_string(),
            PaymentMethod::Upi,
        )
        .unwrap()
    }

    #[test]
    fn test_create_payment() {
        let payment = create_test_payment();
        assert_eq!(payment.id, 1);
        assert_eq!(payment.status, PaymentStatus::Pending);
        assert_eq!(payment.amount, Decimal::new(1000, 2));
    }

    #[test]
    fn test_complete_payment() {
        let mut payment = create_test_payment();
        payment.start_processing().unwrap();
        let event = payment.complete(Some("TXN-123".to_string()), None).unwrap();
        assert_eq!(payment.status, PaymentStatus::Completed);
    }

    #[test]
    fn test_refund_payment() {
        let mut payment = create_test_payment();
        payment.start_processing().unwrap();
        payment.complete(None, None).unwrap();
        let event = payment.refund(Decimal::new(500, 2)).unwrap();
        assert_eq!(payment.status, PaymentStatus::PartiallyRefunded);
        assert_eq!(payment.refunded_amount, Decimal::new(500, 2));
    }

    #[test]
    fn test_full_refund() {
        let mut payment = create_test_payment();
        payment.start_processing().unwrap();
        payment.complete(None, None).unwrap();
        let event = payment.refund(Decimal::new(1000, 2)).unwrap();
        assert_eq!(payment.status, PaymentStatus::Refunded);
    }
}
