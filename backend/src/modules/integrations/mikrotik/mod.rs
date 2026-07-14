//! Mikrotik integration adapter.
//!
//! Adapter for Mikrotik RouterOS API integration.

use serde::{Deserialize, Serialize};

/// Mikrotik device connection configuration.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MikrotikConfig {
    pub host: String,
    pub port: u16,
    pub username: String,
    pub password: String,
    pub use_ssl: bool,
}

/// Mikrotik adapter trait.
#[async_trait::async_trait]
pub trait MikrotikAdapter: Send + Sync {
    /// Get device info from Mikrotik router.
    async fn get_device_info(&self) -> Result<DeviceResponse, MikrotikError>;

    /// Add a firewall rule.
    async fn add_firewall_rule(&self, rule: &FirewallRule) -> Result<(), MikrotikError>;

    /// Remove a firewall rule.
    async fn remove_firewall_rule(&self, rule_id: &str) -> Result<(), MikrotikError>;

    /// Get active PPPoE sessions.
    async fn get_pppoe_sessions(&self) -> Result<Vec<PppoeSession>, MikrotikError>;
}

/// Device response from Mikrotik.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DeviceResponse {
    pub identity: String,
    pub version: String,
    pub board: String,
    pub uptime: String,
}

/// Firewall rule for Mikrotik.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FirewallRule {
    pub chain: String,
    pub src_address: Option<String>,
    pub dst_address: Option<String>,
    pub action: String,
    pub comment: Option<String>,
}

/// PPPoE session info.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PppoeSession {
    pub name: String,
    pub service: String,
    pub caller_id: String,
    pub address: String,
    pub uptime: String,
}

/// Mikrotik adapter errors.
#[derive(Debug, Clone)]
pub enum MikrotikError {
    ConnectionFailed(String),
    AuthenticationFailed,
    CommandFailed(String),
    NetworkError(String),
}

impl std::fmt::Display for MikrotikError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            MikrotikError::ConnectionFailed(msg) => write!(f, "Connection failed: {}", msg),
            MikrotikError::AuthenticationFailed => write!(f, "Authentication failed"),
            MikrotikError::CommandFailed(msg) => write!(f, "Command failed: {}", msg),
            MikrotikError::NetworkError(msg) => write!(f, "Network error: {}", msg),
        }
    }
}

impl std::error::Error for MikrotikError {}
