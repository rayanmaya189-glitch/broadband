//! Submit KYC command handler.
//!
//! Handles KYC document submission with validation.

use crate::common::errors::app_error::AppError;
use crate::modules::customer::domain::aggregates::customer::customer::{Customer, CustomerEvent};
use crate::modules::customer::domain::rules::customer_rules;

/// Command to submit KYC.
#[derive(Debug, Clone)]
pub struct SubmitKycCommand {
    pub customer_id: i64,
    pub id_proof_type: String,
    pub id_proof_number: String,
}

/// Command handler for KYC submission.
pub struct SubmitKycHandler;

impl SubmitKycHandler {
    /// Handle the KYC submission command.
    pub fn handle(
        mut customer: Customer,
        command: SubmitKycCommand,
    ) -> Result<CustomerEvent, AppError> {
        // Validate KYC submission
        customer_rules::validate_kyc_submission(&command.id_proof_type)
            .map_err(|e| AppError::Validation(e.to_string()))?;

        let event = customer
            .submit_kyc(&command.id_proof_type)
            .map_err(|e| AppError::Validation(e.to_string()))?;

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
    fn test_submit_kyc_valid() {
        let customer = create_test_customer();
        let command = SubmitKycCommand {
            customer_id: 1,
            id_proof_type: "aadhaar".to_string(),
            id_proof_number: "1234-5678-9012".to_string(),
        };

        let result = SubmitKycHandler::handle(customer, command);
        assert!(result.is_ok());
    }

    #[test]
    fn test_submit_kyc_invalid_type() {
        let customer = create_test_customer();
        let command = SubmitKycCommand {
            customer_id: 1,
            id_proof_type: "invalid".to_string(),
            id_proof_number: "1234".to_string(),
        };

        let result = SubmitKycHandler::handle(customer, command);
        assert!(result.is_err());
    }
}
