/// Network management business rules and invariants
pub struct NetworkRules;

impl NetworkRules {
    /// Valid VLAN ID range
    pub const MIN_VLAN_TAG: i32 = 1;
    pub const MAX_VLAN_TAG: i32 = 4094;

    /// Management VLAN range
    pub const MANAGEMENT_VLAN_START: i32 = 100;
    pub const MANAGEMENT_VLAN_END: i32 = 199;

    /// Customer residential VLAN range
    pub const CUSTOMER_RESIDENTIAL_VLAN_START: i32 = 200;
    pub const CUSTOMER_RESIDENTIAL_VLAN_END: i32 = 299;

    /// Customer business VLAN range
    pub const CUSTOMER_BUSINESS_VLAN_START: i32 = 300;
    pub const CUSTOMER_BUSINESS_VLAN_END: i32 = 399;

    /// Maximum PPPoE sessions per customer
    pub const MAX_PPPOE_SESSIONS_PER_CUSTOMER: usize = 3;

    /// IP pool warning threshold (80%)
    pub const IP_POOL_WARNING_THRESHOLD: f64 = 80.0;

    /// IP pool critical threshold (95%)
    pub const IP_POOL_CRITICAL_THRESHOLD: f64 = 95.0;

    /// Check if VLAN tag is valid
    pub fn is_valid_vlan_tag(tag: i32) -> bool {
        tag >= Self::MIN_VLAN_TAG && tag <= Self::MAX_VLAN_TAG
    }

    /// Check if VLAN tag is in management range
    pub fn is_management_vlan(tag: i32) -> bool {
        tag >= Self::MANAGEMENT_VLAN_START && tag <= Self::MANAGEMENT_VLAN_END
    }

    /// Get VLAN type from tag range
    pub fn vlan_type_from_tag(tag: i32) -> &'static str {
        match tag {
            100..=199 => "management",
            200..=299 => "customer_residential",
            300..=399 => "customer_business",
            400..=499 => "iptv",
            500..=599 => "voip",
            900..=999 => "monitoring",
            _ => "unknown",
        }
    }

    /// Check if IP pool utilization is concerning
    pub fn pool_utilization_status(allocated: i64, total: i64) -> &'static str {
        if total == 0 {
            return "empty";
        }
        let pct = (allocated as f64 / total as f64) * 100.0;
        if pct >= Self::IP_POOL_CRITICAL_THRESHOLD {
            "critical"
        } else if pct >= Self::IP_POOL_WARNING_THRESHOLD {
            "warning"
        } else {
            "healthy"
        }
    }
}
