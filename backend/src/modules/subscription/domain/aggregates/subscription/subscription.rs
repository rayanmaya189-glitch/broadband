//! Subscription aggregate root.

use chrono::{DateTime, NaiveDate, Utc};
use serde::{Deserialize, Serialize};
use thiserror::Error;

use crate::common::shared::events::EventEnvelope;

/// Subscription aggregate root.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Subscription {
    pub id: i64,
    pub customer_id: i64,
    pub plan_id: i64,
    pub branch_id: i64,
    pub status: SubscriptionStatus,
    pub start_date: NaiveDate,
    pub end_date: Option<NaiveDate>,
    pub next_billing_date: NaiveDate,
    pub billing_cycle: BillingCycle,
    pub auto_renew: bool,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

/// Subscription status.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum SubscriptionStatus {
    Active,
    Suspended,
    Cancelled,
    Expired,
    Pending,
}

impl SubscriptionStatus {
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::Active => "active",
            Self::Suspended => "suspended",
            Self::Cancelled => "cancelled",
            Self::Expired => "expired",
            Self::Pending => "pending",
        }
    }

    pub fn from_str(s: &str) -> Result<Self, SubscriptionError> {
        match s.to_lowercase().as_str() {
            "active" => Ok(Self::Active),
            "suspended" => Ok(Self::Suspended),
            "cancelled" => Ok(Self::Cancelled),
            "expired" => Ok(Self::Expired),
            "pending" => Ok(Self::Pending),
            _ => Err(SubscriptionError::InvalidStatus(s.to_string())),
        }
    }
}

/// Billing cycle.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum BillingCycle {
    Monthly,
    Quarterly,
    HalfYearly,
    Yearly,
}

impl BillingCycle {
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::Monthly => "monthly",
            Self::Quarterly => "quarterly",
            Self::HalfYearly => "half_yearly",
            Self::Yearly => "yearly",
        }
    }
}

/// Subscription domain events.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum SubscriptionEvent {
    Created {
        subscription_id: i64,
        customer_id: i64,
        plan_id: i64,
    },
    Activated {
        subscription_id: i64,
        customer_id: i64,
    },
    Suspended {
        subscription_id: i64,
        customer_id: i64,
        reason: Option<String>,
    },
    Cancelled {
        subscription_id: i64,
        customer_id: i64,
        reason: Option<String>,
    },
    PlanChanged {
        subscription_id: i64,
        customer_id: i64,
        old_plan_id: i64,
        new_plan_id: i64,
    },
}

/// Subscription domain errors.
#[derive(Debug, Error)]
pub enum SubscriptionError {
    #[error("Invalid status: {0}")]
    InvalidStatus(String),

    #[error("Invalid status transition: {0}")]
    InvalidTransition(String),

    #[error("Validation error: {0}")]
    Validation(String),

    #[error("Subscription not found")]
    NotFound,
}

impl Subscription {
    /// Create a new subscription.
    pub fn create(
        id: i64,
        customer_id: i64,
        plan_id: i64,
        branch_id: i64,
        start_date: NaiveDate,
        billing_cycle: BillingCycle,
    ) -> Result<Self, SubscriptionError> {
        let now = Utc::now();

        Ok(Self {
            id,
            customer_id,
            plan_id,
            branch_id,
            status: SubscriptionStatus::Pending,
            start_date,
            end_date: None,
            next_billing_date: start_date,
            billing_cycle,
            auto_renew: true,
            created_at: now,
            updated_at: now,
        })
    }

    /// Activate subscription.
    pub fn activate(&mut self) -> Result<SubscriptionEvent, SubscriptionError> {
        if self.status != SubscriptionStatus::Pending {
            return Err(SubscriptionError::InvalidTransition(format!(
                "Cannot activate from {}",
                self.status.as_str()
            )));
        }
        self.status = SubscriptionStatus::Active;
        self.updated_at = Utc::now();

        Ok(SubscriptionEvent::Activated {
            subscription_id: self.id,
            customer_id: self.customer_id,
        })
    }

    /// Suspend subscription.
    pub fn suspend(&mut self, reason: Option<&str>) -> Result<SubscriptionEvent, SubscriptionError> {
        if self.status != SubscriptionStatus::Active {
            return Err(SubscriptionError::InvalidTransition(format!(
                "Cannot suspend from {}",
                self.status.as_str()
            )));
        }
        self.status = SubscriptionStatus::Suspended;
        self.updated_at = Utc::now();

        Ok(SubscriptionEvent::Suspended {
            subscription_id: self.id,
            customer_id: self.customer_id,
            reason: reason.map(|s| s.to_string()),
        })
    }

    /// Cancel subscription.
    pub fn cancel(&mut self, reason: Option<&str>) -> Result<SubscriptionEvent, SubscriptionError> {
        if self.status == SubscriptionStatus::Cancelled || self.status == SubscriptionStatus::Expired {
            return Err(SubscriptionError::InvalidTransition(format!(
                "Cannot cancel from {}",
                self.status.as_str()
            )));
        }
        self.status = SubscriptionStatus::Cancelled;
        self.end_date = Some(Utc::now().date_naive());
        self.updated_at = Utc::now();

        Ok(SubscriptionEvent::Cancelled {
            subscription_id: self.id,
            customer_id: self.customer_id,
            reason: reason.map(|s| s.to_string()),
        })
    }

    /// Change plan.
    pub fn change_plan(&mut self, new_plan_id: i64) -> Result<SubscriptionEvent, SubscriptionError> {
        if self.status != SubscriptionStatus::Active {
            return Err(SubscriptionError::InvalidTransition(format!(
                "Cannot change plan from {}",
                self.status.as_str()
            )));
        }
        let old_plan_id = self.plan_id;
        self.plan_id = new_plan_id;
        self.updated_at = Utc::now();

        Ok(SubscriptionEvent::PlanChanged {
            subscription_id: self.id,
            customer_id: self.customer_id,
            old_plan_id,
            new_plan_id,
        })
    }

    /// Create creation event.
    pub fn creation_event(&self) -> EventEnvelope<SubscriptionEvent> {
        EventEnvelope::new(
            "subscription.created.v1".to_string(),
            1,
            "subscription-service".to_string(),
            SubscriptionEvent::Created {
                subscription_id: self.id,
                customer_id: self.customer_id,
                plan_id: self.plan_id,
            },
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn create_test_subscription() -> Subscription {
        Subscription::create(
            1,
            100,
            200,
            1,
            NaiveDate::from_ymd_opt(2026, 7, 1).unwrap(),
            BillingCycle::Monthly,
        )
        .unwrap()
    }

    #[test]
    fn test_create_subscription() {
        let sub = create_test_subscription();
        assert_eq!(sub.id, 1);
        assert_eq!(sub.status, SubscriptionStatus::Pending);
    }

    #[test]
    fn test_activate_subscription() {
        let mut sub = create_test_subscription();
        let event = sub.activate().unwrap();
        assert_eq!(sub.status, SubscriptionStatus::Active);
    }

    #[test]
    fn test_suspend_subscription() {
        let mut sub = create_test_subscription();
        sub.activate().unwrap();
        let event = sub.suspend(Some("Non-payment")).unwrap();
        assert_eq!(sub.status, SubscriptionStatus::Suspended);
    }

    #[test]
    fn test_cancel_subscription() {
        let mut sub = create_test_subscription();
        sub.activate().unwrap();
        let event = sub.cancel(Some("Customer request")).unwrap();
        assert_eq!(sub.status, SubscriptionStatus::Cancelled);
    }

    #[test]
    fn test_change_plan() {
        let mut sub = create_test_subscription();
        sub.activate().unwrap();
        let event = sub.change_plan(300).unwrap();
        assert_eq!(sub.plan_id, 300);
    }
}
