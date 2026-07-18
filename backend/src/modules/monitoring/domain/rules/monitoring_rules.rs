/// Monitoring business rules and constants.
///
/// Health score thresholds
pub const HEALTH_SCORE_CRITICAL: i32 = 30;
pub const HEALTH_SCORE_WARNING: i32 = 60;
pub const HEALTH_SCORE_GOOD: i32 = 80;

/// Metric thresholds for alerts
pub const CPU_CRITICAL_THRESHOLD: f64 = 90.0;
pub const CPU_WARNING_THRESHOLD: f64 = 70.0;

pub const MEMORY_CRITICAL_THRESHOLD: f64 = 90.0;
pub const MEMORY_WARNING_THRESHOLD: f64 = 80.0;

pub const TEMPERATURE_CRITICAL_THRESHOLD: f64 = 70.0;
pub const TEMPERATURE_WARNING_THRESHOLD: f64 = 60.0;

pub const OPTICAL_POWER_CRITICAL_THRESHOLD: f64 = -28.0;
pub const OPTICAL_POWER_WARNING_THRESHOLD: f64 = -25.0;

pub const PACKET_LOSS_CRITICAL_THRESHOLD: f64 = 5.0;
pub const PACKET_LOSS_WARNING_THRESHOLD: f64 = 1.0;

pub const LATENCY_WARNING_THRESHOLD: f64 = 20.0;
pub const LATENCY_CRITICAL_THRESHOLD: f64 = 50.0;

/// Alert configuration
pub const MAX_ALERTS_PER_DEVICE: usize = 100;
pub const ALERT_EXPIRY_HOURS: i64 = 24;
pub const DEFAULT_COOLDOWN_SECONDS: i32 = 300; // 5 minutes

/// Metric collection intervals
pub const DEFAULT_COLLECTION_INTERVAL_SECONDS: u64 = 60;
pub const CRITICAL_DEVICE_INTERVAL_SECONDS: u64 = 30;

/// Health status mapping
pub fn health_status_from_score(score: i32) -> &'static str {
    if score >= HEALTH_SCORE_GOOD {
        "healthy"
    } else if score >= HEALTH_SCORE_WARNING {
        "degraded"
    } else if score >= HEALTH_SCORE_CRITICAL {
        "critical"
    } else {
        "offline"
    }
}

use crate::modules::monitoring::domain::value_objects::AlertSeverity;

/// Determine alert severity based on metric value and thresholds
pub fn determine_severity(metric_name: &str, value: f64) -> AlertSeverity {
    match metric_name {
        "cpu_usage" => {
            if value >= CPU_CRITICAL_THRESHOLD {
                AlertSeverity::Critical
            } else if value >= CPU_WARNING_THRESHOLD {
                AlertSeverity::High
            } else {
                AlertSeverity::Low
            }
        }
        "memory_usage" => {
            if value >= MEMORY_CRITICAL_THRESHOLD {
                AlertSeverity::Critical
            } else if value >= MEMORY_WARNING_THRESHOLD {
                AlertSeverity::High
            } else {
                AlertSeverity::Low
            }
        }
        "temperature" => {
            if value >= TEMPERATURE_CRITICAL_THRESHOLD {
                AlertSeverity::Critical
            } else if value >= TEMPERATURE_WARNING_THRESHOLD {
                AlertSeverity::Medium
            } else {
                AlertSeverity::Low
            }
        }
        "rx_power" => {
            if value <= OPTICAL_POWER_CRITICAL_THRESHOLD {
                AlertSeverity::Critical
            } else if value <= OPTICAL_POWER_WARNING_THRESHOLD {
                AlertSeverity::High
            } else {
                AlertSeverity::Low
            }
        }
        "packet_loss" => {
            if value >= PACKET_LOSS_CRITICAL_THRESHOLD {
                AlertSeverity::Critical
            } else if value >= PACKET_LOSS_WARNING_THRESHOLD {
                AlertSeverity::Medium
            } else {
                AlertSeverity::Low
            }
        }
        "latency" => {
            if value >= LATENCY_CRITICAL_THRESHOLD {
                AlertSeverity::High
            } else if value >= LATENCY_WARNING_THRESHOLD {
                AlertSeverity::Medium
            } else {
                AlertSeverity::Low
            }
        }
        _ => AlertSeverity::Medium,
    }
}

/// Check if metric requires alert
pub fn requires_alert(metric_name: &str, value: f64) -> bool {
    match metric_name {
        "cpu_usage" => value >= CPU_WARNING_THRESHOLD,
        "memory_usage" => value >= MEMORY_WARNING_THRESHOLD,
        "temperature" => value >= TEMPERATURE_WARNING_THRESHOLD,
        "rx_power" => value <= OPTICAL_POWER_WARNING_THRESHOLD,
        "packet_loss" => value >= PACKET_LOSS_WARNING_THRESHOLD,
        "latency" => value >= LATENCY_WARNING_THRESHOLD,
        _ => false,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_health_status_mapping() {
        assert_eq!(health_status_from_score(95), "healthy");
        assert_eq!(health_status_from_score(70), "degraded");
        assert_eq!(health_status_from_score(40), "critical");
        assert_eq!(health_status_from_score(10), "offline");
    }

    #[test]
    fn test_determine_severity() {
        assert_eq!(
            determine_severity("cpu_usage", 95.0),
            AlertSeverity::Critical
        );
        assert_eq!(determine_severity("cpu_usage", 75.0), AlertSeverity::High);
        assert_eq!(
            determine_severity("temperature", 65.0),
            AlertSeverity::Medium
        );
    }

    #[test]
    fn test_requires_alert() {
        assert!(requires_alert("cpu_usage", 75.0));
        assert!(!requires_alert("cpu_usage", 50.0));
        assert!(requires_alert("rx_power", -26.0));
        assert!(!requires_alert("rx_power", -20.0));
    }
}
