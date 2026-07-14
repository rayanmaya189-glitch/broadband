//! Network business rules.

use crate::modules::network::domain::aggregates::vlan::vlan::VlanError;

/// Validate VLAN number.
pub fn validate_vlan_number(vlan_number: i32) -> Result<(), VlanError> {
    if vlan_number < 1 || vlan_number > 4094 {
        return Err(VlanError::InvalidVlanNumber(vlan_number));
    }
    Ok(())
}

/// Validate IP address format.
pub fn validate_ip_address(ip: &str) -> Result<(), String> {
    let parts: Vec<&str> = ip.split('.').collect();
    if parts.len() != 4 {
        return Err("Invalid IP address format".to_string());
    }

    for part in &parts {
        if part.parse::<u8>().is_err() {
            return Err(format!("Invalid IP octet: {}", part));
        }
    }

    Ok(())
}

/// Validate subnet mask.
pub fn validate_subnet_mask(mask: &str) -> Result<(), String> {
    validate_ip_address(mask)?;

    let parts: Vec<u8> = mask.split('.').map(|p| p.parse().unwrap_or(0)).collect();
    let binary: u32 = (parts[0] as u32) << 24
        | (parts[1] as u32) << 16
        | (parts[2] as u32) << 8
        | parts[3] as u32;

    // Check if it's a valid contiguous mask
    let inverted = !binary;
    if inverted & (inverted + 1) != 0 {
        return Err("Invalid subnet mask".to_string());
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_validate_vlan_number_valid() {
        assert!(validate_vlan_number(100).is_ok());
        assert!(validate_vlan_number(1).is_ok());
        assert!(validate_vlan_number(4094).is_ok());
    }

    #[test]
    fn test_validate_vlan_number_invalid() {
        assert!(validate_vlan_number(0).is_err());
        assert!(validate_vlan_number(4095).is_err());
    }

    #[test]
    fn test_validate_ip_address() {
        assert!(validate_ip_address("192.168.1.1").is_ok());
        assert!(validate_ip_address("256.1.1.1").is_err());
    }

    #[test]
    fn test_validate_subnet_mask() {
        assert!(validate_subnet_mask("255.255.255.0").is_ok());
        assert!(validate_subnet_mask("255.255.255.1").is_err());
    }
}
