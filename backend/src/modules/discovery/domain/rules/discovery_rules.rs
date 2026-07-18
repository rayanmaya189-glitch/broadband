/// Discovery business rules and invariants
pub struct DiscoveryRules;

impl DiscoveryRules {
    /// Minimum scan interval (60 seconds)
    pub const MIN_SCAN_INTERVAL: i32 = 60;
    /// Maximum scan interval (86400 seconds = 24 hours)
    pub const MAX_SCAN_INTERVAL: i32 = 86400;
    /// Vendor enterprise IDs
    pub const HUAWEI_ENTERPRISE_ID: i32 = 2011;
    pub const MIKROTIK_ENTERPRISE_ID: i32 = 14988;
    pub const ZTE_ENTERPRISE_ID: i32 = 4881;

    pub fn is_valid_interval(seconds: i32) -> bool {
        (Self::MIN_SCAN_INTERVAL..=Self::MAX_SCAN_INTERVAL).contains(&seconds)
    }

    pub fn vendor_from_enterprise_id(id: i32) -> &'static str {
        match id {
            2011 => "Huawei",
            14988 => "MikroTik",
            4881 => "ZTE",
            9 => "Cisco",
            4370 => "TP-Link",
            13014 => "Ubiquiti",
            _ => "Unknown",
        }
    }
}
