//! MikroTik RouterOS Integration Adapter
//!
//! Supports RouterOS v7 REST API and legacy API for:
//! - Simple Queue management (bandwidth limits per customer)
//! - Queue Tree management (QoS policies)
//! - DHCP lease management
//! - PPPoE session monitoring
//! - Device health monitoring (SNMP-style metrics)

use async_trait::async_trait;
use reqwest::Client;
use serde::{Deserialize, Serialize};
use tracing::{debug, info, warn};

use crate::shared::errors::AppError;

// ============================================================================
// Configuration
// ============================================================================

/// MikroTik device connection configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MikrotikConfig {
    pub host: String,
    pub port: u16,
    pub username: String,
    pub password: String,
    pub use_ssl: bool,
    pub api_version: String,
}

impl Default for MikrotikConfig {
    fn default() -> Self {
        Self {
            host: std::env::var("MIKROTIK_HOST").unwrap_or_else(|_| "192.168.88.1".to_string()),
            port: std::env::var("MIKROTIK_PORT")
                .unwrap_or_else(|_| "443".to_string())
                .parse()
                .unwrap_or(443),
            username: std::env::var("MIKROTIK_USERNAME").unwrap_or_else(|_| "admin".to_string()),
            password: std::env::var("MIKROTIK_PASSWORD").unwrap_or_default(),
            use_ssl: std::env::var("MIKROTIK_USE_SSL")
                .unwrap_or_else(|_| "true".to_string())
                .parse()
                .unwrap_or(true),
            api_version: std::env::var("MIKROTIK_API_VERSION")
                .unwrap_or_else(|_| "v7".to_string()),
        }
    }
}

// ============================================================================
// Data Types
// ============================================================================

/// Bandwidth queue configuration for a customer
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct QueueConfig {
    pub name: String,
    pub target: String,
    pub download_kbps: u32,
    pub upload_kbps: u32,
    pub burst_download_kbps: Option<u32>,
    pub burst_upload_kbps: Option<u32>,
    pub burst_threshold_download_kbps: Option<u32>,
    pub burst_threshold_upload_kbps: Option<u32>,
    pub burst_time_seconds: Option<u32>,
    pub priority: Option<u8>,
    pub enabled: bool,
}

/// Device status from MikroTik
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DeviceStatus {
    pub identity: String,
    pub version: String,
    pub uptime: String,
    pub cpu_count: u32,
    pub cpu_load_percent: u8,
    pub total_memory_bytes: u64,
    pub free_memory_bytes: u64,
    pub disk_free_bytes: u64,
    pub board_name: String,
}

/// DHCP lease information
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DhcpLease {
    pub address: String,
    pub mac_address: String,
    pub hostname: Option<String>,
    pub status: String,
    pub expires: Option<String>,
}

/// PPPoE active session
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PppoeSession {
    pub id: String,
    pub service: String,
    pub user: String,
    pub encoding: String,
    pub uptime: String,
    pub bytes_in: u64,
    pub bytes_out: u64,
    pub rate_in: u64,
    pub rate_out: u64,
}

// ============================================================================
// Adapter Trait
// ============================================================================

/// Trait for MikroTik device adapters
#[async_trait]
pub trait MikrotikDeviceAdapter: Send + Sync {
    /// Apply bandwidth queue to a customer's IP
    async fn apply_queue(&self, config: &QueueConfig) -> Result<(), AppError>;

    /// Remove bandwidth queue by name
    async fn remove_queue(&self, name: &str) -> Result<(), AppError>;

    /// Get all active queues
    async fn list_queues(&self) -> Result<Vec<QueueConfig>, AppError>;

    /// Get device status (CPU, memory, uptime)
    async fn get_device_status(&self) -> Result<DeviceStatus, AppError>;

    /// Get DHCP leases
    async fn get_dhcp_leases(&self) -> Result<Vec<DhcpLease>, AppError>;

    /// Get active PPPoE sessions
    async fn get_pppoe_sessions(&self) -> Result<Vec<PppoeSession>, AppError>;

    /// Add a PPPoE user (for customer provisioning)
    async fn add_pppoe_user(
        &self,
        username: &str,
        password: &str,
        service: &str,
    ) -> Result<(), AppError>;

    /// Remove a PPPoE user
    async fn remove_pppoe_user(&self, username: &str) -> Result<(), AppError>;

    /// Execute a raw RouterOS command
    async fn execute_command(&self, command: &str) -> Result<serde_json::Value, AppError>;
}

// ============================================================================
// REST API Adapter (RouterOS v7+)
// ============================================================================

/// MikroTik REST API adapter for RouterOS v7+
pub struct MikrotikAdapter {
    config: MikrotikConfig,
    client: Client,
    base_url: String,
}

impl MikrotikAdapter {
    /// Create a new adapter from configuration
    pub fn new(config: MikrotikConfig) -> Self {
        let scheme = if config.use_ssl { "https" } else { "http" };
        let base_url = format!("{}://{}:{}/rest", scheme, config.host, config.port);

        let client = Client::builder()
            .danger_accept_invalid_certs(true) // MikroTik often uses self-signed certs
            .timeout(std::time::Duration::from_secs(30))
            .build()
            .unwrap_or_default();

        Self {
            config,
            client,
            base_url,
        }
    }

    /// Create adapter from environment variables
    pub fn from_env() -> Self {
        Self::new(MikrotikConfig::default())
    }

    /// Get authentication header
    fn auth_header(&self) -> String {
        use base64::Engine;
        let credentials = format!("{}:{}", self.config.username, self.config.password);
        format!("Basic {}", base64::engine::general_purpose::STANDARD.encode(credentials))
    }

    /// Make a GET request to the REST API
    async fn rest_get(&self, path: &str) -> Result<serde_json::Value, AppError> {
        let url = format!("{}/{}", self.base_url, path.trim_start_matches('/'));
        debug!(url = %url, "MikroTik REST GET");

        let response = self
            .client
            .get(&url)
            .header("Authorization", self.auth_header())
            .send()
            .await
            .map_err(|e| AppError::External(format!("MikroTik connection failed: {}", e)))?;

        if !response.status().is_success() {
            let status = response.status();
            let body = response.text().await.unwrap_or_default();
            warn!(status = %status, body = %body, "MikroTik REST GET failed");
            return Err(AppError::External(format!(
                "MikroTik API error ({}): {}",
                status, body
            )));
        }

        response
            .json()
            .await
            .map_err(|e| AppError::External(format!("Failed to parse MikroTik response: {}", e)))
    }

    /// Make a POST request to the REST API (create/add)
    async fn rest_post(
        &self,
        path: &str,
        body: serde_json::Value,
    ) -> Result<serde_json::Value, AppError> {
        let url = format!("{}/{}", self.base_url, path.trim_start_matches('/'));
        debug!(url = %url, body = %body, "MikroTik REST POST");

        let response = self
            .client
            .post(&url)
            .header("Authorization", self.auth_header())
            .json(&body)
            .send()
            .await
            .map_err(|e| AppError::External(format!("MikroTik connection failed: {}", e)))?;

        if !response.status().is_success() {
            let status = response.status();
            let body = response.text().await.unwrap_or_default();
            warn!(status = %status, body = %body, "MikroTik REST POST failed");
            return Err(AppError::External(format!(
                "MikroTik API error ({}): {}",
                status, body
            )));
        }

        // MikroTik REST API returns 201 with empty body for successful creates
        let text = response.text().await.unwrap_or_default();
        if text.is_empty() {
            Ok(serde_json::json!({}))
        } else {
            serde_json::from_str(&text)
                .map_err(|e| AppError::External(format!("Failed to parse MikroTik response: {}", e)))
        }
    }

    /// Make a DELETE request to the REST API
    async fn rest_delete(&self, path: &str) -> Result<(), AppError> {
        let url = format!("{}/{}", self.base_url, path.trim_start_matches('/'));
        debug!(url = %url, "MikroTik REST DELETE");

        let response = self
            .client
            .delete(&url)
            .header("Authorization", self.auth_header())
            .send()
            .await
            .map_err(|e| AppError::External(format!("MikroTik connection failed: {}", e)))?;

        if !response.status().is_success() {
            let status = response.status();
            let body = response.text().await.unwrap_or_default();
            warn!(status = %status, body = %body, "MikroTik REST DELETE failed");
            return Err(AppError::External(format!(
                "MikroTik API error ({}): {}",
                status, body
            )));
        }

        Ok(())
    }

}

#[async_trait]
impl MikrotikDeviceAdapter for MikrotikAdapter {
    async fn apply_queue(&self, config: &QueueConfig) -> Result<(), AppError> {
        // Build queue simple add payload
        let mut body = serde_json::json!({
            "name": config.name,
            "target": config.target,
            "max-limit": format!("{}/{}", config.download_kbps, config.upload_kbps),
            "enabled": config.enabled,
        });

        // Add burst parameters if specified
        if let Some(burst_dl) = config.burst_download_kbps {
            if let Some(burst_ul) = config.burst_upload_kbps {
                body["burst-limit"] =
                    serde_json::json!(format!("{}/{}", burst_dl, burst_ul));
            }
        }
        if let Some(thresh_dl) = config.burst_threshold_download_kbps {
            if let Some(thresh_ul) = config.burst_threshold_upload_kbps {
                body["burst-threshold"] =
                    serde_json::json!(format!("{}/{}", thresh_dl, thresh_ul));
            }
        }
        if let Some(burst_time) = config.burst_time_seconds {
            body["burst-time"] =
                serde_json::json!(format!("{}/{}", burst_time, burst_time));
        }
        if let Some(priority) = config.priority {
            body["priority"] = serde_json::json!(priority);
        }

        self.rest_post("/queue/simple", body).await?;
        info!(name = %config.name, target = %config.target, "Applied MikroTik queue");
        Ok(())
    }

    async fn remove_queue(&self, name: &str) -> Result<(), AppError> {
        // First find the queue by name to get its .id
        let queues = self.rest_get("/queue/simple").await?;
        if let Some(arr) = queues.as_array() {
            for queue in arr {
                if queue["name"].as_str() == Some(name) {
                    if let Some(id) = queue[".id"].as_str() {
                        self.rest_delete(&format!("/queue/simple/{}", id)).await?;
                        info!(name = %name, "Removed MikroTik queue");
                        return Ok(());
                    }
                }
            }
        }
        warn!(name = %name, "MikroTik queue not found for deletion");
        Ok(())
    }

    async fn list_queues(&self) -> Result<Vec<QueueConfig>, AppError> {
        let queues = self.rest_get("/queue/simple").await?;
        let mut result = Vec::new();

        if let Some(arr) = queues.as_array() {
            for q in arr {
                let max_limit = q["max-limit"].as_str().unwrap_or("0/0");
                let parts: Vec<&str> = max_limit.split('/').collect();
                let download_kbps = parts.first().and_then(|s| s.parse().ok()).unwrap_or(0);
                let upload_kbps = parts.get(1).and_then(|s| s.parse().ok()).unwrap_or(0);

                result.push(QueueConfig {
                    name: q["name"].as_str().unwrap_or("").to_string(),
                    target: q["target"].as_str().unwrap_or("").to_string(),
                    download_kbps,
                    upload_kbps,
                    burst_download_kbps: None,
                    burst_upload_kbps: None,
                    burst_threshold_download_kbps: None,
                    burst_threshold_upload_kbps: None,
                    burst_time_seconds: None,
                    priority: q["priority"].as_str().and_then(|s| s.parse().ok()),
                    enabled: q["disabled"].as_str().map(|d| d != "true").unwrap_or(true),
                });
            }
        }

        Ok(result)
    }

    async fn get_device_status(&self) -> Result<DeviceStatus, AppError> {
        let resource = self.rest_get("/system/resource").await?;

        Ok(DeviceStatus {
            identity: self
                .rest_get("/system/identity")
                .await
                .ok()
                .and_then(|v| v["name"].as_str().map(|s| s.to_string()))
                .unwrap_or_default(),
            version: resource["version"].as_str().unwrap_or("").to_string(),
            uptime: resource["uptime"].as_str().unwrap_or("").to_string(),
            cpu_count: resource["cpu-count"].as_str().and_then(|s| s.parse().ok()).unwrap_or(0),
            cpu_load_percent: resource["cpu-load"].as_str().and_then(|s| s.parse().ok()).unwrap_or(0),
            total_memory_bytes: resource["total-memory"].as_str().and_then(|s| s.parse().ok()).unwrap_or(0),
            free_memory_bytes: resource["free-memory"].as_str().and_then(|s| s.parse().ok()).unwrap_or(0),
            disk_free_bytes: resource["disk-free"].as_str().and_then(|s| s.parse().ok()).unwrap_or(0),
            board_name: resource["board-name"].as_str().unwrap_or("").to_string(),
        })
    }

    async fn get_dhcp_leases(&self) -> Result<Vec<DhcpLease>, AppError> {
        let leases = self.rest_get("/ip/dhcp-server/lease").await?;
        let mut result = Vec::new();

        if let Some(arr) = leases.as_array() {
            for l in arr {
                result.push(DhcpLease {
                    address: l["address"].as_str().unwrap_or("").to_string(),
                    mac_address: l["mac-address"].as_str().unwrap_or("").to_string(),
                    hostname: l["host-name"].as_str().map(|s| s.to_string()),
                    status: if l["status"].as_str() == Some("bound") {
                        "active".to_string()
                    } else {
                        "inactive".to_string()
                    },
                    expires: l["expires"].as_str().map(|s| s.to_string()),
                });
            }
        }

        Ok(result)
    }

    async fn get_pppoe_sessions(&self) -> Result<Vec<PppoeSession>, AppError> {
        let sessions = self.rest_get("/interface/pppoe-server/active").await?;
        let mut result = Vec::new();

        if let Some(arr) = sessions.as_array() {
            for s in arr {
                result.push(PppoeSession {
                    id: s[".id"].as_str().unwrap_or("").to_string(),
                    service: s["service"].as_str().unwrap_or("").to_string(),
                    user: s["user"].as_str().unwrap_or("").to_string(),
                    encoding: s["encoding"].as_str().unwrap_or("").to_string(),
                    uptime: s["uptime"].as_str().unwrap_or("").to_string(),
                    bytes_in: s["bytes-in"].as_str().and_then(|s| s.parse().ok()).unwrap_or(0),
                    bytes_out: s["bytes-out"].as_str().and_then(|s| s.parse().ok()).unwrap_or(0),
                    rate_in: s["rate-in"].as_str().and_then(|s| s.parse().ok()).unwrap_or(0),
                    rate_out: s["rate-out"].as_str().and_then(|s| s.parse().ok()).unwrap_or(0),
                });
            }
        }

        Ok(result)
    }

    async fn add_pppoe_user(
        &self,
        username: &str,
        password: &str,
        service: &str,
    ) -> Result<(), AppError> {
        let body = serde_json::json!({
            "name": username,
            "password": password,
            "service": service,
            "profile": "default",
        });

        self.rest_post("/ppp/secret", body).await?;
        info!(username = %username, service = %service, "Added MikroTik PPPoE user");
        Ok(())
    }

    async fn remove_pppoe_user(&self, username: &str) -> Result<(), AppError> {
        // Find the user by name
        let secrets = self.rest_get("/ppp/secret").await?;
        if let Some(arr) = secrets.as_array() {
            for s in arr {
                if s["name"].as_str() == Some(username) {
                    if let Some(id) = s[".id"].as_str() {
                        self.rest_delete(&format!("/ppp/secret/{}", id)).await?;
                        info!(username = %username, "Removed MikroTik PPPoE user");
                        return Ok(());
                    }
                }
            }
        }
        warn!(username = %username, "MikroTik PPPoE user not found for deletion");
        Ok(())
    }

    async fn execute_command(&self, command: &str) -> Result<serde_json::Value, AppError> {
        // For REST API, we can use /run endpoint
        let body = serde_json::json!({
            "command": command,
        });

        self.rest_post("/run", body).await
    }
}
