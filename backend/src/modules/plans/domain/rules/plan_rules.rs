/// Plan business rules and invariants
pub struct PlanRules;

impl PlanRules {
    /// Minimum download speed (1 Mbps)
    pub const MIN_DOWNLOAD_MBPS: i32 = 1;

    /// Maximum download speed (10 Gbps)
    pub const MAX_DOWNLOAD_MBPS: i32 = 10000;

    /// Valid billing periods in months
    pub const VALID_BILLING_PERIODS: &[i32] = &[1, 3, 6, 12];

    /// Minimum plan price (₹1)
    pub const MIN_PRICE: rust_decimal::Decimal = rust_decimal_macros::dec!(1.00);

    /// Maximum plan price (₹1,00,000/month)
    pub const MAX_MONTHLY_PRICE: rust_decimal::Decimal = rust_decimal_macros::dec!(100000.00);

    /// Check if speed is within valid range
    pub fn is_valid_speed(download_mbps: i32, upload_mbps: i32) -> bool {
        (Self::MIN_DOWNLOAD_MBPS..=Self::MAX_DOWNLOAD_MBPS).contains(&download_mbps)
            && upload_mbps <= download_mbps
    }

    /// Check if billing period is valid
    pub fn is_valid_billing_period(months: i32) -> bool {
        Self::VALID_BILLING_PERIODS.contains(&months)
    }

    /// Calculate discount for longer billing periods
    pub fn calculate_period_discount(months: i32) -> rust_decimal::Decimal {
        match months {
            1 => rust_decimal::Decimal::ZERO,
            3 => rust_decimal_macros::dec!(0.05),   // 5% discount
            6 => rust_decimal_macros::dec!(0.08),   // 8% discount
            12 => rust_decimal_macros::dec!(0.12),  // 12% discount
            _ => rust_decimal::Decimal::ZERO,
        }
    }

    /// Validate plan slug format
    pub fn is_valid_slug(slug: &str) -> bool {
        if slug.is_empty() || slug.len() > 100 {
            return false;
        }
        slug.chars()
            .all(|c| c.is_ascii_lowercase() || c.is_ascii_digit() || c == '-')
    }
}
