use crate::modules::device::domain::value_objects::{DeviceId, DeviceStatus, DeviceType};

/// NetworkDevice aggregate root - represents a managed network device
#[derive(Debug, Clone)]
pub struct NetworkDevice {
    pub id: DeviceId,
    pub branch_id: i64,
    pub name: String,
    pub device_model_id: i64,
    pub serial_number: String,
    pub management_ip: String,
    pub firmware_version: Option<String>,
    pub status: DeviceStatus,
    pub health_score: i32,
    pub device_type: DeviceType,
}

/// Domain errors for NetworkDevice aggregate
#[derive(Debug, Clone, PartialEq)]
pub enum DeviceDomainError {
    DeviceNotFound(i64),
    DuplicateSerialNumber(String),
    InvalidManagementIp,
    CannotDecommissionOnlineDevice,
    FirmwareUpdateAlreadyInProgress,
}

impl std::fmt::Display for DeviceDomainError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::DeviceNotFound(id) => write!(f, "Device {} not found", id),
            Self::DuplicateSerialNumber(sn) => write!(f, "Serial number '{}' already exists", sn),
            Self::InvalidManagementIp => write!(f, "Invalid management IP address"),
            Self::CannotDecommissionOnlineDevice => write!(f, "Cannot decommission an online device"),
            Self::FirmwareUpdateAlreadyInProgress => write!(f, "Firmware update already in progress"),
        }
    }
}

impl std::error::Error for DeviceDomainError {}

impl NetworkDevice {
    pub fn new(
        branch_id: i64,
        name: String,
        device_model_id: i64,
        serial_number: String,
        management_ip: String,
        device_type: DeviceType,
    ) -> Result<Self, DeviceDomainError> {
        if !Self::is_valid_ip(&management_ip) {
            return Err(DeviceDomainError::InvalidManagementIp);
        }
        Ok(Self {
            id: DeviceId::new(0),
            branch_id,
            name,
            device_model_id,
            serial_number,
            management_ip,
            firmware_version: None,
            status: DeviceStatus::Offline,
            health_score: 0,
            device_type,
        })
    }

    fn is_valid_ip(ip: &str) -> bool {
        ip.parse::<std::net::Ipv4Addr>().is_ok() || ip.parse::<std::net::Ipv6Addr>().is_ok()
    }

    pub fn go_online(&mut self) {
        self.status = DeviceStatus::Online;
        self.health_score = 100;
    }

    pub fn go_offline(&mut self) {
        self.status = DeviceStatus::Offline;
        self.health_score = 0;
    }

    pub fn set_degraded(&mut self, health_score: i32) {
        self.status = DeviceStatus::Degraded;
        self.health_score = health_score.clamp(0, 100);
    }

    pub fn decommission(&mut self) -> Result<(), DeviceDomainError> {
        if self.status == DeviceStatus::Online {
            return Err(DeviceDomainError::CannotDecommissionOnlineDevice);
        }
        self.status = DeviceStatus::Decommissioned;
        Ok(())
    }

    pub fn is_online(&self) -> bool {
        self.status == DeviceStatus::Online
    }

    pub fn needs_attention(&self) -> bool {
        self.health_score < 50 || self.status == DeviceStatus::Degraded
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new_device() {
        let device = NetworkDevice::new(
            1, "OLT-01".to_string(), 1, "SN-001".to_string(),
            "10.0.1.1".to_string(), DeviceType::Olt,
        );
        assert!(device.is_ok());
        let device = device.unwrap();
        assert_eq!(device.status, DeviceStatus::Offline);
    }

    #[test]
    fn test_invalid_ip() {
        let device = NetworkDevice::new(
            1, "OLT-01".to_string(), 1, "SN-001".to_string(),
            "invalid-ip".to_string(), DeviceType::Olt,
        );
        assert_eq!(device, Err(DeviceDomainError::InvalidManagementIp));
    }

    #[test]
    fn test_device_lifecycle() {
        let mut device = NetworkDevice::new(
            1, "OLT-01".to_string(), 1, "SN-001".to_string(),
            "10.0.1.1".to_string(), DeviceType::Olt,
        ).unwrap();
        device.go_online();
        assert!(device.is_online());
        device.set_degraded(30);
        assert!(device.needs_attention());
        device.go_offline();
        assert!(!device.is_online());
    }
}
