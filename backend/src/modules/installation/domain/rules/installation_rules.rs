/// Installation business rules and invariants
pub struct InstallationRules;

impl InstallationRules {
    /// Valid installation types
    pub const VALID_TYPES: &[&str] = &["new", "relocation", "upgrade", "repair"];

    /// Default installation duration (hours)
    pub const DEFAULT_DURATION_HOURS: i32 = 4;

    /// Maximum fiber drop length (meters)
    pub const MAX_FIBER_DROP_METERS: i32 = 5000;

    /// Minimum ONU power (dBm)
    pub const MIN_ONU_POWER_DBM: f64 = -28.0;

    /// Check if installation type is valid
    pub fn is_valid_type(inst_type: &str) -> bool {
        Self::VALID_TYPES.contains(&inst_type)
    }
}
