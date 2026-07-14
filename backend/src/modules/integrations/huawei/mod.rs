//! Huawei integration adapter.
//!
//! Adapter for Huawei OLT/ONU management integration.

use serde::{Deserialize, Serialize};

/// Huawei device configuration.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HuaweiConfig {
    pub host: String,
    pub port: u16,
    pub username: String,
    pub password: String,
    pub device_type: HuaweiDeviceType,
}

/// Huawei device types.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum HuaweiDeviceType {
    Olt,
    Onu,
}

/// Huawei adapter trait.
#[async_trait::async_trait]
pub trait HuaweiAdapter: Send + Sync {
    /// Get OLT device status.
    async fn get_olt_status(&self) -> Result<OltStatus, HuaweiError>;

    /// Get all ONUs connected to OLT.
    async fn get_onu_list(&self) -> Result<Vec<OnuInfo>, HuaweiError>;

    /// Get ONU status by SN.
    async fn get_onu_status(&self, sn: &str) -> Result<OnuInfo, HuaweiError>;
}

/// OLT status response.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OltStatus {
    pub device_name: String,
    pub model: String,
    pub version: String,
    pub board_temperature: f64,
    pub cpu_usage: f64,
    pub memory_usage: f64,
}

/// ONU information.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OnuInfo {
    pub sn: String,
    pub name: String,
    pub port: String,
    pub status: String,
    pub rx_power: Option<f64>,
    pub tx_power: Option<f64>,
}

/// Huawei adapter errors.
#[derive(Debug, Clone)]
pub enum HuaweiError {
    ConnectionFailed(String),
    AuthenticationFailed,
    DeviceNotFound(String),
    CommandFailed(String),
}

impl std::fmt::Display for HuaweiError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            HuaweiError::ConnectionFailed(msg) => write!(f, "Connection failed: {}", msg),
            HuaweiError::AuthenticationFailed => write!(f, "Authentication failed"),
            HuaweiError::DeviceNotFound(msg) => write!(f, "Device not found: {}", msg),
            HuaweiError::CommandFailed(msg) => write!(f, "Command failed: {}", msg),
        }
    }
}

impl std::error::Error for HuaweiError {}
