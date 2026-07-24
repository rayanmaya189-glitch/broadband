//! Device Adapter Factory
//!
//! Creates the appropriate network device adapter based on device type.
//! Supports MikroTik routers and Huawei OLT devices.

use std::sync::Arc;

pub use crate::modules::device::domain::value_objects::DeviceType;
use crate::modules::integrations::huawei::adapter::{
    HuaweiOltAdapter, HuaweiOltConfig, HuaweiOltSshAdapter,
};
use crate::modules::integrations::mikrotik::adapter::{
    MikrotikAdapter, MikrotikConfig, MikrotikDeviceAdapter,
};
use crate::modules::integrations::radius::adapter::{RadiusAdapter, RadiusClient};
use crate::modules::integrations::sms::msg91::Msg91Adapter;
use crate::modules::integrations::sms::twilio::TwilioSmsAdapter;
use crate::modules::integrations::sms::SmsProvider;
use crate::shared::errors::AppError;

/// Device adapter factory for creating adapters based on device type
pub struct DeviceAdapterFactory;

impl DeviceAdapterFactory {
    /// Create a MikroTik adapter for a specific device
    pub fn create_mikrotik(
        host: &str,
        port: u16,
        username: &str,
        password: &str,
    ) -> Arc<dyn MikrotikDeviceAdapter> {
        let config = MikrotikConfig {
            host: host.to_string(),
            port,
            username: username.to_string(),
            password: password.to_string(),
            use_ssl: true,
            accept_invalid_certs: false,
            api_version: "v7".to_string(),
        };
        Arc::new(MikrotikAdapter::new(config))
    }

    /// Create a Huawei OLT adapter for a specific device
    pub fn create_huawei_olt(
        host: &str,
        port: u16,
        username: &str,
        password: &str,
    ) -> Arc<dyn HuaweiOltAdapter> {
        let config = HuaweiOltConfig {
            host: host.to_string(),
            port,
            username: username.to_string(),
            password: password.to_string(),
            enable_password: None,
            frame_id: None,
            slot_id: None,
            pon_id: None,
            ont_id: None,
            ssh_timeout_secs: 30,
        };
        Arc::new(HuaweiOltSshAdapter::new(config))
    }

    /// Create a RADIUS client from environment
    pub fn create_radius() -> Arc<dyn RadiusClient> {
        Arc::new(RadiusAdapter::from_env())
    }

    /// Create an SMS provider based on provider name
    pub fn create_sms_provider(provider: &str) -> Arc<dyn SmsProvider> {
        match provider.to_lowercase().as_str() {
            "msg91" => Arc::new(Msg91Adapter::from_env()),
            "twilio" => Arc::new(TwilioSmsAdapter::from_env()),
            _ => Arc::new(Msg91Adapter::from_env()), // Default to MSG91 for India
        }
    }

    /// Create the appropriate network device adapter based on device type
    pub fn create_for_device(
        device_type: &DeviceType,
        management_ip: &str,
    ) -> Option<Arc<dyn NetworkDeviceAdapter>> {
        match device_type {
            DeviceType::Router | DeviceType::Switch | DeviceType::AccessPoint => {
                // Use MikroTik adapter for routers, switches, and APs
                let username =
                    std::env::var("MIKROTIK_USERNAME").unwrap_or_else(|_| "admin".to_string());
                let password = std::env::var("MIKROTIK_PASSWORD").unwrap_or_default();
                let port: u16 = std::env::var("MIKROTIK_PORT")
                    .unwrap_or_else(|_| "443".to_string())
                    .parse()
                    .unwrap_or(443);

                let adapter = Self::create_mikrotik(management_ip, port, &username, &password);
                Some(Arc::new(MikrotikNetworkAdapter(adapter)))
            }
            DeviceType::Olt | DeviceType::Ont => {
                // Use Huawei OLT adapter for OLT and ONT devices
                let username =
                    std::env::var("HUAWEI_OLT_USERNAME").unwrap_or_else(|_| "root".to_string());
                let password = std::env::var("HUAWEI_OLT_PASSWORD").unwrap_or_default();
                let port: u16 = std::env::var("HUAWEI_OLT_PORT")
                    .unwrap_or_else(|_| "22".to_string())
                    .parse()
                    .unwrap_or(22);

                let adapter = Self::create_huawei_olt(management_ip, port, &username, &password);
                Some(Arc::new(HuaweiNetworkAdapter(adapter)))
            }
        }
    }
}

/// Unified network device adapter trait for worker use
#[async_trait::async_trait]
pub trait NetworkDeviceAdapter: Send + Sync {
    /// Get device health status (0-100)
    async fn get_health_score(&self) -> Result<i32, AppError>;

    /// Apply bandwidth configuration
    async fn apply_bandwidth(
        &self,
        queue_name: &str,
        target: &str,
        download_kbps: u32,
        upload_kbps: u32,
    ) -> Result<(), AppError>;

    /// Remove bandwidth configuration
    async fn remove_bandwidth(&self, queue_name: &str) -> Result<(), AppError>;

    /// Get device status info
    async fn get_status_info(&self) -> Result<serde_json::Value, AppError>;
}

/// Wrapper for MikroTik adapter implementing unified trait
struct MikrotikNetworkAdapter(Arc<dyn MikrotikDeviceAdapter>);

#[async_trait::async_trait]
impl NetworkDeviceAdapter for MikrotikNetworkAdapter {
    async fn get_health_score(&self) -> Result<i32, AppError> {
        let status = self.0.get_device_status().await?;

        // Calculate health score based on CPU, memory, and disk
        let mut score = 100i32;

        // CPU load penalty (0-30 points)
        if status.cpu_load_percent > 90 {
            score -= 30;
        } else if status.cpu_load_percent > 70 {
            score -= 15;
        } else if status.cpu_load_percent > 50 {
            score -= 5;
        }

        // Memory pressure penalty (0-30 points)
        if status.total_memory_bytes > 0 {
            let memory_usage = (status.total_memory_bytes - status.free_memory_bytes) as f64
                / status.total_memory_bytes as f64;
            if memory_usage > 0.9 {
                score -= 30;
            } else if memory_usage > 0.7 {
                score -= 15;
            } else if memory_usage > 0.5 {
                score -= 5;
            }
        }

        // Disk pressure penalty (0-20 points)
        if status.disk_free_bytes < 10 * 1024 * 1024 {
            score -= 20;
        } else if status.disk_free_bytes < 50 * 1024 * 1024 {
            score -= 10;
        }

        Ok(score.max(0))
    }

    async fn apply_bandwidth(
        &self,
        queue_name: &str,
        target: &str,
        download_kbps: u32,
        upload_kbps: u32,
    ) -> Result<(), AppError> {
        use crate::modules::integrations::mikrotik::adapter::QueueConfig;

        let config = QueueConfig {
            name: queue_name.to_string(),
            target: target.to_string(),
            download_kbps,
            upload_kbps,
            burst_download_kbps: None,
            burst_upload_kbps: None,
            burst_threshold_download_kbps: None,
            burst_threshold_upload_kbps: None,
            burst_time_seconds: None,
            priority: None,
            enabled: true,
        };

        self.0.apply_queue(&config).await
    }

    async fn remove_bandwidth(&self, queue_name: &str) -> Result<(), AppError> {
        self.0.remove_queue(queue_name).await
    }

    async fn get_status_info(&self) -> Result<serde_json::Value, AppError> {
        let status = self.0.get_device_status().await?;
        Ok(serde_json::to_value(status).map_err(|e| {
            AppError::Internal(anyhow::anyhow!("Failed to serialize status: {}", e))
        })?)
    }
}

/// Wrapper for Huawei OLT adapter implementing unified trait
struct HuaweiNetworkAdapter(Arc<dyn HuaweiOltAdapter>);

#[async_trait::async_trait]
impl NetworkDeviceAdapter for HuaweiNetworkAdapter {
    async fn get_health_score(&self) -> Result<i32, AppError> {
        let frame = std::env::var("HUAWEI_OLT_FRAME_ID")
            .ok()
            .and_then(|s| s.parse().ok())
            .unwrap_or(0);
        let slot = std::env::var("HUAWEI_OLT_SLOT_ID")
            .ok()
            .and_then(|s| s.parse().ok())
            .unwrap_or(1);
        let pon = std::env::var("HUAWEI_OLT_PON_ID")
            .ok()
            .and_then(|s| s.parse().ok())
            .unwrap_or(0);

        let pon_status = self.0.get_pon_status(frame, slot, pon).await?;

        let mut score = 100i32;

        if pon_status.state != "online" {
            score -= 50;
        }

        let utilization = pon_status.ont_count as f64 / pon_status.max_ont_count as f64;
        if utilization > 0.9 {
            score -= 20;
        } else if utilization > 0.7 {
            score -= 10;
        }

        Ok(score.max(0))
    }

    async fn apply_bandwidth(
        &self,
        queue_name: &str,
        target: &str,
        download_kbps: u32,
        _upload_kbps: u32,
    ) -> Result<(), AppError> {
        use crate::modules::integrations::huawei::adapter::TrafficTable;

        // Use a deterministic index derived from queue_name hash
        use std::collections::hash_map::DefaultHasher;
        use std::hash::{Hash, Hasher};
        let mut hasher = DefaultHasher::new();
        queue_name.hash(&mut hasher);
        let index = (hasher.finish() % 99) as u32 + 1;
        let table = TrafficTable {
            index,
            name: queue_name.to_string(),
            cir_kbps: download_kbps,
            pir_kbps: download_kbps,
        };

        self.0.create_traffic_table(&table).await?;

        let parts: Vec<&str> = target.split('/').collect();
        if parts.len() >= 4 {
            let frame = parts[0].parse().unwrap_or(0);
            let slot = parts[1].parse().unwrap_or(1);
            let pon = parts[2].parse().unwrap_or(0);
            let ont_id = parts[3].parse().unwrap_or(0);

            self.0
                .apply_bandwidth_to_ont(frame, slot, pon, ont_id, 1, Some(index))
                .await?;
        }

        Ok(())
    }

    async fn remove_bandwidth(&self, queue_name: &str) -> Result<(), AppError> {
        use std::collections::hash_map::DefaultHasher;
        use std::hash::{Hash, Hasher};
        let mut hasher = DefaultHasher::new();
        queue_name.hash(&mut hasher);
        let index = (hasher.finish() % 99) as u32 + 1;
        self.0.delete_traffic_table(index).await
    }

    async fn get_status_info(&self) -> Result<serde_json::Value, AppError> {
        let frame = std::env::var("HUAWEI_OLT_FRAME_ID")
            .ok()
            .and_then(|s| s.parse().ok())
            .unwrap_or(0);
        let slot = std::env::var("HUAWEI_OLT_SLOT_ID")
            .ok()
            .and_then(|s| s.parse().ok())
            .unwrap_or(1);
        let pon = std::env::var("HUAWEI_OLT_PON_ID")
            .ok()
            .and_then(|s| s.parse().ok())
            .unwrap_or(0);

        let pon_status = self.0.get_pon_status(frame, slot, pon).await?;
        Ok(serde_json::to_value(pon_status).map_err(|e| {
            AppError::Internal(anyhow::anyhow!("Failed to serialize status: {}", e))
        })?)
    }
}
