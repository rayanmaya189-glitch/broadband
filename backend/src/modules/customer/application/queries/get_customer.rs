//! Get customer query handler.
//!
//! Handles retrieving a single customer by ID.

use crate::common::errors::app_error::AppError;
use crate::modules::customer::domain::aggregates::customer::customer::Customer;

/// Query to get a customer by ID.
#[derive(Debug, Clone)]
pub struct GetCustomerQuery {
    pub customer_id: i64,
}

/// Query handler for getting a customer.
pub struct GetCustomerHandler;

impl GetCustomerHandler {
    /// Execute the get customer query.
    ///
    /// # Arguments
    /// * `customer` - The customer aggregate (already loaded from repository)
    pub fn execute(customer: Option<Customer>) -> Result<Customer, AppError> {
        customer.ok_or_else(|| AppError::NotFound("Customer not found".to_string()))
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
    fn test_get_customer_found() {
        let customer = create_test_customer();
        let result = GetCustomerHandler::execute(Some(customer));
        assert!(result.is_ok());
    }

    #[test]
    fn test_get_customer_not_found() {
        let result = GetCustomerHandler::execute(None);
        assert!(matches!(result, Err(AppError::NotFound(_))));
    }
}
