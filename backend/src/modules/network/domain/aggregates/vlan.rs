use crate::modules::network::domain::value_objects::{VlanId, VlanStatus, VlanType};

/// Vlan aggregate root - represents a network VLAN configuration
#[derive(Debug, Clone, PartialEq)]
pub struct Vlan {
    pub id: VlanId,
    pub branch_id: i64,
    pub vlan_id: i32,
    pub name: String,
    pub description: Option<String>,
    pub vlan_type: VlanType,
    pub is_active: bool,
    pub status: VlanStatus,
}

/// Domain errors for Vlan aggregate
#[derive(Debug, Clone, PartialEq)]
pub enum VlanDomainError {
    InvalidVlanId,
    InvalidVlanType,
    VlanNotFound(i64),
    DuplicateVlanId(i32),
    CannotDeleteActiveVlan,
}

impl std::fmt::Display for VlanDomainError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::InvalidVlanId => write!(f, "VLAN ID must be between 1 and 4094"),
            Self::InvalidVlanType => write!(f, "Invalid VLAN type"),
            Self::VlanNotFound(id) => write!(f, "VLAN {} not found", id),
            Self::DuplicateVlanId(vid) => write!(f, "VLAN ID {} already exists", vid),
            Self::CannotDeleteActiveVlan => write!(f, "Cannot delete an active VLAN"),
        }
    }
}

impl std::error::Error for VlanDomainError {}

impl Vlan {
    pub fn new(
        branch_id: i64,
        vlan_id: i32,
        name: String,
        vlan_type: VlanType,
    ) -> Result<Self, VlanDomainError> {
        if vlan_id < 1 || vlan_id > 4094 {
            return Err(VlanDomainError::InvalidVlanId);
        }
        Ok(Self {
            id: VlanId::new(0),
            branch_id,
            vlan_id,
            name,
            description: None,
            vlan_type,
            is_active: true,
            status: VlanStatus::Active,
        })
    }

    pub fn deactivate(&mut self) -> Result<(), VlanDomainError> {
        if self.status == VlanStatus::Active && self.is_active {
            return Err(VlanDomainError::CannotDeleteActiveVlan);
        }
        self.status = VlanStatus::Inactive;
        self.is_active = false;
        Ok(())
    }

    pub fn is_available(&self) -> bool {
        self.is_active && self.status == VlanStatus::Active
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new_vlan() {
        let vlan = Vlan::new(
            1,
            200,
            "Customer Data".to_string(),
            VlanType::CustomerResidential,
        );
        assert!(vlan.is_ok());
        let vlan = vlan.unwrap();
        assert!(vlan.is_available());
    }

    #[test]
    fn test_invalid_vlan_id() {
        let vlan = Vlan::new(1, 0, "Invalid".to_string(), VlanType::CustomerResidential);
        assert_eq!(vlan, Err(VlanDomainError::InvalidVlanId));
    }

    #[test]
    fn test_invalid_vlan_id_too_high() {
        let vlan = Vlan::new(
            1,
            4095,
            "Invalid".to_string(),
            VlanType::CustomerResidential,
        );
        assert_eq!(vlan, Err(VlanDomainError::InvalidVlanId));
    }
}
