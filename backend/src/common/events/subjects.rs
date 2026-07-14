//! Centralized NATS subject definitions for the AeroXe Broadband platform.
//!
//! All event subjects follow the naming convention:
//! `aeroxe.<module>.<action>`
//!
//! This module ensures consistent subject naming across all publishers and subscribers.

/// Customer module subjects
pub mod customer {
    pub const CREATED: &str = "aeroxe.customer.created";
    pub const UPDATED: &str = "aeroxe.customer.updated";
    pub const SUSPENDED: &str = "aeroxe.customer.suspended";
    pub const DEACTIVATED: &str = "aeroxe.customer.deactivated";
    pub const KYC_VERIFIED: &str = "aeroxe.customer.kyc_verified";
}

/// Subscription module subjects
pub mod subscription {
    pub const CREATED: &str = "aeroxe.subscription.created";
    pub const CHANGED: &str = "aeroxe.subscription.changed";
    pub const CANCELLED: &str = "aeroxe.subscription.cancelled";
    pub const RENEWED: &str = "aeroxe.subscription.renewed";
    pub const EXPIRED: &str = "aeroxe.subscription.expired";
}

/// Billing module subjects
pub mod billing {
    pub const INVOICE_CREATED: &str = "aeroxe.billing.invoice_created";
    pub const INVOICE_PAID: &str = "aeroxe.billing.invoice_paid";
    pub const INVOICE_OVERDUE: &str = "aeroxe.billing.invoice_overdue";
    pub const PAYMENT_COMPLETED: &str = "aeroxe.billing.payment_completed";
    pub const PAYMENT_FAILED: &str = "aeroxe.billing.payment_failed";
    pub const REFUND_PROCESSED: &str = "aeroxe.billing.refund_processed";
}

/// Network module subjects
pub mod network {
    pub const DEVICE_ONLINE: &str = "aeroxe.network.device_online";
    pub const DEVICE_OFFLINE: &str = "aeroxe.network.device_offline";
    pub const VLAN_CREATED: &str = "aeroxe.network.vlan_created";
    pub const IP_ALLOCATED: &str = "aeroxe.network.ip_allocated";
    pub const IP_RELEASED: &str = "aeroxe.network.ip_released";
    pub const BANDWIDTH_APPLIED: &str = "aeroxe.network.bandwidth_applied";
}

/// Device module subjects
pub mod device {
    pub const REGISTERED: &str = "aeroxe.device.registered";
    pub const STATUS_CHANGED: &str = "aeroxe.device.status_changed";
    pub const FIRMWARE_UPDATED: &str = "aeroxe.device.firmware_updated";
    pub const METRIC_RECORDED: &str = "aeroxe.device.metric_recorded";
}

/// Ticket module subjects
pub mod ticket {
    pub const CREATED: &str = "aeroxe.ticket.created";
    pub const ASSIGNED: &str = "aeroxe.ticket.assigned";
    pub const ESCALATED: &str = "aeroxe.ticket.escalated";
    pub const RESOLVED: &str = "aeroxe.ticket.resolved";
    pub const CLOSED: &str = "aeroxe.ticket.closed";
}

/// Notification module subjects
pub mod notification {
    pub const SEND: &str = "aeroxe.notification.send";
    pub const SENT: &str = "aeroxe.notification.sent";
    pub const FAILED: &str = "aeroxe.notification.failed";
}

/// Installation module subjects
pub mod installation {
    pub const SCHEDULED: &str = "aeroxe.installation.scheduled";
    pub const STARTED: &str = "aeroxe.installation.started";
    pub const COMPLETED: &str = "aeroxe.installation.completed";
}

/// Lead module subjects
pub mod lead {
    pub const CREATED: &str = "aeroxe.lead.created";
    pub const CONVERTED: &str = "aeroxe.lead.converted";
    pub const LOST: &str = "aeroxe.lead.lost";
}

/// Monitoring module subjects
pub mod monitoring {
    pub const ALERT_TRIGGERED: &str = "aeroxe.monitoring.alert_triggered";
    pub const ALERT_RESOLVED: &str = "aeroxe.monitoring.alert_resolved";
    pub const HEALTH_CHECK_FAILED: &str = "aeroxe.monitoring.health_check_failed";
}

/// Audit module subjects
pub mod audit {
    pub const ENTITY_CHANGED: &str = "aeroxe.audit.entity_changed";
    pub const ENTITY_ROLLBACK: &str = "aeroxe.audit.entity_rollback";
}

/// Workflow module subjects
pub mod workflow {
    pub const STARTED: &str = "aeroxe.workflow.started";
    pub const STEP_COMPLETED: &str = "aeroxe.workflow.step_completed";
    pub const APPROVED: &str = "aeroxe.workflow.approved";
    pub const REJECTED: &str = "aeroxe.workflow.rejected";
    pub const COMPLETED: &str = "aeroxe.workflow.completed";
}

/// Payment gateway module subjects
pub mod payment_gateway {
    pub const WEBHOOK_RECEIVED: &str = "aeroxe.payment_gateway.webhook_received";
    pub const TRANSACTION_COMPLETED: &str = "aeroxe.payment_gateway.transaction_completed";
    pub const TRANSACTION_FAILED: &str = "aeroxe.payment_gateway.transaction_failed";
}

/// Helper to build a custom subject with the aeroxe prefix
pub fn build_subject(module: &str, action: &str) -> String {
    format!("aeroxe.{module}.{action}")
}
