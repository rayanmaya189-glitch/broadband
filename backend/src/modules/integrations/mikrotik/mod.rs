//! MikroTik RouterOS Integration Adapter
//!
//! Supports RouterOS v7 REST API for:
//! - Simple Queue management (bandwidth limits per customer)
//! - Queue Tree management (QoS policies)
//! - DHCP lease management
//! - PPPoE session management
//! - Firewall rules
//! - Device health monitoring

pub mod adapter;

pub use adapter::{MikrotikAdapter, MikrotikConfig, QueueConfig, DeviceStatus};
