//! Customer aggregate root.
//!
//! The Customer aggregate is the consistency boundary for all customer-related
//! operations. It enforces business invariants and produces domain events.

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use thiserror::Error;

use crate::common::shared::events::EventEnvelope;
use crate::modules::customer::domain::value_objects::{
    customer_id::CustomerId, customer_status::CustomerStatus, email::Email, phone::Phone,
};

/// Customer aggregate root.
///
/// # Invariants
/// - Customer cannot be activated without KYC verification
/// - Customer cannot be deleted while active subscriptions exist
/// - Any status change is recorded via domain event
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Customer {
    pub id: CustomerId,
    pub customer_code: String,
    pub first_name: String,
    pub last_name: Option<String>,
    pub email: Option<Email>,
    pub phone: Phone,
    pub alternate_phone: Option<Phone>,
    pub status: CustomerStatus,
    pub branch_id: i64,
    pub lead_id: Option<i64>,
    pub referred_by: Option<i64>,
    pub created_by: Option<i64>,
    pub kyc_status: String,
    pub notes: Option<String>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

/// Domain events produced by the Customer aggregate.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum CustomerEvent {
    Created {
        customer_id: i64,
        customer_code: String,
        first_name: String,
        last_name: Option<String>,
        email: Option<String>,
        phone: String,
        branch_id: i64,
    },
    Updated {
        customer_id: i64,
        changed_fields: Vec<String>,
    },
    StatusChanged {
        customer_id: i64,
        old_status: String,
        new_status: String,
        reason: Option<String>,
    },
    KycSubmitted {
        customer_id: i64,
        id_proof_type: String,
    },
    KycVerified {
        customer_id: i64,
        verified: bool,
        rejection_reason: Option<String>,
    },
}

/// Customer domain errors.
#[derive(Debug, Error)]
pub enum CustomerError {
    #[error("Validation error: {0}")]
    Validation(String),

    #[error("Invalid status transition: {0}")]
    InvalidStatusTransition(String),

    #[error("KYC verification required before activation")]
    KycRequired,

    #[error("Cannot deactivate customer with active subscriptions")]
    ActiveSubscriptionsExist,

    #[error("Customer not found")]
    NotFound,
}

impl Customer {
    /// Create a new Customer aggregate.
    ///
    /// The ID will be assigned by the persistence layer (repository).
    /// This method creates a new customer with Lead status.
    ///
    /// # Arguments
    /// * `customer_code` - The customer's code (e.g., "AX-GEN-202607-0001")
    /// * `first_name` - The customer's first name
    /// * `last_name` - Optional last name
    /// * `email` - Optional email address
    /// * `phone` - Phone number (required)
    /// * `branch_id` - The branch this customer belongs to
    /// * `lead_id` - Optional lead that converted to this customer
    /// * `referred_by` - Optional referrer customer ID
    /// * `created_by` - The user who created this customer
    pub fn create(
        customer_code: String,
        first_name: String,
        last_name: Option<String>,
        email: Option<Email>,
        phone: Phone,
        branch_id: i64,
        lead_id: Option<i64>,
        referred_by: Option<i64>,
        created_by: Option<i64>,
    ) -> Result<Self, CustomerError> {
        // Validate first name
        if first_name.trim().is_empty() {
            return Err(CustomerError::Validation(
                "First name cannot be empty".to_string(),
            ));
        }

        let now = Utc::now();

        Ok(Self {
            id: CustomerId::new(0), // Will be set by repository after insert
            customer_code,
            first_name,
            last_name,
            email,
            phone,
            alternate_phone: None,
            status: CustomerStatus::Lead,
            branch_id,
            lead_id,
            referred_by,
            created_by,
            kyc_status: "pending".to_string(),
            notes: None,
            created_at: now,
            updated_at: now,
        })
    }

    /// Set the ID after persistence (called by repository after insert).
    pub fn set_id(&mut self, id: i64) {
        self.id = CustomerId::new(id);
    }

    /// Update customer profile information.
    ///
    /// Returns a list of changed fields for event publishing.
    pub fn update_profile(
        &mut self,
        first_name: Option<&str>,
        last_name: Option<&str>,
        email: Option<Email>,
        phone: Option<Phone>,
        alternate_phone: Option<Phone>,
        notes: Option<&str>,
    ) -> Vec<String> {
        let mut changed = Vec::new();

        if let Some(name) = first_name {
            if !name.trim().is_empty() && self.first_name != name {
                self.first_name = name.to_string();
                changed.push("first_name".to_string());
            }
        }

        if let Some(name) = last_name {
            if self.last_name.as_deref() != Some(name) {
                self.last_name = Some(name.to_string());
                changed.push("last_name".to_string());
            }
        }

        if let Some(e) = email {
            if self.email.as_ref().map(|x| x.as_str()) != Some(e.as_str()) {
                self.email = Some(e);
                changed.push("email".to_string());
            }
        }

        if let Some(p) = phone {
            if self.phone.as_str() != p.as_str() {
                self.phone = p;
                changed.push("phone".to_string());
            }
        }

        if let Some(p) = alternate_phone {
            self.alternate_phone = Some(p);
            changed.push("alternate_phone".to_string());
        }

        if let Some(n) = notes {
            if self.notes.as_deref() != Some(n) {
                self.notes = Some(n.to_string());
                changed.push("notes".to_string());
            }
        }

        if !changed.is_empty() {
            self.updated_at = Utc::now();
        }

        changed
    }

    /// Transition the customer to a new status.
    ///
    /// # Invariants
    /// - Valid status transition must exist
    /// - KYC must be verified before activation
    /// - Cannot deactivate with active subscriptions
    pub fn transition_status(
        &mut self,
        new_status: CustomerStatus,
        has_active_subscriptions: bool,
        reason: Option<&str>,
    ) -> Result<CustomerEvent, CustomerError> {
        // Check if transition is valid
        if !self.status.can_transition_to(&new_status) {
            return Err(CustomerError::InvalidStatusTransition(format!(
                "Cannot transition from {} to {}",
                self.status, new_status
            )));
        }

        // KYC must be verified before activation
        if new_status == CustomerStatus::Active && self.kyc_status != "verified" {
            return Err(CustomerError::KycRequired);
        }

        // Cannot deactivate with active subscriptions
        if new_status == CustomerStatus::Deactivated && has_active_subscriptions {
            return Err(CustomerError::ActiveSubscriptionsExist);
        }

        let old_status = self.status.clone();
        self.status = new_status.clone();
        self.updated_at = Utc::now();

        Ok(CustomerEvent::StatusChanged {
            customer_id: self.id.inner(),
            old_status: old_status.as_str().to_string(),
            new_status: new_status.as_str().to_string(),
            reason: reason.map(|s| s.to_string()),
        })
    }

    /// Submit KYC documents.
    pub fn submit_kyc(&mut self, id_proof_type: &str) -> Result<CustomerEvent, CustomerError> {
        if id_proof_type.trim().is_empty() {
            return Err(CustomerError::Validation(
                "ID proof type is required".to_string(),
            ));
        }

        self.kyc_status = "pending".to_string();
        self.updated_at = Utc::now();

        Ok(CustomerEvent::KycSubmitted {
            customer_id: self.id.inner(),
            id_proof_type: id_proof_type.to_string(),
        })
    }

    /// Verify or reject KYC.
    pub fn verify_kyc(
        &mut self,
        verified: bool,
        rejection_reason: Option<&str>,
    ) -> Result<CustomerEvent, CustomerError> {
        if verified {
            self.kyc_status = "verified".to_string();
        } else {
            self.kyc_status = "rejected".to_string();
        }
        self.updated_at = Utc::now();

        Ok(CustomerEvent::KycVerified {
            customer_id: self.id.inner(),
            verified,
            rejection_reason: rejection_reason.map(|s| s.to_string()),
        })
    }

    /// Check if customer can be activated.
    pub fn can_activate(&self) -> bool {
        self.kyc_status == "verified" && self.status != CustomerStatus::Blacklist
    }

    /// Get the customer's full name.
    pub fn full_name(&self) -> String {
        match &self.last_name {
            Some(last) => format!("{} {}", self.first_name, last),
            None => self.first_name.clone(),
        }
    }

    /// Create a domain event for customer creation.
    pub fn creation_event(&self) -> EventEnvelope<CustomerEvent> {
        EventEnvelope::new(
            "customer.created.v1".to_string(),
            1,
            "customer-service".to_string(),
            CustomerEvent::Created {
                customer_id: self.id.inner(),
                customer_code: self.customer_code.clone(),
                first_name: self.first_name.clone(),
                last_name: self.last_name.clone(),
                email: self.email.as_ref().map(|e| e.as_str().to_string()),
                phone: self.phone.as_str().to_string(),
                branch_id: self.branch_id,
            },
        )
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
