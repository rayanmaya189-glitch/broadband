use serde::{Deserialize, Serialize};
use std::fmt;

/// Metric name value object
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum MetricName {
    CpuUsage,
    MemoryUsage,
    DiskUsage,
    Temperature,
    Uptime,
    RxPower,
    TxPower,
    BandwidthUsage,
    PacketLoss,
    Latency,
    DeviceHealth,
    ActiveSessions,
    ErrorRate,
}

impl MetricName {
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::CpuUsage => "cpu_usage",
            Self::MemoryUsage => "memory_usage",
            Self::DiskUsage => "disk_usage",
            Self::Temperature => "temperature",
            Self::Uptime => "uptime",
            Self::RxPower => "rx_power",
            Self::TxPower => "tx_power",
            Self::BandwidthUsage => "bandwidth_usage",
            Self::PacketLoss => "packet_loss",
            Self::Latency => "latency",
            Self::DeviceHealth => "device_health",
            Self::ActiveSessions => "active_sessions",
            Self::ErrorRate => "error_rate",
        }
    }

    pub fn unit(&self) -> &'static str {
        match self {
            Self::CpuUsage | Self::MemoryUsage | Self::DiskUsage | Self::PacketLoss => "%",
            Self::Temperature => "°C",
            Self::Uptime => "seconds",
            Self::RxPower | Self::TxPower => "dBm",
            Self::BandwidthUsage => "bps",
            Self::Latency => "ms",
            Self::DeviceHealth => "score",
            Self::ActiveSessions => "count",
            Self::ErrorRate => "per_second",
        }
    }

    pub fn from_str(s: &str) -> Option<Self> {
        match s {
            "cpu_usage" => Some(Self::CpuUsage),
            "memory_usage" => Some(Self::MemoryUsage),
            "disk_usage" => Some(Self::DiskUsage),
            "temperature" => Some(Self::Temperature),
            "uptime" => Some(Self::Uptime),
            "rx_power" => Some(Self::RxPower),
            "tx_power" => Some(Self::TxPower),
            "bandwidth_usage" => Some(Self::BandwidthUsage),
            "packet_loss" => Some(Self::PacketLoss),
            "latency" => Some(Self::Latency),
            "device_health" => Some(Self::DeviceHealth),
            "active_sessions" => Some(Self::ActiveSessions),
            "error_rate" => Some(Self::ErrorRate),
            _ => None,
        }
    }
}

impl fmt::Display for MetricName {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.as_str())
    }
}
