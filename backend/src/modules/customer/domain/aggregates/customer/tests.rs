//! Customer aggregate tests.
//!
//! Domain tests for the Customer aggregate root.

#[cfg(test)]
mod tests {
    use super::super::customer::{Customer, CustomerError, CustomerEvent};
    use crate::modules::customer::domain::value_objects::{
        customer_status::CustomerStatus, email::Email, phone::Phone,
    };

    fn create_test_customer() -> Customer {
        Customer::create(
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
        .unwrap()
    }

    #[test]
    fn test_create_customer() {
        let customer = create_test_customer();
        assert_eq!(customer.id.inner(), 1);
        assert_eq!(customer.customer_code, "AX-GEN-202607-0001");
        assert_eq!(customer.first_name, "John");
        assert_eq!(customer.last_name, Some("Doe".to_string()));
        assert_eq!(customer.status, CustomerStatus::Lead);
        assert_eq!(customer.kyc_status, "pending");
    }

    #[test]
    fn test_create_customer_empty_first_name_fails() {
        let result = Customer::create(
            1,
            "CODE".to_string(),
            "".to_string(),
            None,
            None,
            Phone::new("1234567890").unwrap(),
            1,
            None,
            None,
            None,
        );
        assert!(matches!(result, Err(CustomerError::Validation(_))));
    }

    #[test]
    fn test_update_profile() {
        let mut customer = create_test_customer();
        let changed = customer.update_profile(
            Some("Jane"),
            Some("Smith"),
            Some(Email::new("jane@example.com").unwrap()),
            None,
            None,
            Some("New notes"),
        );
        assert!(changed.contains(&"first_name".to_string()));
        assert!(changed.contains(&"last_name".to_string()));
        assert!(changed.contains(&"email".to_string()));
        assert!(changed.contains(&"notes".to_string()));
        assert_eq!(customer.first_name, "Jane");
    }

    #[test]
    fn test_status_transition_lead_to_prospect() {
        let mut customer = create_test_customer();
        let event = customer
            .transition_status(CustomerStatus::Prospect, false, Some("Interested"))
            .unwrap();
        assert_eq!(customer.status, CustomerStatus::Prospect);
        match event {
            CustomerEvent::StatusChanged {
                old_status,
                new_status,
                ..
            } => {
                assert_eq!(old_status, "lead");
                assert_eq!(new_status, "prospect");
            }
            _ => panic!("Expected StatusChanged event"),
        }
    }

    #[test]
    fn test_invalid_status_transition() {
        let mut customer = create_test_customer();
        let result = customer.transition_status(CustomerStatus::Active, false, None);
        assert!(matches!(result, Err(CustomerError::InvalidStatusTransition(_))));
    }

    #[test]
    fn test_activate_without_kyc_fails() {
        let mut customer = create_test_customer();
        customer.status = CustomerStatus::Prospect;
        let result = customer.transition_status(CustomerStatus::Active, false, None);
        assert!(matches!(result, Err(CustomerError::KycRequired)));
    }

    #[test]
    fn test_activate_with_kyc_succeeds() {
        let mut customer = create_test_customer();
        customer.status = CustomerStatus::Prospect;
        customer.kyc_status = "verified".to_string();
        let result = customer.transition_status(CustomerStatus::Active, false, None);
        assert!(result.is_ok());
        assert_eq!(customer.status, CustomerStatus::Active);
    }

    #[test]
    fn test_deactivate_with_active_subscriptions_fails() {
        let mut customer = create_test_customer();
        customer.status = CustomerStatus::Active;
        let result = customer.transition_status(CustomerStatus::Deactivated, true, None);
        assert!(matches!(result, Err(CustomerError::ActiveSubscriptionsExist)));
    }

    #[test]
    fn test_submit_kyc() {
        let mut customer = create_test_customer();
        let event = customer.submit_kyc("aadhaar").unwrap();
        assert_eq!(customer.kyc_status, "pending");
        match event {
            CustomerEvent::KycSubmitted {
                id_proof_type, ..
            } => {
                assert_eq!(id_proof_type, "aadhaar");
            }
            _ => panic!("Expected KycSubmitted event"),
        }
    }

    #[test]
    fn test_verify_kyc_success() {
        let mut customer = create_test_customer();
        let event = customer.verify_kyc(true, None).unwrap();
        assert_eq!(customer.kyc_status, "verified");
        match event {
            CustomerEvent::KycVerified { verified, .. } => {
                assert!(verified);
            }
            _ => panic!("Expected KycVerified event"),
        }
    }

    #[test]
    fn test_verify_kyc_rejection() {
        let mut customer = create_test_customer();
        let event = customer.verify_kyc(false, Some("Invalid document")).unwrap();
        assert_eq!(customer.kyc_status, "rejected");
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

    #[test]
    fn test_full_name() {
        let customer = create_test_customer();
        assert_eq!(customer.full_name(), "John Doe");

        let mut customer_no_last = create_test_customer();
        customer_no_last.last_name = None;
        assert_eq!(customer_no_last.full_name(), "John");
    }

    #[test]
    fn test_can_activate() {
        let mut customer = create_test_customer();
        assert!(!customer.can_activate());

        customer.kyc_status = "verified".to_string();
        assert!(customer.can_activate());

        customer.status = CustomerStatus::Blacklist;
        assert!(!customer.can_activate());
    }

    #[test]
    fn test_creation_event() {
        let customer = create_test_customer();
        let event = customer.creation_event();
        assert_eq!(event.event_type, "customer.created.v1");
        assert_eq!(event.version, 1);
        assert_eq!(event.producer, "customer-service");
    }
}
