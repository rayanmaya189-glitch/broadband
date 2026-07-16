/// Coverage business rules and invariants
pub struct CoverageRules;

impl CoverageRules {
    /// Valid area types
    pub const VALID_AREA_TYPES: &[&str] = &["polygon", "circle", "pincode"];

    /// Maximum coverage radius (meters)
    pub const MAX_RADIUS_METERS: i32 = 50000;

    /// Check if area type is valid
    pub fn is_valid_area_type(area_type: &str) -> bool {
        Self::VALID_AREA_TYPES.contains(&area_type)
    }
}
