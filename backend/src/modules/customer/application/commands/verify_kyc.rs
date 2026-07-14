//! Verify KYC command handler.
//!
//! Handles KYC verification/rejection with event publishing.

use crate::common::errors::app_error::AppError;
use crate::modules::customer::domain::aggregates::customer::customer::{Customer, CustomerEvent};

/// Command to verify or reject KYC.
#[derive(Debug, Clone)]
pub struct VerifyKycCommand {
    pub customer_id: i64,
    pub verified: bool,
    pub rejection_reason: Option<String>,
}

/// Command handler for KYC verification.
pub struct VerifyKycHandler;

impl VerifyKycHandler {
    /// Handle the KYC verification command.
    pub fn handle(
        mut customer: Customer,
        command: VerifyKycCommand,
    ) -> Result<CustomerEvent, AppError> {
        let event = customer
            .verify_kyc(command.verified, command.rejection_reason.as_deref())
            .map_err(|e| AppError::Internal(e.into()))?;

        Ok(event)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::modules::customer::domain::value_objects::{email::Email, phone::Phone};

    fn create_test_customer() -> Customer {
        let mut customer = Customer::create(
            "AX-GEN-202607-0001".to_string(),
            "John".to_string(),
            Some("Doe".to_string()),
            Some(Email::new("john@example.com").unwrap()),
            Phone::new("+1-234-567-8900").unwrap(),
            1,
            None,
            None,
            Some(1),
        )
        .unwrap();
        customer.set_id(1);
        customer
    }

    #[test]
    fn test_verify_kyc_success() {
        let customer = create_test_customer();
        let command = VerifyKycCommand {
            customer_id: 1,
            verified: true,
            rejection_reason: None,
        };

        let event = VerifyKycHandler::handle(customer, command).unwrap();
        match event {
            CustomerEvent::KycVerified { verified, .. } => assert!(verified),
            _ => panic!("Expected KycVerified event"),
        }
    }

    #[test]
    fn test_verify_kyc_rejection() {
        let customer = create_test_customer();
        let command = VerifyKycCommand {
            customer_id: 1,
            verified: false,
            rejection_reason: Some("Invalid document".to_string()),
        };

        let event = VerifyKycHandler::handle(customer, command).unwrap();
        match event {
            CustomerEvent::KycVerified {
                verified,
                rejection_reason,
                ..
            } => {
                assert!(!verified);
                assert_eq!(rejection_reason, Some("Invalid document".to_string()));
            }
            _ => panic!("Expected KycVerified event"),
        }
    }
}
