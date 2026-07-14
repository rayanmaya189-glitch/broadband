//! Network event types.
//!
//! These events are published when network-related actions occur
//! and are consumed by other modules (device, monitoring, notification).

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

/// Published when a device comes online.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DeviceOnlineEvent {
    pub device_id: i64,
    pub device_name: String,
    pub management_ip: String,
    pub branch_id: i64,
    pub timestamp: DateTime<Utc>,
}

/// Published when a device goes offline.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DeviceOfflineEvent {
    pub device_id: i64,
    pub device_name: String,
    pub management_ip: String,
    pub branch_id: i64,
    pub last_seen: DateTime<Utc>,
    pub timestamp: DateTime<Utc>,
}

/// Published when a VLAN is created.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VlanCreatedEvent {
    pub vlan_id: i64,
    pub vlan_number: i32,
    pub name: String,
    pub branch_id: i64,
    pub timestamp: DateTime<Utc>,
}

/// Published when an IP address is allocated.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct IpAllocatedEvent {
    pub ip_address_id: i64,
    pub ip_address: String,
    pub pool_id: i64,
    pub allocated_to_type: Option<String>,
    pub allocated_to_id: Option<i64>,
    pub timestamp: DateTime<Utc>,
}

/// Published when bandwidth is applied to a subscription.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BandwidthAppliedEvent {
    pub application_id: i64,
    pub subscription_id: i64,
    pub profile_id: i64,
    pub device_id: i64,
    pub download_kbps: i32,
    pub upload_kbps: i32,
    pub timestamp: DateTime<Utc>,
}
