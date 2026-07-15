use crate::modules::customer::domain::value_objects::{Email, Phone, CustomerStatus, CustomerId};

/// Customer aggregate root - the core business entity
#[derive(Debug, Clone)]
pub struct Customer {
    pub id: CustomerId,
    pub customer_code: String,
    pub branch_id: i64,
    pub name: String,
    pub email: Option<Email>,
    pub phone: Phone,
    pub alternate_phone: Option<Phone>,
    pub status: CustomerStatus,
    pub referral_code: Option<String>,
    pub referred_by: Option<i64>,
}

/// Domain errors for Customer aggregate
#[derive(Debug, Clone, PartialEq)]
pub enum CustomerDomainError {
    /// Customer cannot be activated without KYC verification
    KycRequired,
    /// Customer cannot be deleted while active subscriptions exist
    ActiveSubscriptionsExist,
    /// Customer cannot be suspended if already suspended
    AlreadySuspended,
    /// Customer cannot be activated if already active
    AlreadyActive,
    /// Invalid email format
    InvalidEmail,
    /// Invalid phone format
    InvalidPhone,
    /// Customer code already exists
    DuplicateCustomerCode,
    /// Customer not found
    NotFound(i64),
}

impl std::fmt::Display for CustomerDomainError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::KycRequired => write!(f, "KYC verification is required before activation"),
            Self::ActiveSubscriptionsExist => write!(f, "Cannot delete customer with active subscriptions"),
            Self::AlreadySuspended => write!(f, "Customer is already suspended"),
            Self::AlreadyActive => write!(f, "Customer is already active"),
            Self::InvalidEmail => write!(f, "Invalid email format"),
            Self::InvalidPhone => write!(f, "Invalid phone format"),
            Self::DuplicateCustomerCode => write!(f, "Customer code already exists"),
            Self::NotFound(id) => write!(f, "Customer {} not found", id),
        }
    }
}

impl std::error::Error for CustomerDomainError {}

impl Customer {
    /// Create a new customer (factory method)
    pub fn new(
        customer_code: String,
        branch_id: i64,
        name: String,
        email: Option<String>,
        phone: String,
        alternate_phone: Option<String>,
        referral_code: Option<String>,
        referred_by: Option<i64>,
    ) -> Result<Self, CustomerDomainError> {
        // Validate email if provided
        let email_obj = email.as_deref().map(Email::new).transpose()?;
        
        // Validate phone
        let phone_obj = Phone::new(&phone)?;
        let alt_phone_obj = alternate_phone.map(|p| Phone::new(&p)).transpose()?;

        Ok(Self {
            id: CustomerId::new(0), // Will be set by database
            customer_code,
            branch_id,
            name,
            email: email_obj,
            phone: phone_obj,
            alternate_phone: alt_phone_obj,
            status: CustomerStatus::Pending,
            referral_code,
            referred_by,
        })
    }

    /// Activate customer (requires KYC verification)
    pub fn activate(&mut self, kyc_verified: bool) -> Result<(), CustomerDomainError> {
        if !kyc_verified {
            return Err(CustomerDomainError::KycRequired);
        }
        if self.status == CustomerStatus::Active {
            return Err(CustomerDomainError::AlreadyActive);
        }
        self.status = CustomerStatus::Active;
        Ok(())
    }

    /// Suspend customer
    pub fn suspend(&mut self) -> Result<(), CustomerDomainError> {
        if self.status == CustomerStatus::Suspended {
            return Err(CustomerDomainError::AlreadySuspended);
        }
        self.status = CustomerStatus::Suspended;
        // TODO: Emit customer.suspended.v1 event
        Ok(())
    }

    /// Check if customer can be deleted
    pub fn can_delete(&self, has_active_subscriptions: bool) -> Result<(), CustomerDomainError> {
        if has_active_subscriptions {
            return Err(CustomerDomainError::ActiveSubscriptionsExist);
        }
        Ok(())
    }

    /// Soft delete customer
    pub fn soft_delete(&mut self) {
        self.status = CustomerStatus::Deleted;
    }

    /// Check if customer is active
    pub fn is_active(&self) -> bool {
        self.status == CustomerStatus::Active
    }

    /// Check if customer needs KYC
    pub fn needs_kyc(&self) -> bool {
        self.status == CustomerStatus::Pending
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new_customer() {
        let customer = Customer::new(
            "CUST-001".to_string(),
            1,
            "John Doe".to_string(),
            Some("john@example.com".to_string()),
            "+919876543210".to_string(),
            None,
            None,
            None,
        );
        assert!(customer.is_ok());
        let customer = customer.unwrap();
        assert_eq!(customer.status, CustomerStatus::Pending);
    }

    #[test]
    fn test_activate_without_kyc_fails() {
        let mut customer = Customer::new(
            "CUST-001".to_string(),
            1,
            "John Doe".to_string(),
            None,
            "+919876543210".to_string(),
            None,
            None,
            None,
        ).unwrap();
        
        let result = customer.activate(false);
        assert_eq!(result, Err(CustomerDomainError::KycRequired));
        assert_eq!(customer.status, CustomerStatus::Pending);
    }

    #[test]
    fn test_activate_with_kyc_succeeds() {
        let mut customer = Customer::new(
            "CUST-001".to_string(),
            1,
            "John Doe".to_string(),
            None,
            "+919876543210".to_string(),
            None,
            None,
            None,
        ).unwrap();
        
        let result = customer.activate(true);
        assert!(result.is_ok());
        assert_eq!(customer.status, CustomerStatus::Active);
    }

    #[test]
    fn test_suspend_active_customer() {
        let mut customer = Customer::new(
            "CUST-001".to_string(),
            1,
            "John Doe".to_string(),
            None,
            "+919876543210".to_string(),
            None,
            None,
            None,
        ).unwrap();
        
        customer.activate(true).unwrap();
        let result = customer.suspend("non-payment");
        assert!(result.is_ok());
        assert_eq!(customer.status, CustomerStatus::Suspended);
    }

    #[test]
    fn test_suspend_already_suspended_fails() {
        let mut customer = Customer::new(
            "CUST-001".to_string(),
            1,
            "John Doe".to_string(),
            None,
            "+919876543210".to_string(),
            None,
            None,
            None,
        ).unwrap();
        
        customer.activate(true).unwrap();
        customer.suspend("non-payment").unwrap();
        let result = customer.suspend("another reason");
        assert_eq!(result, Err(CustomerDomainError::AlreadySuspended));
    }

    #[test]
    fn test_delete_with_active_subscriptions_fails() {
        let customer = Customer::new(
            "CUST-001".to_string(),
            1,
            "John Doe".to_string(),
            None,
            "+919876543210".to_string(),
            None,
            None,
            None,
        ).unwrap();
        
        let result = customer.can_delete(true);
        assert_eq!(result, Err(CustomerDomainError::ActiveSubscriptionsExist));
    }

    #[test]
    fn test_delete_without_subscriptions_succeeds() {
        let customer = Customer::new(
            "CUST-001".to_string(),
            1,
            "John Doe".to_string(),
            None,
            "+919876543210".to_string(),
            None,
            None,
            None,
        ).unwrap();
        
        let result = customer.can_delete(false);
        assert!(result.is_ok());
    }
}
