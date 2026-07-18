use crate::modules::payment::domain::value_objects::{PaymentId, PaymentMethod, PaymentStatus};

/// Payment aggregate root - represents a payment transaction
#[derive(Debug, Clone, PartialEq)]
pub struct Payment {
    pub id: PaymentId,
    pub payment_number: String,
    pub invoice_id: i64,
    pub customer_id: i64,
    pub branch_id: i64,
    pub amount: rust_decimal::Decimal,
    pub currency: String,
    pub payment_method: PaymentMethod,
    pub status: PaymentStatus,
    pub gateway_transaction_id: Option<String>,
}

/// Domain errors for Payment aggregate
#[derive(Debug, Clone, PartialEq)]
pub enum PaymentDomainError {
    PaymentNotFound(i64),
    InvalidAmount,
    PaymentAlreadyCompleted,
    PaymentAlreadyFailed,
    IdempotencyConflict,
}

impl std::fmt::Display for PaymentDomainError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::PaymentNotFound(id) => write!(f, "Payment {} not found", id),
            Self::InvalidAmount => write!(f, "Payment amount must be greater than zero"),
            Self::PaymentAlreadyCompleted => write!(f, "Payment is already completed"),
            Self::PaymentAlreadyFailed => write!(f, "Payment has already failed"),
            Self::IdempotencyConflict => write!(f, "Idempotency key conflict"),
        }
    }
}

impl std::error::Error for PaymentDomainError {}

impl Payment {
    pub fn new(
        payment_number: String,
        invoice_id: i64,
        customer_id: i64,
        branch_id: i64,
        amount: rust_decimal::Decimal,
        payment_method: PaymentMethod,
    ) -> Result<Self, PaymentDomainError> {
        if amount <= rust_decimal::Decimal::ZERO {
            return Err(PaymentDomainError::InvalidAmount);
        }
        Ok(Self {
            id: PaymentId::new(0),
            payment_number,
            invoice_id,
            customer_id,
            branch_id,
            amount,
            currency: "INR".to_string(),
            payment_method,
            status: PaymentStatus::Pending,
            gateway_transaction_id: None,
        })
    }

    pub fn mark_completed(&mut self, gateway_txn_id: &str) -> Result<(), PaymentDomainError> {
        if self.status == PaymentStatus::Completed {
            return Err(PaymentDomainError::PaymentAlreadyCompleted);
        }
        self.status = PaymentStatus::Completed;
        self.gateway_transaction_id = Some(gateway_txn_id.to_string());
        Ok(())
    }

    pub fn mark_failed(&mut self) -> Result<(), PaymentDomainError> {
        if self.status == PaymentStatus::Failed {
            return Err(PaymentDomainError::PaymentAlreadyFailed);
        }
        self.status = PaymentStatus::Failed;
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new_payment() {
        let payment = Payment::new(
            "PAY-001".to_string(),
            1,
            1,
            1,
            rust_decimal_macros::dec!(708.00),
            PaymentMethod::Upi,
        );
        assert!(payment.is_ok());
        assert_eq!(payment.unwrap().status, PaymentStatus::Pending);
    }

    #[test]
    fn test_invalid_amount() {
        let payment = Payment::new(
            "PAY-001".to_string(),
            1,
            1,
            1,
            rust_decimal_macros::dec!(0),
            PaymentMethod::Upi,
        );
        assert_eq!(payment, Err(PaymentDomainError::InvalidAmount));
    }
}
