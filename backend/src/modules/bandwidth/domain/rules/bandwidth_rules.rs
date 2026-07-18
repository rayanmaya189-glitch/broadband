/// Bandwidth management business rules and invariants
pub struct BandwidthRules;

impl BandwidthRules {
    /// Maximum download speed (10 Gbps in kbps)
    pub const MAX_DOWNLOAD_KBPS: i32 = 10_000_000;

    /// Maximum upload speed (10 Gbps in kbps)
    pub const MAX_UPLOAD_KBPS: i32 = 10_000_000;

    /// Maximum burst duration (seconds)
    pub const MAX_BURST_DURATION: i32 = 300;

    /// Priority levels (1 = highest, 8 = lowest)
    pub const MIN_PRIORITY: i32 = 1;
    pub const MAX_PRIORITY: i32 = 8;

    /// Check if speed values are valid
    pub fn is_valid_speed(download_kbps: i32, upload_kbps: i32) -> bool {
        download_kbps > 0
            && upload_kbps > 0
            && download_kbps <= Self::MAX_DOWNLOAD_KBPS
            && upload_kbps <= Self::MAX_UPLOAD_KBPS
    }

    /// Check if burst configuration is valid
    pub fn is_valid_burst(sustained_kbps: i32, burst_kbps: i32, duration_seconds: i32) -> bool {
        burst_kbps >= sustained_kbps
            && duration_seconds > 0
            && duration_seconds <= Self::MAX_BURST_DURATION
    }

    /// Check if priority is valid
    pub fn is_valid_priority(priority: i32) -> bool {
        (Self::MIN_PRIORITY..=Self::MAX_PRIORITY).contains(&priority)
    }
}
