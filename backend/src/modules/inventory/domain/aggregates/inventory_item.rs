use crate::modules::inventory::domain::value_objects::{InventoryItemId, InventoryStatus};

/// InventoryItem aggregate root - represents a piece of hardware inventory
#[derive(Debug, Clone)]
pub struct InventoryItem {
    pub id: InventoryItemId,
    pub branch_id: i64,
    pub item_type: String,
    pub device_model_id: Option<i64>,
    pub serial_number: Option<String>,
    pub barcode: Option<String>,
    pub status: InventoryStatus,
    pub assigned_to: Option<i64>,
    pub notes: Option<String>,
}

/// Domain errors for InventoryItem aggregate
#[derive(Debug, Clone, PartialEq)]
pub enum InventoryDomainError {
    ItemNotFound(i64),
    AlreadyInstalled,
    AlreadyAssigned,
    CannotScrapActiveItem,
    InvalidStatusTransition,
}

impl std::fmt::Display for InventoryDomainError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::ItemNotFound(id) => write!(f, "Inventory item {} not found", id),
            Self::AlreadyInstalled => write!(f, "Item is already installed"),
            Self::AlreadyAssigned => write!(f, "Item is already assigned"),
            Self::CannotScrapActiveItem => write!(f, "Cannot scrap an active item"),
            Self::InvalidStatusTransition => write!(f, "Invalid status transition"),
        }
    }
}

impl std::error::Error for InventoryDomainError {}

impl InventoryItem {
    pub fn new(branch_id: i64, item_type: String, serial_number: Option<String>, barcode: Option<String>) -> Self {
        Self {
            id: InventoryItemId::new(0),
            branch_id,
            item_type,
            device_model_id: None,
            serial_number,
            barcode,
            status: InventoryStatus::InStock,
            assigned_to: None,
            notes: None,
        }
    }

    pub fn assign(&mut self, user_id: i64) -> Result<(), InventoryDomainError> {
        if self.status == InventoryStatus::Installed {
            return Err(InventoryDomainError::AlreadyInstalled);
        }
        self.status = InventoryStatus::Assigned;
        self.assigned_to = Some(user_id);
        Ok(())
    }

    pub fn install(&mut self) -> Result<(), InventoryDomainError> {
        if self.status == InventoryStatus::Installed {
            return Err(InventoryDomainError::AlreadyInstalled);
        }
        self.status = InventoryStatus::Installed;
        Ok(())
    }

    pub fn return_item(&mut self) -> Result<(), InventoryDomainError> {
        self.status = InventoryStatus::InStock;
        self.assigned_to = None;
        Ok(())
    }

    pub fn scrap(&mut self) -> Result<(), InventoryDomainError> {
        if self.status == InventoryStatus::Installed {
            return Err(InventoryDomainError::CannotScrapActiveItem);
        }
        self.status = InventoryStatus::Scrapped;
        Ok(())
    }

    pub fn is_available(&self) -> bool {
        self.status == InventoryStatus::InStock
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new_inventory_item() {
        let item = InventoryItem::new(1, "ont".to_string(), Some("SN-001".to_string()), None);
        assert_eq!(item.status, InventoryStatus::InStock);
        assert!(item.is_available());
    }

    #[test]
    fn test_assign_and_install() {
        let mut item = InventoryItem::new(1, "router".to_string(), None, None);
        item.assign(10).unwrap();
        assert_eq!(item.status, InventoryStatus::Assigned);
        item.install().unwrap();
        assert_eq!(item.status, InventoryStatus::Installed);
    }
}
