use crate::modules::discovery::domain::value_objects::{DiscoveryScanId, DiscoveryScanStatus, ScanType};

/// DiscoveryScan aggregate root - represents a network discovery scan configuration
#[derive(Debug, Clone)]
pub struct DiscoveryScan {
    pub id: DiscoveryScanId,
    pub branch_id: i64,
    pub name: String,
    pub scan_type: ScanType,
    pub target_subnets: Option<serde_json::Value>,
    pub scan_interval_seconds: i32,
    pub is_active: bool,
    pub last_scan_at: Option<chrono::DateTime<chrono::Utc>>,
    pub next_scan_at: Option<chrono::DateTime<chrono::Utc>>,
    pub status: DiscoveryScanStatus,
}

/// Domain errors for DiscoveryScan aggregate
#[derive(Debug, Clone, PartialEq)]
pub enum DiscoveryDomainError {
    ScanNotFound(i64),
    InvalidScanType,
    ScanAlreadyRunning,
}

impl std::fmt::Display for DiscoveryDomainError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::ScanNotFound(id) => write!(f, "Discovery scan {} not found", id),
            Self::InvalidScanType => write!(f, "Invalid scan type"),
            Self::ScanAlreadyRunning => write!(f, "Scan is already running"),
        }
    }
}

impl std::error::Error for DiscoveryDomainError {}

impl DiscoveryScan {
    pub fn new(branch_id: i64, name: String, scan_type: ScanType) -> Self {
        Self {
            id: DiscoveryScanId::new(0),
            branch_id,
            name,
            scan_type,
            target_subnets: None,
            scan_interval_seconds: 900,
            is_active: true,
            last_scan_at: None,
            next_scan_at: None,
            status: DiscoveryScanStatus::Idle,
        }
    }

    pub fn start(&mut self) -> Result<(), DiscoveryDomainError> {
        if self.status == DiscoveryScanStatus::Running {
            return Err(DiscoveryDomainError::ScanAlreadyRunning);
        }
        self.status = DiscoveryScanStatus::Running;
        self.last_scan_at = Some(chrono::Utc::now());
        Ok(())
    }

    pub fn complete(&mut self) {
        self.status = DiscoveryScanStatus::Idle;
        self.next_scan_at = Some(chrono::Utc::now() + chrono::Duration::seconds(self.scan_interval_seconds as i64));
    }

    pub fn is_due(&self) -> bool {
        self.is_active && self.next_scan_at.map_or(true, |t| t <= chrono::Utc::now())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new_scan() {
        let scan = DiscoveryScan::new(1, "SNMP Walk".to_string(), ScanType::SnmpWalk);
        assert_eq!(scan.status, DiscoveryScanStatus::Idle);
        assert!(scan.is_active);
    }

    #[test]
    fn test_scan_lifecycle() {
        let mut scan = DiscoveryScan::new(1, "Test".to_string(), ScanType::ArpScan);
        scan.start().unwrap();
        assert_eq!(scan.status, DiscoveryScanStatus::Running);
        scan.complete();
        assert_eq!(scan.status, DiscoveryScanStatus::Idle);
    }
}
