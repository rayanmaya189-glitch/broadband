//! VLAN aggregate root.

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use thiserror::Error;

use crate::common::shared::events::EventEnvelope;

/// VLAN aggregate root.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Vlan {
    pub id: i64,
    pub vlan_number: i32,
    pub name: String,
    pub branch_id: i64,
    pub status: VlanStatus,
    pub description: Option<String>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

/// VLAN status.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum VlanStatus {
    Active,
    Inactive,
    Maintenance,
}

impl VlanStatus {
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::Active => "active",
            Self::Inactive => "inactive",
            Self::Maintenance => "maintenance",
        }
    }
}

/// VLAN domain events.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum VlanEvent {
    Created {
        vlan_id: i64,
        vlan_number: i32,
        branch_id: i64,
    },
    StatusChanged {
        vlan_id: i64,
        old_status: String,
        new_status: String,
    },
}

/// VLAN domain errors.
#[derive(Debug, Error)]
pub enum VlanError {
    #[error("Invalid VLAN number: {0}")]
    InvalidVlanNumber(i32),

    #[error("VLAN already exists: {0}")]
    AlreadyExists(i32),

    #[error("Validation error: {0}")]
    Validation(String),

    #[error("VLAN not found")]
    NotFound,
}

impl Vlan {
    /// Create a new VLAN.
    pub fn create(
        id: i64,
        vlan_number: i32,
        name: String,
        branch_id: i64,
    ) -> Result<Self, VlanError> {
        if vlan_number < 1 || vlan_number > 4094 {
            return Err(VlanError::InvalidVlanNumber(vlan_number));
        }

        if name.trim().is_empty() {
            return Err(VlanError::Validation("VLAN name cannot be empty".to_string()));
        }

        let now = Utc::now();

        Ok(Self {
            id,
            vlan_number,
            name,
            branch_id,
            status: VlanStatus::Active,
            description: None,
            created_at: now,
            updated_at: now,
        })
    }

    /// Change VLAN status.
    pub fn change_status(&mut self, new_status: VlanStatus) -> Result<VlanEvent, VlanError> {
        let old_status = self.status.clone();
        self.status = new_status;
        self.updated_at = Utc::now();

        Ok(VlanEvent::StatusChanged {
            vlan_id: self.id,
            old_status: old_status.as_str().to_string(),
            new_status: self.status.as_str().to_string(),
        })
    }

    /// Create creation event.
    pub fn creation_event(&self) -> EventEnvelope<VlanEvent> {
        EventEnvelope::new(
            "network.vlan.created.v1".to_string(),
            1,
            "network-service".to_string(),
            VlanEvent::Created {
                vlan_id: self.id,
                vlan_number: self.vlan_number,
                branch_id: self.branch_id,
            },
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_create_vlan() {
        let vlan = Vlan::create(1, 100, "Management".to_string(), 1).unwrap();
        assert_eq!(vlan.id, 1);
        assert_eq!(vlan.vlan_number, 100);
        assert_eq!(vlan.status, VlanStatus::Active);
    }

    #[test]
    fn test_create_vlan_invalid_number() {
        assert!(Vlan::create(1, 0, "Test".to_string(), 1).is_err());
        assert!(Vlan::create(1, 4095, "Test".to_string(), 1).is_err());
    }

    #[test]
    fn test_change_status() {
        let mut vlan = Vlan::create(1, 100, "Test".to_string(), 1).unwrap();
        let event = vlan.change_status(VlanStatus::Inactive).unwrap();
        assert_eq!(vlan.status, VlanStatus::Inactive);
    }
}
