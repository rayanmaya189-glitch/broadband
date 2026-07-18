//! Huawei OLT Integration Adapter
//!
//! Supports Huawei GPON/XG-PON OLT devices (MA5683T, MA5800 series) via:
//! - Real SSH CLI commands for configuration (using russh)
//! - DBA profile management (upstream bandwidth)
//! - Traffic table management (downstream bandwidth)
//! - ONT authorization and provisioning

use async_trait::async_trait;
use serde::{Deserialize, Serialize};
use tracing::{debug, info, warn};

use super::ssh_client;
use crate::shared::errors::AppError;

// ============================================================================
// Configuration
// ============================================================================

/// Huawei OLT connection configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HuaweiOltConfig {
    pub host: String,
    pub port: u16,
    pub username: String,
    pub password: String,
    pub enable_password: Option<String>,
    pub frame_id: Option<u32>,
    pub slot_id: Option<u32>,
    pub pon_id: Option<u32>,
    pub ont_id: Option<u32>,
    pub ssh_timeout_secs: u64,
}

impl Default for HuaweiOltConfig {
    fn default() -> Self {
        Self {
            host: std::env::var("HUAWEI_OLT_HOST").unwrap_or_else(|_| "192.168.10.1".to_string()),
            port: std::env::var("HUAWEI_OLT_PORT")
                .unwrap_or_else(|_| "22".to_string())
                .parse()
                .unwrap_or(22),
            username: std::env::var("HUAWEI_OLT_USERNAME").unwrap_or_else(|_| "root".to_string()),
            password: std::env::var("HUAWEI_OLT_PASSWORD").unwrap_or_default(),
            enable_password: std::env::var("HUAWEI_OLT_ENABLE_PASSWORD").ok(),
            frame_id: std::env::var("HUAWEI_OLT_FRAME_ID")
                .ok()
                .and_then(|s| s.parse().ok()),
            slot_id: std::env::var("HUAWEI_OLT_SLOT_ID")
                .ok()
                .and_then(|s| s.parse().ok()),
            pon_id: std::env::var("HUAWEI_OLT_PON_ID")
                .ok()
                .and_then(|s| s.parse().ok()),
            ont_id: std::env::var("HUAWEI_OLT_ONT_ID")
                .ok()
                .and_then(|s| s.parse().ok()),
            ssh_timeout_secs: std::env::var("HUAWEI_OLT_SSH_TIMEOUT")
                .unwrap_or_else(|_| "30".to_string())
                .parse()
                .unwrap_or(30),
        }
    }
}

// ============================================================================
// Data Types
// ============================================================================

/// DBA Profile for upstream bandwidth control
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DbaProfile {
    pub profile_id: u32,
    pub name: String,
    pub profile_type: DbaProfileType,
    pub max_bandwidth_kbps: u32,
    pub assured_bandwidth_kbps: Option<u32>,
    pub fixed_bandwidth_kbps: Option<u32>,
}

/// DBA Profile type (determines bandwidth allocation behavior)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum DbaProfileType {
    /// Fixed bandwidth (e.g., for VoIP)
    Type1 = 1,
    /// Assured bandwidth
    Type2 = 2,
    /// Assured + Maximum bandwidth
    Type3 = 3,
    /// Best effort (Maximum only, typical for Internet)
    Type4 = 4,
}

impl DbaProfileType {
    pub fn as_str(&self) -> &str {
        match self {
            Self::Type1 => "type1",
            Self::Type2 => "type2",
            Self::Type3 => "type3",
            Self::Type4 => "type4",
        }
    }
}

/// Traffic Table for downstream bandwidth control
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TrafficTable {
    pub index: u32,
    pub name: String,
    pub cir_kbps: u32, // Committed Information Rate
    pub pir_kbps: u32, // Peak Information Rate
}

/// ONT Status information
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OntStatus {
    pub frame: u32,
    pub slot: u32,
    pub pon: u32,
    pub ont_id: u32,
    pub sn: String,
    pub state: String,
    pub rx_power_dbm: Option<f64>,
    pub tx_power_dbm: Option<f64>,
    pub distance_meters: Option<u32>,
    pub uptime: Option<String>,
    pub model: Option<String>,
}

/// PON Interface Status
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PonInterfaceStatus {
    pub frame: u32,
    pub slot: u32,
    pub pon_id: u32,
    pub state: String,
    pub ont_count: u32,
    pub max_ont_count: u32,
    pub bandwidth_mbps: u32,
}

/// CLI command result
#[derive(Debug, Clone)]
pub struct CliResult {
    pub success: bool,
    pub output: String,
    pub error: Option<String>,
}

// ============================================================================
// Adapter Trait
// ============================================================================

/// Trait for Huawei OLT device adapters
#[async_trait]
pub trait HuaweiOltAdapter: Send + Sync {
    /// Create a DBA profile for upstream bandwidth
    async fn create_dba_profile(&self, profile: &DbaProfile) -> Result<(), AppError>;

    /// Delete a DBA profile
    async fn delete_dba_profile(&self, profile_id: u32) -> Result<(), AppError>;

    /// List all DBA profiles
    async fn list_dba_profiles(&self) -> Result<Vec<DbaProfile>, AppError>;

    /// Create a traffic table for downstream bandwidth
    async fn create_traffic_table(&self, table: &TrafficTable) -> Result<(), AppError>;

    /// Delete a traffic table
    async fn delete_traffic_table(&self, index: u32) -> Result<(), AppError>;

    /// List all traffic tables
    async fn list_traffic_tables(&self) -> Result<Vec<TrafficTable>, AppError>;

    /// Get ONT status by PON and ONT ID
    async fn get_ont_status(
        &self,
        frame: u32,
        slot: u32,
        pon: u32,
        ont_id: u32,
    ) -> Result<OntStatus, AppError>;

    /// Get all ONTs on a PON interface
    async fn list_onts_on_pon(
        &self,
        frame: u32,
        slot: u32,
        pon: u32,
    ) -> Result<Vec<OntStatus>, AppError>;

    /// Get PON interface status
    async fn get_pon_status(
        &self,
        frame: u32,
        slot: u32,
        pon: u32,
    ) -> Result<PonInterfaceStatus, AppError>;

    /// Apply bandwidth profile to an ONT
    async fn apply_bandwidth_to_ont(
        &self,
        frame: u32,
        slot: u32,
        pon: u32,
        ont_id: u32,
        dba_profile_id: u32,
        traffic_table_index: Option<u32>,
    ) -> Result<(), AppError>;

    /// Execute raw CLI command (for advanced operations)
    async fn execute_cli(&self, command: &str) -> Result<CliResult, AppError>;
}

// ============================================================================
// SSH CLI Adapter (Real SSH Connection)
// ============================================================================

/// Huawei OLT adapter using real SSH CLI commands via russh
pub struct HuaweiOltSshAdapter {
    config: HuaweiOltConfig,
}

impl HuaweiOltSshAdapter {
    /// Create a new adapter from configuration
    pub fn new(config: HuaweiOltConfig) -> Self {
        Self { config }
    }

    /// Create adapter from environment variables
    pub fn from_env() -> Self {
        Self::new(HuaweiOltConfig::default())
    }

    /// Execute a command via real SSH connection
    async fn ssh_execute(&self, command: &str) -> Result<CliResult, AppError> {
        debug!(
            host = %self.config.host,
            command = %command,
            "Executing Huawei OLT CLI command via SSH"
        );

        let output = if let Some(ref enable_pw) = self.config.enable_password {
            // Use enable mode if configured
            ssh_client::execute_olt_command_with_enable(
                &self.config.host,
                self.config.port,
                &self.config.username,
                &self.config.password,
                enable_pw,
                command,
                self.config.ssh_timeout_secs,
            )
            .await?
        } else {
            // Direct command execution
            ssh_client::execute_olt_command(
                &self.config.host,
                self.config.port,
                &self.config.username,
                &self.config.password,
                command,
                self.config.ssh_timeout_secs,
            )
            .await?
        };

        Ok(CliResult {
            success: true,
            output,
            error: None,
        })
    }

    /// Parse DBA profile from CLI output
    fn parse_dba_profiles(&self, output: &str) -> Vec<DbaProfile> {
        let mut profiles = Vec::new();
        let mut current_profile: Option<DbaProfile> = None;

        for line in output.lines() {
            let line = line.trim();
            if line.contains("Profile ID:") || line.starts_with("DBA Profile") {
                if let Some(profile) = current_profile.take() {
                    profiles.push(profile);
                }
                // Try to extract profile ID
                let id = if let Some(pos) = line.find(':') {
                    line[pos + 1..].trim().parse().unwrap_or(0)
                } else {
                    0
                };
                current_profile = Some(DbaProfile {
                    profile_id: id,
                    name: String::new(),
                    profile_type: DbaProfileType::Type4,
                    max_bandwidth_kbps: 0,
                    assured_bandwidth_kbps: None,
                    fixed_bandwidth_kbps: None,
                });
            } else if let Some(ref mut profile) = current_profile {
                if line.contains("Profile Name:") || line.contains("Name:") {
                    profile.name = line
                        .split_once(':')
                        .map(|x| x.1)
                        .unwrap_or("")
                        .trim()
                        .to_string();
                } else if line.contains("Type:") {
                    let type_num = line
                        .split_once(':')
                        .map(|x| x.1)
                        .unwrap_or("4")
                        .trim()
                        .parse()
                        .unwrap_or(4);
                    profile.profile_type = match type_num {
                        1 => DbaProfileType::Type1,
                        2 => DbaProfileType::Type2,
                        3 => DbaProfileType::Type3,
                        _ => DbaProfileType::Type4,
                    };
                } else if line.contains("Max BW:") || line.contains("Max Bandwidth:") {
                    let bw_str = line.split_once(':').map(|x| x.1).unwrap_or("0").trim();
                    profile.max_bandwidth_kbps =
                        bw_str.replace("kbps", "").trim().parse().unwrap_or(0);
                } else if line.contains("Assured BW:") {
                    let bw_str = line.split_once(':').map(|x| x.1).unwrap_or("0").trim();
                    profile.assured_bandwidth_kbps = bw_str.replace("kbps", "").trim().parse().ok();
                }
            }
        }

        if let Some(profile) = current_profile {
            profiles.push(profile);
        }

        profiles
    }

    /// Parse ONT status from CLI output
    fn parse_ont_status(&self, output: &str, frame: u32, slot: u32, pon: u32) -> Vec<OntStatus> {
        let mut onts = Vec::new();
        let mut current_ont: Option<OntStatus> = None;

        for line in output.lines() {
            let line = line.trim();
            if line.contains("ONT ID:") || line.contains("ONT: ") {
                if let Some(ont) = current_ont.take() {
                    onts.push(ont);
                }
                let id = line
                    .split_once(':')
                    .map(|x| x.1)
                    .unwrap_or("0")
                    .trim()
                    .parse()
                    .unwrap_or(0);
                current_ont = Some(OntStatus {
                    frame,
                    slot,
                    pon,
                    ont_id: id,
                    sn: String::new(),
                    state: "unknown".to_string(),
                    rx_power_dbm: None,
                    tx_power_dbm: None,
                    distance_meters: None,
                    uptime: None,
                    model: None,
                });
            } else if let Some(ref mut ont) = current_ont {
                if line.contains("SN:") || line.contains("SN ") {
                    ont.sn = line.splitn(2, ':').last().unwrap_or("").trim().to_string();
                } else if line.contains("State:") || line.contains("State ") {
                    ont.state = line.splitn(2, ':').last().unwrap_or("").trim().to_string();
                } else if line.contains("Rx Power:") {
                    let power_str = line.splitn(2, ':').last().unwrap_or("0").trim();
                    ont.rx_power_dbm = power_str.replace("dBm", "").trim().parse().ok();
                } else if line.contains("Tx Power:") {
                    let power_str = line.splitn(2, ':').last().unwrap_or("0").trim();
                    ont.tx_power_dbm = power_str.replace("dBm", "").trim().parse().ok();
                } else if line.contains("Distance:") {
                    let dist_str = line.splitn(2, ':').last().unwrap_or("0").trim();
                    ont.distance_meters = dist_str.replace("m", "").trim().parse().ok();
                }
            }
        }

        if let Some(ont) = current_ont {
            onts.push(ont);
        }

        onts
    }
}

#[async_trait]
impl HuaweiOltAdapter for HuaweiOltSshAdapter {
    async fn create_dba_profile(&self, profile: &DbaProfile) -> Result<(), AppError> {
        let mut cmd = format!(
            "dba-profile add profile-id {} profile-name {} {}",
            profile.profile_id,
            profile.name,
            profile.profile_type.as_str()
        );

        // Add bandwidth parameters based on type
        match profile.profile_type {
            DbaProfileType::Type1 => {
                if let Some(fixed) = profile.fixed_bandwidth_kbps {
                    cmd = format!("{} fixed {}", cmd, fixed);
                }
            }
            DbaProfileType::Type2 => {
                if let Some(assured) = profile.assured_bandwidth_kbps {
                    cmd = format!("{} assured {}", cmd, assured);
                }
            }
            DbaProfileType::Type3 => {
                if let Some(assured) = profile.assured_bandwidth_kbps {
                    cmd = format!(
                        "{} assured {} max {}",
                        cmd, assured, profile.max_bandwidth_kbps
                    );
                }
            }
            DbaProfileType::Type4 => {
                cmd = format!("{} max {}", cmd, profile.max_bandwidth_kbps);
            }
        }

        let result = self.ssh_execute(&cmd).await?;
        if result.success {
            info!(profile_id = profile.profile_id, name = %profile.name, "Created Huawei DBA profile");
            Ok(())
        } else {
            Err(AppError::External(format!(
                "Failed to create DBA profile: {}",
                result.error.unwrap_or(result.output)
            )))
        }
    }

    async fn delete_dba_profile(&self, profile_id: u32) -> Result<(), AppError> {
        let cmd = format!("dba-profile delete profile-id {}", profile_id);
        let result = self.ssh_execute(&cmd).await?;
        if result.success {
            info!(profile_id = profile_id, "Deleted Huawei DBA profile");
            Ok(())
        } else {
            Err(AppError::External(format!(
                "Failed to delete DBA profile: {}",
                result.error.unwrap_or(result.output)
            )))
        }
    }

    async fn list_dba_profiles(&self) -> Result<Vec<DbaProfile>, AppError> {
        let result = self.ssh_execute("display dba-profile all").await?;
        Ok(self.parse_dba_profiles(&result.output))
    }

    async fn create_traffic_table(&self, table: &TrafficTable) -> Result<(), AppError> {
        let cmd = format!(
            "traffic table ip index {} name {} cir {} pir {}",
            table.index, table.name, table.cir_kbps, table.pir_kbps
        );

        let result = self.ssh_execute(&cmd).await?;
        if result.success {
            info!(index = table.index, name = %table.name, "Created Huawei traffic table");
            Ok(())
        } else {
            Err(AppError::External(format!(
                "Failed to create traffic table: {}",
                result.error.unwrap_or(result.output)
            )))
        }
    }

    async fn delete_traffic_table(&self, index: u32) -> Result<(), AppError> {
        let cmd = format!("traffic table ip delete index {}", index);
        let result = self.ssh_execute(&cmd).await?;
        if result.success {
            info!(index = index, "Deleted Huawei traffic table");
            Ok(())
        } else {
            Err(AppError::External(format!(
                "Failed to delete traffic table: {}",
                result.error.unwrap_or(result.output)
            )))
        }
    }

    async fn list_traffic_tables(&self) -> Result<Vec<TrafficTable>, AppError> {
        let result = self.ssh_execute("display traffic-table all").await?;
        let mut tables = Vec::new();
        for line in result.output.lines() {
            if line.contains("Index:") && line.contains("Name:") {
                let parts: Vec<&str> = line.split_whitespace().collect();
                if parts.len() >= 4 {
                    tables.push(TrafficTable {
                        index: parts[1].parse().unwrap_or(0),
                        name: parts[3].to_string(),
                        cir_kbps: 0,
                        pir_kbps: 0,
                    });
                }
            }
        }
        Ok(tables)
    }

    async fn get_ont_status(
        &self,
        frame: u32,
        slot: u32,
        pon: u32,
        ont_id: u32,
    ) -> Result<OntStatus, AppError> {
        let cmd = format!("display ont-info {} {} {} {}", frame, slot, pon, ont_id);
        let result = self.ssh_execute(&cmd).await?;
        let onts = self.parse_ont_status(&result.output, frame, slot, pon);
        onts.into_iter().next().ok_or_else(|| {
            AppError::NotFound(format!(
                "ONT {}/{}/{}/{} not found",
                frame, slot, pon, ont_id
            ))
        })
    }

    async fn list_onts_on_pon(
        &self,
        frame: u32,
        slot: u32,
        pon: u32,
    ) -> Result<Vec<OntStatus>, AppError> {
        let cmd = format!("display ont-info {} {} {}", frame, slot, pon);
        let result = self.ssh_execute(&cmd).await?;
        Ok(self.parse_ont_status(&result.output, frame, slot, pon))
    }

    async fn get_pon_status(
        &self,
        frame: u32,
        slot: u32,
        pon: u32,
    ) -> Result<PonInterfaceStatus, AppError> {
        let cmd = format!("display ont info {} {} {} summary", frame, slot, pon);
        let result = self.ssh_execute(&cmd).await?;

        // Parse output for ONT count
        let ont_count = result
            .output
            .lines()
            .filter(|l| l.contains("ONT") || l.contains("Total"))
            .count() as u32;

        Ok(PonInterfaceStatus {
            frame,
            slot,
            pon_id: pon,
            state: "online".to_string(),
            ont_count: ont_count.max(1),
            max_ont_count: 128,
            bandwidth_mbps: 2500,
        })
    }

    async fn apply_bandwidth_to_ont(
        &self,
        frame: u32,
        slot: u32,
        pon: u32,
        ont_id: u32,
        dba_profile_id: u32,
        traffic_table_index: Option<u32>,
    ) -> Result<(), AppError> {
        // Apply DBA profile (upstream)
        let cmd = format!(
            "ont traffic-profile {} {} {} {} dba-profile-id {}",
            frame, slot, pon, ont_id, dba_profile_id
        );
        let result = self.ssh_execute(&cmd).await?;
        if !result.success {
            return Err(AppError::External(format!(
                "Failed to apply DBA profile: {}",
                result.error.unwrap_or(result.output)
            )));
        }

        // Apply traffic table (downstream) if specified
        if let Some(tt_index) = traffic_table_index {
            let cmd = format!(
                "ont traffic-table {} {} {} {} index {}",
                frame, slot, pon, ont_id, tt_index
            );
            let result = self.ssh_execute(&cmd).await?;
            if !result.success {
                warn!(
                    "Failed to apply traffic table: {}",
                    result.error.unwrap_or(result.output)
                );
            }
        }

        info!(
            frame,
            slot, pon, ont_id, dba_profile_id, "Applied bandwidth profile to Huawei ONT"
        );
        Ok(())
    }

    async fn execute_cli(&self, command: &str) -> Result<CliResult, AppError> {
        self.ssh_execute(command).await
    }
}
