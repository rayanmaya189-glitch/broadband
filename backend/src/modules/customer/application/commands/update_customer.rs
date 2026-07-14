//! Update customer command handler.
//!
//! Handles updating customer profile information with event publishing.

use crate::common::errors::app_error::AppError;
use crate::modules::customer::domain::aggregates::customer::customer::Customer;
use crate::modules::customer::domain::value_objects::{email::Email, phone::Phone};

/// Command to update a customer.
#[derive(Debug, Clone)]
pub struct UpdateCustomerCommand {
    pub customer_id: i64,
    pub first_name: Option<String>,
    pub last_name: Option<String>,
    pub email: Option<String>,
    pub phone: Option<String>,
    pub alternate_phone: Option<String>,
    pub notes: Option<String>,
}

/// Result of customer update.
#[derive(Debug)]
pub struct UpdateCustomerResult {
    pub customer: Customer,
    pub changed_fields: Vec<String>,
}

/// Command handler for updating customers.
pub struct UpdateCustomerHandler;

impl UpdateCustomerHandler {
    /// Handle the update customer command.
    pub fn handle(
        mut customer: Customer,
        command: UpdateCustomerCommand,
    ) -> Result<UpdateCustomerResult, AppError> {
        // Parse value objects
        let email = command
            .email
            .as_ref()
            .map(|e| Email::new(e))
            .transpose()
            .map_err(|e| AppError::Validation(e.to_string()))?;

        let phone = command
            .phone
            .as_ref()
            .map(|p| Phone::new(p))
            .transpose()
            .map_err(|e| AppError::Validation(e.to_string()))?;

        let alternate_phone = command
            .alternate_phone
            .as_ref()
            .map(|p| Phone::new(p))
            .transpose()
            .map_err(|e| AppError::Validation(e.to_string()))?;

        // Update aggregate
        let changed_fields = customer.update_profile(
            command.first_name.as_deref(),
            command.last_name.as_deref(),
            email,
            phone,
            alternate_phone,
            command.notes.as_deref(),
        );

        Ok(UpdateCustomerResult {
            customer,
            changed_fields,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

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
    fn test_update_customer_handler() {
        let customer = create_test_customer();
        let command = UpdateCustomerCommand {
            customer_id: 1,
            first_name: Some("Jane".to_string()),
            last_name: Some("Smith".to_string()),
            email: None,
            phone: None,
            alternate_phone: None,
            notes: None,
        };

        let result = UpdateCustomerHandler::handle(customer, command).unwrap();
        assert_eq!(result.customer.first_name, "Jane");
        assert!(result.changed_fields.contains(&"first_name".to_string()));
    }
}
