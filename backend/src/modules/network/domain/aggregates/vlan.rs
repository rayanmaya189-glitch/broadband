use crate::modules::network::domain::value_objects::{VlanId, VlanStatus, VlanType};

/// VLAN aggregate root - represents a virtual LAN segment
#[derive(Debug, Clone)]
pub struct Vlan {
    pub id: VlanId,
    pub branch_id: i64,
    pub vlan_tag: i32,
    pub name: String,
    pub description: Option<String>,
    pub vlan_type: VlanType,
    pub is_active: bool,
    pub status: VlanStatus,
}

/// Domain errors for VLAN aggregate
#[derive(Debug, Clone, PartialEq)]
pub enum VlanDomainError {
    InvalidVlanTag,
    VlanNotFound(i64),
    DuplicateVlanTag(i64),
    CannotDeleteActiveVlan,
}

impl std::fmt::Display for VlanDomainError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::InvalidVlanTag => write!(f, "VLAN tag must be between 1 and 4094"),
            Self::VlanNotFound(id) => write!(f, "VLAN {} not found", id),
            Self::DuplicateVlanTag(tag) => write!(f, "VLAN tag {} already exists in this branch", tag),
            Self::CannotDeleteActiveVlan => write!(f, "Cannot delete VLAN with active sessions"),
        }
    }
}

impl std::error::Error for VlanDomainError {}

impl Vlan {
    pub fn new(branch_id: i64, vlan_tag: i32, name: String, vlan_type: VlanType) -> Result<Self, VlanDomainError> {
        if !(1..=4094).contains(&vlan_tag) {
            return Err(VlanDomainError::InvalidVlanTag);
        }
        Ok(Self {
            id: VlanId::new(0),
            branch_id,
            vlan_tag,
            name,
            description: None,
            vlan_type,
            is_active: true,
            status: VlanStatus::Active,
        })
    }

    pub fn deactivate(&mut self) -> Result<(), VlanDomainError> {
        if self.status == VlanStatus::Active {
            return Err(VlanDomainError::CannotDeleteActiveVlan);
        }
        self.status = VlanStatus::Inactive;
        self.is_active = false;
        Ok(())
    }

    pub fn is_management(&self) -> bool {
        matches!(self.vlan_type, VlanType::Management)
    }

    pub fn is_customer(&self) -> bool {
        matches!(self.vlan_type, VlanType::CustomerResidential | VlanType::CustomerBusiness)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new_vlan() {
        let vlan = Vlan::new(1, 200, "Customer VLAN".to_string(), VlanType::CustomerResidential);
        assert!(vlan.is_ok());
        let vlan = vlan.unwrap();
        assert!(vlan.is_customer());
        assert!(!vlan.is_management());
    }

    #[test]
    fn test_invalid_vlan_tag() {
        let vlan = Vlan::new(1, 0, "Invalid".to_string(), VlanType::Management);
        assert_eq!(vlan, Err(VlanDomainError::InvalidVlanTag));
        let vlan = Vlan::new(1, 4095, "Invalid".to_string(), VlanType::Management);
        assert_eq!(vlan, Err(VlanDomainError::InvalidVlanTag));
    }
}
