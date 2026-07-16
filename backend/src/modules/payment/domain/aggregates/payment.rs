use crate::modules::payment::domain::value_objects::{PaymentId, PaymentMethod, PaymentStatus};

/// Payment aggregate root
#[derive(Debug, Clone)]
pub struct Payment {
    pub id: PaymentId,
    pub invoice_id: i64,
    pub customer_id: i64,
    pub branch_id: i64,
    pub amount: rust_decimal::Decimal,
    pub currency: String,
    pub payment_method: PaymentMethod,
    pub payment_gateway: Option<String>,
    pub gateway_transaction_id: Option<String>,
    pub status: PaymentStatus,
    pub processed_at: Option<chrono::DateTime<chrono::Utc>>,
}

/// Domain errors for Payment aggregate
#[derive(Debug, Clone, PartialEq)]
pub enum PaymentDomainError {
    AlreadyCompleted,
    AlreadyFailed,
    InvalidAmount,
    PaymentNotFound(i64),
    GatewayNotConfigured,
    DuplicateTransaction(String),
    InsufficientAmount,
}

impl std::fmt::Display for PaymentDomainError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::AlreadyCompleted => write!(f, "Payment is already completed"),
            Self::AlreadyFailed => write!(f, "Payment has already failed"),
            Self::InvalidAmount => write!(f, "Payment amount must be greater than zero"),
            Self::PaymentNotFound(id) => write!(f, "Payment {} not found", id),
            Self::GatewayNotConfigured => write!(f, "Payment gateway is not configured"),
            Self::DuplicateTransaction(ref tx_id) => {
                write!(f, "Duplicate transaction: {}", tx_id)
            }
            Self::InsufficientAmount => write!(f, "Payment amount is less than required"),
        }
    }
}

impl std::error::Error for PaymentDomainError {}

impl Payment {
    pub fn new(
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
            invoice_id,
            customer_id,
            branch_id,
            amount,
            currency: "INR".to_string(),
            payment_method,
            payment_gateway: None,
            gateway_transaction_id: None,
            status: PaymentStatus::Pending,
            processed_at: None,
        })
    }

    pub fn complete(&mut self, gateway_transaction_id: String) -> Result<(), PaymentDomainError> {
        if self.status == PaymentStatus::Completed {
            return Err(PaymentDomainError::AlreadyCompleted);
        }
        self.gateway_transaction_id = Some(gateway_transaction_id);
        self.status = PaymentStatus::Completed;
        self.processed_at = Some(chrono::Utc::now());
        Ok(())
    }

    pub fn fail(&mut self) -> Result<(), PaymentDomainError> {
        if self.status == PaymentStatus::Completed {
            return Err(PaymentDomainError::AlreadyCompleted);
        }
        if self.status == PaymentStatus::Failed {
            return Err(PaymentDomainError::AlreadyFailed);
        }
        self.status = PaymentStatus::Failed;
        Ok(())
    }

    pub fn is_completed(&self) -> bool {
        self.status == PaymentStatus::Completed
    }

    pub fn can_be_refunded(&self) -> bool {
        self.status == PaymentStatus::Completed
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new_payment() {
        let payment = Payment::new(
            1,
            1,
            1,
            rust_decimal::Decimal::new(70800, 2),
            PaymentMethod::Upi,
        );
        assert!(payment.is_ok());
        let payment = payment.unwrap();
        assert_eq!(payment.status, PaymentStatus::Pending);
        assert_eq!(payment.currency, "INR");
    }

    #[test]
    fn test_invalid_amount() {
        let payment = Payment::new(
            1,
            1,
            1,
            rust_decimal::Decimal::ZERO,
            PaymentMethod::Upi,
        );
        assert_eq!(payment, Err(PaymentDomainError::InvalidAmount));
    }

    #[test]
    fn test_complete_payment() {
        let mut payment = Payment::new(
            1,
            1,
            1,
            rust_decimal::Decimal::new(70800, 2),
            PaymentMethod::Upi,
        )
        .unwrap();
        let result = payment.complete("TXN-123".to_string());
        assert!(result.is_ok());
        assert!(payment.is_completed());
        assert!(payment.can_be_refunded());
    }

    #[test]
    fn test_fail_payment() {
        let mut payment = Payment::new(
            1,
            1,
            1,
            rust_decimal::Decimal::new(70800, 2),
            PaymentMethod::Card,
        )
        .unwrap();
        let result = payment.fail();
        assert!(result.is_ok());
        assert_eq!(payment.status, PaymentStatus::Failed);
    }
}
