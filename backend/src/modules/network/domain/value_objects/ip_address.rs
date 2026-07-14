//! IP address value object.

use serde::{Deserialize, Serialize};
use thiserror::Error;

/// IP address value object.
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct IpAddress(String);

/// IP address errors.
#[derive(Debug, Error)]
pub enum IpAddressError {
    #[error("Invalid IP address format: {0}")]
    InvalidFormat(String),
}

impl IpAddress {
    /// Create a new IP address value object.
    pub fn new(ip: &str) -> Result<Self, IpAddressError> {
        let parts: Vec<&str> = ip.split('.').collect();
        if parts.len() != 4 {
            return Err(IpAddressError::InvalidFormat(ip.to_string()));
        }

        for part in &parts {
            if part.parse::<u8>().is_err() {
                return Err(IpAddressError::InvalidFormat(ip.to_string()));
            }
        }

        Ok(Self(ip.to_string()))
    }

    /// Get the IP address as a string.
    pub fn as_str(&self) -> &str {
        &self.0
    }

    /// Check if this is a private IP address.
    pub fn is_private(&self) -> bool {
        let parts: Vec<u8> = self.0.split('.').map(|p| p.parse().unwrap_or(0)).collect();
        if parts.len() != 4 {
            return false;
        }

        parts[0] == 10
            || (parts[0] == 172 && parts[1] >= 16 && parts[1] <= 31)
            || (parts[0] == 192 && parts[1] == 168)
    }
}

impl std::fmt::Display for IpAddress {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl TryFrom<&str> for IpAddress {
    type Error = IpAddressError;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        Self::new(value)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_valid_ip() {
        let ip = IpAddress::new("192.168.1.1").unwrap();
        assert_eq!(ip.as_str(), "192.168.1.1");
    }

    #[test]
    fn test_invalid_ip() {
        assert!(IpAddress::new("256.1.1.1").is_err());
        assert!(IpAddress::new("1.2.3").is_err());
    }

    #[test]
    fn test_private_ip() {
        assert!(IpAddress::new("192.168.1.1").unwrap().is_private());
        assert!(IpAddress::new("10.0.0.1").unwrap().is_private());
        assert!(IpAddress::new("8.8.8.8").unwrap().is_private() == false);
    }
}
