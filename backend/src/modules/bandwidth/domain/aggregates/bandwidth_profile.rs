use crate::modules::bandwidth::domain::value_objects::{BandwidthProfileId, BandwidthStatus};

/// BandwidthProfile aggregate root - represents a traffic shaping profile
#[derive(Debug, Clone, PartialEq)]
pub struct BandwidthProfile {
    pub id: BandwidthProfileId,
    pub name: String,
    pub description: Option<String>,
    pub plan_id: Option<i64>,
    pub download_kbps: i32,
    pub upload_kbps: i32,
    pub burst_download_kbps: Option<i32>,
    pub burst_upload_kbps: Option<i32>,
    pub burst_duration_seconds: i32,
    pub priority: i32,
    pub is_active: bool,
    pub status: BandwidthStatus,
}

/// Domain errors for BandwidthProfile aggregate
#[derive(Debug, Clone, PartialEq)]
pub enum BandwidthDomainError {
    InvalidSpeed,
    ProfileNotFound(i64),
    InvalidBurstConfiguration,
    CannotApplyInactiveProfile,
}

impl std::fmt::Display for BandwidthDomainError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::InvalidSpeed => write!(f, "Speed values must be positive"),
            Self::ProfileNotFound(id) => write!(f, "Bandwidth profile {} not found", id),
            Self::InvalidBurstConfiguration => write!(f, "Burst speed must be greater than sustained speed"),
            Self::CannotApplyInactiveProfile => write!(f, "Cannot apply an inactive bandwidth profile"),
        }
    }
}

impl std::error::Error for BandwidthDomainError {}

impl BandwidthProfile {
    pub fn new(
        name: String,
        plan_id: Option<i64>,
        download_kbps: i32,
        upload_kbps: i32,
    ) -> Result<Self, BandwidthDomainError> {
        if download_kbps <= 0 || upload_kbps <= 0 {
            return Err(BandwidthDomainError::InvalidSpeed);
        }
        Ok(Self {
            id: BandwidthProfileId::new(0),
            name,
            description: None,
            plan_id,
            download_kbps,
            upload_kbps,
            burst_download_kbps: None,
            burst_upload_kbps: None,
            burst_duration_seconds: 30,
            priority: 1,
            is_active: true,
            status: BandwidthStatus::Active,
        })
    }

    pub fn set_burst(&mut self, download: i32, upload: i32, duration: i32) -> Result<(), BandwidthDomainError> {
        if download < self.download_kbps || upload < self.upload_kbps {
            return Err(BandwidthDomainError::InvalidBurstConfiguration);
        }
        self.burst_download_kbps = Some(download);
        self.burst_upload_kbps = Some(upload);
        self.burst_duration_seconds = duration;
        Ok(())
    }

    pub fn can_apply(&self) -> bool {
        self.is_active && self.status == BandwidthStatus::Active
    }

    pub fn disable(&mut self) {
        self.status = BandwidthStatus::Inactive;
        self.is_active = false;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new_bandwidth_profile() {
        let profile = BandwidthProfile::new("Basic 50".to_string(), Some(1), 50000, 25000);
        assert!(profile.is_ok());
        let profile = profile.unwrap();
        assert!(profile.can_apply());
    }

    #[test]
    fn test_invalid_speed() {
        let profile = BandwidthProfile::new("Invalid".to_string(), None, 0, 25000);
        assert_eq!(profile, Err(BandwidthDomainError::InvalidSpeed));
    }

    #[test]
    fn test_set_burst() {
        let mut profile = BandwidthProfile::new("Basic".to_string(), None, 50000, 25000).unwrap();
        assert!(profile.set_burst(75000, 37500, 30).is_ok());
        assert_eq!(profile.burst_download_kbps, Some(75000));
        // Burst less than sustained should fail
        assert_eq!(
            profile.set_burst(40000, 20000, 30),
            Err(BandwidthDomainError::InvalidBurstConfiguration)
        );
    }
}
