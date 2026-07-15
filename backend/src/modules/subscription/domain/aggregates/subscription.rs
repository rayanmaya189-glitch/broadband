use crate::modules::subscription::domain::value_objects::{SubscriptionId, SubscriptionStatus};

/// Subscription aggregate root
#[derive(Debug, Clone)]
pub struct Subscription {
    pub id: SubscriptionId,
    pub customer_id: i64,
    pub branch_id: i64,
    pub plan_id: i64,
    pub status: SubscriptionStatus,
    pub billing_period_months: i32,
    pub start_date: chrono::NaiveDate,
    pub end_date: Option<chrono::NaiveDate>,
    pub next_billing_date: Option<chrono::NaiveDate>,
    pub auto_renew: bool,
}

/// Domain errors for Subscription aggregate
#[derive(Debug, Clone, PartialEq)]
pub enum SubscriptionDomainError {
    /// Subscription cannot be cancelled if already cancelled
    AlreadyCancelled,
    /// Subscription cannot be suspended if already suspended
    AlreadySuspended,
    /// Subscription cannot be activated if already active
    AlreadyActive,
    /// Subscription cannot be activated if expired
    Expired,
    /// Invalid billing period
    InvalidBillingPeriod,
    /// Start date cannot be in the future
    StartDateInFuture,
    /// End date must be after start date
    EndDateBeforeStart,
    /// Subscription not found
    NotFound(i64),
}

impl std::fmt::Display for SubscriptionDomainError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::AlreadyCancelled => write!(f, "Subscription is already cancelled"),
            Self::AlreadySuspended => write!(f, "Subscription is already suspended"),
            Self::AlreadyActive => write!(f, "Subscription is already active"),
            Self::Expired => write!(f, "Subscription has expired"),
            Self::InvalidBillingPeriod => write!(f, "Invalid billing period"),
            Self::StartDateInFuture => write!(f, "Start date cannot be in the future"),
            Self::EndDateBeforeStart => write!(f, "End date must be after start date"),
            Self::NotFound(id) => write!(f, "Subscription {} not found", id),
        }
    }
}

impl std::error::Error for SubscriptionDomainError {}

impl Subscription {
    /// Create a new subscription
    pub fn new(
        customer_id: i64,
        branch_id: i64,
        plan_id: i64,
        billing_period_months: i32,
        start_date: chrono::NaiveDate,
        auto_renew: bool,
    ) -> Result<Self, SubscriptionDomainError> {
        if billing_period_months <= 0 || billing_period_months > 12 {
            return Err(SubscriptionDomainError::InvalidBillingPeriod);
        }

        if start_date < chrono::Utc::now().date_naive() {
            return Err(SubscriptionDomainError::StartDateInFuture);
        }

        Ok(Self {
            id: SubscriptionId::new(0),
            customer_id,
            branch_id,
            plan_id,
            status: SubscriptionStatus::Pending,
            billing_period_months,
            start_date,
            end_date: None,
            next_billing_date: Some(start_date),
            auto_renew,
        })
    }

    /// Activate subscription
    pub fn activate(&mut self) -> Result<(), SubscriptionDomainError> {
        if self.status == SubscriptionStatus::Active {
            return Err(SubscriptionDomainError::AlreadyActive);
        }
        if self.status == SubscriptionStatus::Expired {
            return Err(SubscriptionDomainError::Expired);
        }
        self.status = SubscriptionStatus::Active;
        Ok(())
    }

    /// Suspend subscription
    pub fn suspend(&mut self) -> Result<(), SubscriptionDomainError> {
        if self.status == SubscriptionStatus::Suspended {
            return Err(SubscriptionDomainError::AlreadySuspended);
        }
        self.status = SubscriptionStatus::Suspended;
        Ok(())
    }

    /// Cancel subscription
    pub fn cancel(&mut self) -> Result<(), SubscriptionDomainError> {
        if self.status == SubscriptionStatus::Cancelled {
            return Err(SubscriptionDomainError::AlreadyCancelled);
        }
        self.status = SubscriptionStatus::Cancelled;
        self.auto_renew = false;
        Ok(())
    }

    /// Check if subscription is active
    pub fn is_active(&self) -> bool {
        self.status == SubscriptionStatus::Active
    }

    /// Check if subscription can be modified
    pub fn can_be_modified(&self) -> bool {
        matches!(self.status, SubscriptionStatus::Active | SubscriptionStatus::Pending)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new_subscription() {
        let sub = Subscription::new(
            1, 1, 1, 1,
            chrono::Utc::now().date_naive(),
            true,
        );
        assert!(sub.is_ok());
        let sub = sub.unwrap();
        assert_eq!(sub.status, SubscriptionStatus::Pending);
    }

    #[test]
    fn test_activate_subscription() {
        let mut sub = Subscription::new(
            1, 1, 1, 1,
            chrono::Utc::now().date_naive(),
            true,
        ).unwrap();
        
        let result = sub.activate();
        assert!(result.is_ok());
        assert_eq!(sub.status, SubscriptionStatus::Active);
    }

    #[test]
    fn test_suspend_subscription() {
        let mut sub = Subscription::new(
            1, 1, 1, 1,
            chrono::Utc::now().date_naive(),
            true,
        ).unwrap();
        
        sub.activate().unwrap();
        let result = sub.suspend();
        assert!(result.is_ok());
        assert_eq!(sub.status, SubscriptionStatus::Suspended);
    }

    #[test]
    fn test_cancel_subscription() {
        let mut sub = Subscription::new(
            1, 1, 1, 1,
            chrono::Utc::now().date_naive(),
            true,
        ).unwrap();
        
        sub.activate().unwrap();
        let result = sub.cancel();
        assert!(result.is_ok());
        assert_eq!(sub.status, SubscriptionStatus::Cancelled);
        assert!(!sub.auto_renew);
    }
}
