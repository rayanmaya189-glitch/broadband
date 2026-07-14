//! Data retention policy value object.
//!
//! Defines how long data should be retained and when it should be deleted.

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use thiserror::Error;

/// Data retention policy value object.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DataRetentionPolicy {
    pub policy_id: String,
    pub data_type: String,
    pub retention_days: u32,
    pub delete_after_expiry: bool,
    pub notify_before_expiry_days: Option<u32>,
    pub created_at: DateTime<Utc>,
}

/// Data retention policy errors.
#[derive(Debug, Error)]
pub enum DataRetentionPolicyError {
    #[error("Invalid retention period: {0}")]
    InvalidRetentionPeriod(String),

    #[error("Validation error: {0}")]
    Validation(String),
}

impl DataRetentionPolicy {
    /// Create a new data retention policy.
    pub fn new(
        policy_id: String,
        data_type: String,
        retention_days: u32,
        delete_after_expiry: bool,
        notify_before_expiry_days: Option<u32>,
    ) -> Result<Self, DataRetentionPolicyError> {
        if retention_days == 0 {
            return Err(DataRetentionPolicyError::InvalidRetentionPeriod(
                "Retention period must be greater than 0".to_string(),
            ));
        }

        if let Some(notify_days) = notify_before_expiry_days {
            if notify_days >= retention_days {
                return Err(DataRetentionPolicyError::Validation(
                    "Notification period must be less than retention period".to_string(),
                ));
            }
        }

        Ok(Self {
            policy_id,
            data_type,
            retention_days,
            delete_after_expiry,
            notify_before_expiry_days,
            created_at: Utc::now(),
        })
    }

    /// Calculate the expiry date for data created at the given time.
    pub fn expiry_date(&self, created_at: DateTime<Utc>) -> DateTime<Utc> {
        created_at + chrono::Duration::days(self.retention_days as i64)
    }

    /// Check if data has expired.
    pub fn is_expired(&self, created_at: DateTime<Utc>) -> bool {
        Utc::now() > self.expiry_date(created_at)
    }

    /// Check if notification should be sent.
    pub fn should_notify(&self, created_at: DateTime<Utc>) -> bool {
        if let Some(notify_days) = self.notify_before_expiry_days {
            let expiry = self.expiry_date(created_at);
            let notify_at = expiry - chrono::Duration::days(notify_days as i64);
            Utc::now() >= notify_at
        } else {
            false
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_create_retention_policy() {
        let policy = DataRetentionPolicy::new(
            "policy_001".to_string(),
            "customer_data".to_string(),
            365,
            true,
            Some(30),
        ).unwrap();

        assert_eq!(policy.policy_id, "policy_001");
        assert_eq!(policy.retention_days, 365);
    }

    #[test]
    fn test_invalid_retention_period() {
        let result = DataRetentionPolicy::new(
            "policy_002".to_string(),
            "customer_data".to_string(),
            0,
            true,
            None,
        );
        assert!(matches!(result, Err(DataRetentionPolicyError::InvalidRetentionPeriod(_))));
    }

    #[test]
    fn test_expiry_date() {
        let policy = DataRetentionPolicy::new(
            "policy_003".to_string(),
            "customer_data".to_string(),
            30,
            true,
            None,
        ).unwrap();

        let created_at = Utc::now();
        let expiry = policy.expiry_date(created_at);
        let expected = created_at + chrono::Duration::days(30);
        
        // Allow 1 second difference for test execution time
        assert!((expiry - expected).num_seconds().abs() < 2);
    }

    #[test]
    fn test_is_expired() {
        let policy = DataRetentionPolicy::new(
            "policy_004".to_string(),
            "customer_data".to_string(),
            30,
            true,
            None,
        ).unwrap();

        // Data created 31 days ago should be expired
        let created_at = Utc::now() - chrono::Duration::days(31);
        assert!(policy.is_expired(created_at));

        // Data created today should not be expired
        let created_at = Utc::now();
        assert!(!policy.is_expired(created_at));
    }
}
