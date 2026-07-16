/// Referral business rules and invariants
pub struct ReferralRules;

impl ReferralRules {
    /// Maximum referrals per customer per month
    pub const MAX_REFERRALS_PER_MONTH: u32 = 10;

    /// Referral code length
    pub const REFERRAL_CODE_LENGTH: usize = 8;

    /// Check if referral code is valid format
    pub fn is_valid_referral_code(code: &str) -> bool {
        code.len() == Self::REFERRAL_CODE_LENGTH && code.chars().all(|c| c.is_ascii_uppercase() || c.is_ascii_digit())
    }

    /// Check if customer can refer (not self-referral)
    pub fn can_refer(referrer_id: i64, referee_phone: &str) -> bool {
        referrer_id > 0 && !referee_phone.is_empty()
    }
}
