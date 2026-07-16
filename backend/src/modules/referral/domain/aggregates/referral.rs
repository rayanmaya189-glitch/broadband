use crate::modules::referral::domain::value_objects::{ReferralId, ReferralStatus};

/// Referral aggregate root - represents a customer referral
#[derive(Debug, Clone)]
pub struct Referral {
    pub id: ReferralId,
    pub referrer_id: i64,
    pub referee_name: String,
    pub referee_phone: String,
    pub referral_code: String,
    pub status: ReferralStatus,
    pub referrer_reward_status: Option<String>,
    pub referee_reward_status: Option<String>,
}

/// Domain errors for Referral aggregate
#[derive(Debug, Clone, PartialEq)]
pub enum ReferralDomainError {
    ReferralNotFound(i64),
    AlreadyRewarded,
    CannotRewardNonActivated,
    SelfReferral,
}

impl std::fmt::Display for ReferralDomainError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::ReferralNotFound(id) => write!(f, "Referral {} not found", id),
            Self::AlreadyRewarded => write!(f, "Referral already rewarded"),
            Self::CannotRewardNonActivated => write!(f, "Cannot reward referral for non-activated customer"),
            Self::SelfReferral => write!(f, "Cannot refer yourself"),
        }
    }
}

impl std::error::Error for ReferralDomainError {}

impl Referral {
    pub fn new(referrer_id: i64, referee_name: String, referee_phone: String, referral_code: String) -> Self {
        Self {
            id: ReferralId::new(0),
            referrer_id,
            referee_name,
            referee_phone,
            referral_code,
            status: ReferralStatus::Pending,
            referrer_reward_status: None,
            referee_reward_status: None,
        }
    }

    pub fn activate(&mut self) {
        self.status = ReferralStatus::Activated;
    }

    pub fn reward(&mut self) -> Result<(), ReferralDomainError> {
        if self.status == ReferralStatus::Rewarded {
            return Err(ReferralDomainError::AlreadyRewarded);
        }
        if self.status != ReferralStatus::Activated {
            return Err(ReferralDomainError::CannotRewardNonActivated);
        }
        self.status = ReferralStatus::Rewarded;
        self.referrer_reward_status = Some("credited".to_string());
        self.referee_reward_status = Some("applied".to_string());
        Ok(())
    }

    pub fn is_rewardable(&self) -> bool {
        self.status == ReferralStatus::Activated
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new_referral() {
        let referral = Referral::new(1, "Rahul".to_string(), "+919876543210".to_string(), "REF-ABC".to_string());
        assert_eq!(referral.status, ReferralStatus::Pending);
    }

    #[test]
    fn test_referral_lifecycle() {
        let mut referral = Referral::new(1, "Rahul".to_string(), "+919876543210".to_string(), "REF-ABC".to_string());
        referral.activate();
        assert!(referral.is_rewardable());
        referral.reward().unwrap();
        assert_eq!(referral.status, ReferralStatus::Rewarded);
    }
}
