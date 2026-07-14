//! Transition customer status command handler.
//!
//! Handles customer status changes with validation and event publishing.

use crate::common::errors::app_error::AppError;
use crate::modules::customer::domain::aggregates::customer::customer::{Customer, CustomerEvent, CustomerError};
use crate::modules::customer::domain::value_objects::customer_status::CustomerStatus;

/// Command to transition customer status.
#[derive(Debug, Clone)]
pub struct TransitionCustomerStatusCommand {
    pub customer_id: i64,
    pub new_status: String,
    pub reason: Option<String>,
}

/// Result of status transition.
#[derive(Debug)]
pub struct TransitionStatusResult {
    pub customer: Customer,
    pub event: CustomerEvent,
}

/// Command handler for status transitions.
pub struct TransitionCustomerStatusHandler;

impl TransitionCustomerStatusHandler {
    /// Handle the status transition command.
    pub fn handle(
        mut customer: Customer,
        command: TransitionCustomerStatusCommand,
        has_active_subscriptions: bool,
    ) -> Result<TransitionStatusResult, AppError> {
        let new_status = CustomerStatus::from_str(&command.new_status)
            .map_err(|e| AppError::Validation(e.to_string()))?;

        let event = customer
            .transition_status(new_status, has_active_subscriptions, command.reason.as_deref())
            .map_err(|e| match e {
                CustomerError::InvalidStatusTransition(msg) => AppError::Validation(msg),
                CustomerError::KycRequired => AppError::Validation(e.to_string()),
                CustomerError::ActiveSubscriptionsExist => AppError::Conflict(e.to_string()),
                _ => AppError::Internal(e.into()),
            })?;

        Ok(TransitionStatusResult { customer, event })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::modules::customer::domain::value_objects::{email::Email, phone::Phone};

    fn create_test_customer() -> Customer {
        let mut customer = Customer::create(
            1,
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
        customer.status = CustomerStatus::Prospect;
        customer
    }

    #[test]
    fn test_transition_to_active_without_kyc_fails() {
        let customer = create_test_customer();
        let command = TransitionCustomerStatusCommand {
            customer_id: 1,
            new_status: "active".to_string(),
            reason: None,
        };

        let result = TransitionCustomerStatusHandler::handle(customer, command, false);
        assert!(matches!(result, Err(AppError::Validation(_))));
    }

    #[test]
    fn test_transition_to_active_with_kyc_succeeds() {
        let mut customer = create_test_customer();
        customer.kyc_status = "verified".to_string();
        
        let command = TransitionCustomerStatusCommand {
            customer_id: 1,
            new_status: "active".to_string(),
            reason: None,
        };

        let result = TransitionCustomerStatusHandler::handle(customer, command, false);
        assert!(result.is_ok());
    }
}
