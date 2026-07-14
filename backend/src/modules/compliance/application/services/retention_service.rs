//! Data retention service.
//!
//! Manages data retention policies and handles data expiry.

use crate::modules::compliance::domain::value_objects::data_retention_policy::DataRetentionPolicy;

/// Data retention service.
pub struct RetentionService {
    policies: Vec<DataRetentionPolicy>,
}

impl RetentionService {
    /// Create a new retention service.
    pub fn new() -> Self {
        Self {
            policies: Vec::new(),
        }
    }

    /// Add a retention policy.
    pub fn add_policy(&mut self, policy: DataRetentionPolicy) {
        self.policies.push(policy);
    }

    /// Get retention policy for a data type.
    pub fn get_policy(&self, data_type: &str) -> Option<&DataRetentionPolicy> {
        self.policies.iter().find(|p| p.data_type == data_type)
    }

    /// Check if data should be deleted based on retention policies.
    pub fn should_delete(&self, data_type: &str, created_at: chrono::DateTime<chrono::Utc>) -> bool {
        if let Some(policy) = self.get_policy(data_type) {
            policy.is_expired(created_at)
        } else {
            false
        }
    }

    /// Get all data types that need cleanup.
    pub fn get_cleanup_candidates(&self) -> Vec<&DataRetentionPolicy> {
        self.policies
            .iter()
            .filter(|p| p.delete_after_expiry)
            .collect()
    }
}

impl Default for RetentionService {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_retention_service() {
        let mut service = RetentionService::new();
        
        let policy = DataRetentionPolicy::new(
            "policy_001".to_string(),
            "customer_data".to_string(),
            30,
            true,
            None,
        )
        .unwrap();

        service.add_policy(policy);

        assert!(service.get_policy("customer_data").is_some());
        assert!(service.get_policy("other_data").is_none());
    }

    #[test]
    fn test_should_delete() {
        let mut service = RetentionService::new();
        
        let policy = DataRetentionPolicy::new(
            "policy_002".to_string(),
            "customer_data".to_string(),
            30,
            true,
            None,
        )
        .unwrap();

        service.add_policy(policy);

        // Data created 31 days ago should be deleted
        let created_at = chrono::Utc::now() - chrono::Duration::days(31);
        assert!(service.should_delete("customer_data", created_at));

        // Data created today should not be deleted
        let created_at = chrono::Utc::now();
        assert!(!service.should_delete("customer_data", created_at));
    }
}
