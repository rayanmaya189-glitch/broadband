//! Create customer command handler.
//!
//! Handles the creation of a new customer with validation and event publishing.

use crate::common::errors::app_error::AppError;
use crate::common::shared::events::EventEnvelope;
use crate::modules::customer::domain::aggregates::customer::customer::{Customer, CustomerEvent};
use crate::modules::customer::domain::rules::customer_rules;
use crate::modules::customer::domain::value_objects::{email::Email, phone::Phone};

/// Command to create a new customer.
#[derive(Debug, Clone)]
pub struct CreateCustomerCommand {
    pub customer_code: String,
    pub first_name: String,
    pub last_name: Option<String>,
    pub email: Option<String>,
    pub phone: String,
    pub alternate_phone: Option<String>,
    pub branch_id: i64,
    pub lead_id: Option<i64>,
    pub referred_by: Option<i64>,
    pub created_by: Option<i64>,
    pub notes: Option<String>,
}

/// Result of customer creation.
#[derive(Debug)]
pub struct CreateCustomerResult {
    pub customer: Customer,
    pub event: EventEnvelope<CustomerEvent>,
}

/// Command handler for creating customers.
///
/// This handler orchestrates:
/// 1. Input validation
/// 2. Business rule validation
/// 3. Aggregate creation
/// 4. Event generation
pub struct CreateCustomerHandler;

impl CreateCustomerHandler {
    /// Handle the create customer command.
    ///
    /// # Arguments
    /// * `command` - The create customer command
    ///
    /// # Returns
    /// The created customer (with ID=0, to be set by repository) and the domain event.
    pub fn handle(command: CreateCustomerCommand) -> Result<CreateCustomerResult, AppError> {
        // Validate input
        customer_rules::validate_customer_creation(
            &command.first_name,
            &command.phone,
            command.email.as_deref(),
        )
        .map_err(|e| AppError::Validation(e.to_string()))?;

        // Parse value objects
        let email = command
            .email
            .as_ref()
            .map(|e| Email::new(e))
            .transpose()
            .map_err(|e| AppError::Validation(e.to_string()))?;

        let phone = Phone::new(&command.phone)
            .map_err(|e| AppError::Validation(e.to_string()))?;

        // Create aggregate (ID will be set by repository after insert)
        let mut customer = Customer::create(
            command.customer_code,
            command.first_name,
            command.last_name,
            email,
            phone,
            command.branch_id,
            command.lead_id,
            command.referred_by,
            command.created_by,
        )
        .map_err(|e| AppError::Validation(e.to_string()))?;

        // Set optional fields
        if let Some(ref notes) = command.notes {
            customer.notes = Some(notes.clone());
        }

        // Generate domain event
        let event = customer.creation_event();

        Ok(CreateCustomerResult { customer, event })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_create_customer_handler() {
        let command = CreateCustomerCommand {
            customer_code: "AX-GEN-202607-0001".to_string(),
            first_name: "John".to_string(),
            last_name: Some("Doe".to_string()),
            email: Some("john@example.com".to_string()),
            phone: "+1-234-567-8900".to_string(),
            alternate_phone: None,
            branch_id: 1,
            lead_id: None,
            referred_by: None,
            created_by: Some(1),
            notes: Some("Test customer".to_string()),
        };

        let result = CreateCustomerHandler::handle(command).unwrap();
        assert_eq!(result.customer.id.inner(), 0); // ID not yet assigned
        assert_eq!(result.event.event_type, "customer.created.v1");
    }

    #[test]
    fn test_create_customer_handler_validation_error() {
        let command = CreateCustomerCommand {
            customer_code: "AX-GEN-202607-0002".to_string(),
            first_name: "".to_string(),
            last_name: None,
            email: None,
            phone: "+1-234-567-8900".to_string(),
            alternate_phone: None,
            branch_id: 1,
            lead_id: None,
            referred_by: None,
            created_by: None,
            notes: None,
        };

        let result = CreateCustomerHandler::handle(command);
        assert!(result.is_err());
    }
}
