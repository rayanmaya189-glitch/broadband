//! Dunning policy constants shared by the billing API and the billing worker.

/// Days overdue before a subscription is suspended.
pub const SUSPENSION_DAY: i32 = 10;

/// Days overdue before a subscription is terminated.
pub const TERMINATION_DAY: i32 = 30;

/// Days overdue at which reminder stages escalate.
pub const REMINDER_DAYS: [i32; 2] = [3, 7];

/// Late fee as a percentage of the invoice total.
pub const LATE_FEE_PERCENT: &str = "2.0";

/// Maximum late fee as a percentage of the invoice total.
pub const LATE_FEE_CAP_PERCENT: &str = "10.0";

/// Notification channels used for dunning.
pub const CHANNELS: [&str; 3] = ["sms", "email", "whatsapp"];
