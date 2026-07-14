//! IP pool aggregate root.

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use thiserror::Error;

use crate::common::shared::events::EventEnvelope;

/// IP pool aggregate root.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct IpPool {
    pub id: i64,
    pub name: String,
    pub network_address: String,
    pub subnet_mask: String,
    pub gateway: String,
    pub vlan_id: Option<i64>,
    pub branch_id: i64,
    pub status: IpPoolStatus,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

/// IP pool status.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum IpPoolStatus {
    Active,
    Inactive,
    Exhausted,
}

impl IpPoolStatus {
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::Active => "active",
            Self::Inactive => "inactive",
            Self::Exhausted => "exhausted",
        }
    }
}

/// IP pool domain events.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum IpPoolEvent {
    Created {
        pool_id: i64,
        name: String,
        network_address: String,
    },
    IpAllocated {
        pool_id: i64,
        ip_address: String,
        allocated_to: String,
    },
    IpReleased {
        pool_id: i64,
        ip_address: String,
    },
}

/// IP pool domain errors.
#[derive(Debug, Error)]
pub enum IpPoolError {
    #[error("Invalid IP address: {0}")]
    InvalidIpAddress(String),

    #[error("Pool exhausted")]
    PoolExhausted,

    #[error("IP address already allocated: {0}")]
    AlreadyAllocated(String),

    #[error("IP address not found: {0}")]
    IpNotFound(String),

    #[error("Validation error: {0}")]
    Validation(String),

    #[error("IP pool not found")]
    NotFound,
}

impl IpPool {
    /// Create a new IP pool.
    pub fn create(
        id: i64,
        name: String,
        network_address: String,
        subnet_mask: String,
        gateway: String,
        branch_id: i64,
    ) -> Result<Self, IpPoolError> {
        if name.trim().is_empty() {
            return Err(IpPoolError::Validation("Pool name cannot be empty".to_string()));
        }

        let now = Utc::now();

        Ok(Self {
            id,
            name,
            network_address,
            subnet_mask,
            gateway,
            vlan_id: None,
            branch_id,
            status: IpPoolStatus::Active,
            created_at: now,
            updated_at: now,
        })
    }

    /// Create creation event.
    pub fn creation_event(&self) -> EventEnvelope<IpPoolEvent> {
        EventEnvelope::new(
            "network.ip_pool.created.v1".to_string(),
            1,
            "network-service".to_string(),
            IpPoolEvent::Created {
                pool_id: self.id,
                name: self.name.clone(),
                network_address: self.network_address.clone(),
            },
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_create_ip_pool() {
        let pool = IpPool::create(
            1,
            "Customer Pool".to_string(),
            "192.168.1.0".to_string(),
            "255.255.255.0".to_string(),
            "192.168.1.1".to_string(),
            1,
        )
        .unwrap();

        assert_eq!(pool.id, 1);
        assert_eq!(pool.status, IpPoolStatus::Active);
    }

    #[test]
    fn test_create_ip_pool_empty_name() {
        assert!(IpPool::create(
            1,
            "".to_string(),
            "192.168.1.0".to_string(),
            "255.255.255.0".to_string(),
            "192.168.1.1".to_string(),
            1,
        )
        .is_err());
    }
}
