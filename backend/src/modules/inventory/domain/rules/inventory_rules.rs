/// Inventory business rules and invariants
pub struct InventoryRules;

impl InventoryRules {
    /// Valid item types
    pub const VALID_ITEM_TYPES: &[&str] = &[
        "ont", "router", "switch", "cable", "sfp", "splitter", "pole",
    ];

    /// Check if item type is valid
    pub fn is_valid_item_type(item_type: &str) -> bool {
        Self::VALID_ITEM_TYPES.contains(&item_type)
    }

    /// Low stock threshold
    pub const LOW_STOCK_THRESHOLD: i32 = 10;

    /// Warranty period (days)
    pub const DEFAULT_WARRANTY_DAYS: i32 = 365;
}
