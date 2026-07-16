/// Payment business rules and invariants
pub struct PaymentRules;

impl PaymentRules {
    /// Minimum payment amount (₹1)
    pub const MIN_PAYMENT_AMOUNT: rust_decimal::Decimal =
        rust_decimal_macros::dec!(1.00);

    /// Maximum payment amount (₹10,00,000)
    pub const MAX_PAYMENT_AMOUNT: rust_decimal::Decimal =
        rust_decimal_macros::dec!(1000000.00);

    /// Payment link expiry duration (24 hours)
    pub const LINK_EXPIRY_HOURS: i64 = 24;

    /// Maximum retry attempts for failed payments
    pub const MAX_RETRY_ATTEMPTS: u32 = 3;

    /// Check if payment amount is within valid range
    pub fn is_valid_amount(amount: rust_decimal::Decimal) -> bool {
        amount >= Self::MIN_PAYMENT_AMOUNT && amount <= Self::MAX_PAYMENT_AMOUNT
    }

    /// Check if payment method is allowed for the given gateway
    pub fn is_method_allowed_for_gateway(
        method: &str,
        gateway: &str,
    ) -> bool {
        match gateway {
            "razorpay" => matches!(method, "upi" | "card" | "netbanking" | "wallet"),
            "payu" => matches!(method, "upi" | "card" | "netbanking"),
            "instamojo" => matches!(method, "upi" | "card"),
            _ => true,
        }
    }

    /// Check if a refund is allowed (within 30 days of payment)
    pub fn is_refund_allowed(payment_date: chrono::NaiveDate) -> bool {
        let now = chrono::Utc::now().date_naive();
        now - payment_date <= chrono::Duration::days(30)
    }
}
