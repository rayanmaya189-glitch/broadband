use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

/// Device metrics aggregate root - represents a point-in-time snapshot of device health
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DeviceMetrics {
    pub id: i64,
    pub device_id: i64,
    pub branch_id: i64,
    pub cpu_usage_percent: Option<f64>,
    pub memory_usage_percent: Option<f64>,
    pub memory_total_bytes: Option<i64>,
    pub memory_free_bytes: Option<i64>,
    pub disk_usage_percent: Option<f64>,
    pub disk_total_bytes: Option<i64>,
    pub disk_free_bytes: Option<i64>,
    pub uptime_seconds: Option<i64>,
    pub temperature_celsius: Option<f64>,
    pub rx_power_dbm: Option<f64>,
    pub tx_power_dbm: Option<f64>,
    pub bandwidth_usage_bps: Option<i64>,
    pub packet_loss_percent: Option<f64>,
    pub latency_ms: Option<f64>,
    pub health_score: i32,
    pub raw_metrics: Option<serde_json::Value>,
    pub recorded_at: DateTime<Utc>,
    pub created_at: DateTime<Utc>,
}

impl DeviceMetrics {
    /// Create a new device metrics snapshot
    pub fn new(
        device_id: i64,
        branch_id: i64,
        health_score: i32,
    ) -> Self {
        let now = Utc::now();
        Self {
            id: 0,
            device_id,
            branch_id,
            cpu_usage_percent: None,
            memory_usage_percent: None,
            memory_total_bytes: None,
            memory_free_bytes: None,
            disk_usage_percent: None,
            disk_total_bytes: None,
            disk_free_bytes: None,
            uptime_seconds: None,
            temperature_celsius: None,
            rx_power_dbm: None,
            tx_power_dbm: None,
            bandwidth_usage_bps: None,
            packet_loss_percent: None,
            latency_ms: None,
            health_score,
            raw_metrics: None,
            recorded_at: now,
            created_at: now,
        }
    }

    /// Check if CPU is in critical state (>90%)
    pub fn is_cpu_critical(&self) -> bool {
        self.cpu_usage_percent.map_or(false, |cpu| cpu > 90.0)
    }

    /// Check if CPU is in warning state (>70%)
    pub fn is_cpu_warning(&self) -> bool {
        self.cpu_usage_percent.map_or(false, |cpu| cpu > 70.0)
    }

    /// Check if memory is in critical state (>90%)
    pub fn is_memory_critical(&self) -> bool {
        self.memory_usage_percent.map_or(false, |mem| mem > 90.0)
    }

    /// Check if memory is in warning state (>80%)
    pub fn is_memory_warning(&self) -> bool {
        self.memory_usage_percent.map_or(false, |mem| mem > 80.0)
    }

    /// Check if temperature is critical (>70°C)
    pub fn is_temperature_critical(&self) -> bool {
        self.temperature_celsius.map_or(false, |temp| temp > 70.0)
    }

    /// Check if temperature is warning (>60°C)
    pub fn is_temperature_warning(&self) -> bool {
        self.temperature_celsius.map_or(false, |temp| temp > 60.0)
    }

    /// Check if optical power is critically low (<-28 dBm)
    pub fn is_optical_power_critical(&self) -> bool {
        self.rx_power_dbm.map_or(false, |power| power < -28.0)
    }

    /// Check if optical power is low (<-25 dBm)
    pub fn is_optical_power_warning(&self) -> bool {
        self.rx_power_dbm.map_or(false, |power| power < -25.0)
    }

    /// Check if packet loss is critical (>5%)
    pub fn is_packet_loss_critical(&self) -> bool {
        self.packet_loss_percent.map_or(false, |loss| loss > 5.0)
    }

    /// Check if packet loss is warning (>1%)
    pub fn is_packet_loss_warning(&self) -> bool {
        self.packet_loss_percent.map_or(false, |loss| loss > 1.0)
    }

    /// Calculate health score based on metrics
    pub fn calculate_health_score(&self) -> i32 {
        let mut score = 100i32;

        // CPU penalty (0-25 points)
        if let Some(cpu) = self.cpu_usage_percent {
            if cpu > 90.0 {
                score -= 25;
            } else if cpu > 70.0 {
                score -= 15;
            } else if cpu > 50.0 {
                score -= 5;
            }
        }

        // Memory penalty (0-25 points)
        if let Some(memory) = self.memory_usage_percent {
            if memory > 90.0 {
                score -= 25;
            } else if memory > 80.0 {
                score -= 15;
            } else if memory > 60.0 {
                score -= 5;
            }
        }

        // Temperature penalty (0-20 points)
        if let Some(temp) = self.temperature_celsius {
            if temp > 70.0 {
                score -= 20;
            } else if temp > 60.0 {
                score -= 10;
            } else if temp > 50.0 {
                score -= 5;
            }
        }

        // Optical power penalty (0-15 points) - for fiber devices
        if let Some(power) = self.rx_power_dbm {
            if power < -28.0 {
                score -= 15;
            } else if power < -25.0 {
                score -= 10;
            } else if power < -20.0 {
                score -= 5;
            }
        }

        // Packet loss penalty (0-15 points)
        if let Some(loss) = self.packet_loss_percent {
            if loss > 5.0 {
                score -= 15;
            } else if loss > 1.0 {
                score -= 10;
            } else if loss > 0.5 {
                score -= 5;
            }
        }

        score.max(0)
    }
}
