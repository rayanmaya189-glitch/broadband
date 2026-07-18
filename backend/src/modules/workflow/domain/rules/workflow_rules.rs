/// Workflow business rules and invariants
pub struct WorkflowRules;

impl WorkflowRules {
    /// Maximum approval timeout (hours)
    pub const MAX_APPROVAL_TIMEOUT_HOURS: i64 = 72;

    /// Operations requiring approval
    pub const OPERATIONS_REQUIRING_APPROVAL: &[&str] = &[
        "firmware_update",
        "bulk_suspension",
        "network_config_change",
        "large_refund",
        "device_removal",
        "plan_pricing_change",
    ];

    /// Check if an operation requires approval
    pub fn requires_approval(operation: &str) -> bool {
        Self::OPERATIONS_REQUIRING_APPROVAL.contains(&operation)
    }

    /// Minimum refund amount requiring approval (₹5000)
    pub const MIN_REFUND_APPROVAL_AMOUNT: rust_decimal::Decimal =
        rust_decimal_macros::dec!(5000.00);
}
