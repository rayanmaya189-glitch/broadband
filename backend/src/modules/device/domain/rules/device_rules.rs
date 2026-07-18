/// Device management business rules and invariants
pub struct DeviceRules;

impl DeviceRules {
    /// Health score thresholds
    pub const HEALTH_CRITICAL: i32 = 30;
    pub const HEALTH_WARNING: i32 = 50;
    pub const HEALTH_GOOD: i32 = 70;
    pub const HEALTH_EXCELLENT: i32 = 90;

    /// SNMP polling interval (seconds)
    pub const SNMP_POLL_INTERVAL_SECS: u64 = 300;

    /// Maximum device restarts per hour
    pub const MAX_RESTARTS_PER_HOUR: u32 = 3;

    /// Firmware update timeout (minutes)
    pub const FIRMWARE_UPDATE_TIMEOUT_MINUTES: u64 = 30;

    /// Health score from SNMP metrics
    pub fn calculate_health_score(cpu: f64, memory: f64, uptime_hours: f64) -> i32 {
        let cpu_score = ((100.0 - cpu) / 100.0 * 40.0) as i32;
        let mem_score = ((100.0 - memory) / 100.0 * 30.0) as i32;
        let uptime_bonus = if uptime_hours > 24.0 {
            30
        } else {
            (uptime_hours / 24.0 * 30.0) as i32
        };
        (cpu_score + mem_score + uptime_bonus).clamp(0, 100)
    }

    /// Get health status from score
    pub fn health_status(score: i32) -> &'static str {
        match score {
            0..=29 => "critical",
            30..=49 => "warning",
            50..=69 => "fair",
            70..=89 => "good",
            _ => "excellent",
        }
    }
}
