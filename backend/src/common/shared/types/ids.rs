//! Newtype ID wrappers for type safety.
//!
//! These wrappers prevent accidentally mixing up IDs from different entities.
//! They are serializable for API responses and database operations.

use serde::{Deserialize, Serialize};

macro_rules! define_id {
    ($name:ident, $doc:expr) => {
        #[doc = $doc]
        #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
        pub struct $name(pub i64);

        impl $name {
            pub fn new(id: i64) -> Self {
                Self(id)
            }

            pub fn inner(&self) -> i64 {
                self.0
            }
        }

        impl std::fmt::Display for $name {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                write!(f, "{}", self.0)
            }
        }

        impl From<i64> for $name {
            fn from(id: i64) -> Self {
                Self(id)
            }
        }

        impl From<$name> for i64 {
            fn from(id: $name) -> Self {
                id.0
            }
        }
    };
}

define_id!(UserId, "Unique identifier for a user");
define_id!(CustomerId, "Unique identifier for a customer");
define_id!(BranchId, "Unique identifier for a branch");
define_id!(SubscriptionId, "Unique identifier for a subscription");
define_id!(InvoiceId, "Unique identifier for an invoice");
define_id!(PaymentId, "Unique identifier for a payment");
define_id!(PlanId, "Unique identifier for a plan");
define_id!(TicketId, "Unique identifier for a ticket");
define_id!(DeviceId, "Unique identifier for a network device");
define_id!(RoleId, "Unique identifier for a role");
define_id!(PermissionId, "Unique identifier for a permission");
define_id!(LeadId, "Unique identifier for a lead");
define_id!(CoverageAreaId, "Unique identifier for a coverage area");
define_id!(InstallationId, "Unique identifier for an installation order");
define_id!(InventoryItemId, "Unique identifier for an inventory item");
define_id!(NotificationId, "Unique identifier for a notification");
define_id!(AuditLogId, "Unique identifier for an audit log");
define_id!(EventId, "Unique identifier for an event");
define_id!(DocumentId, "Unique identifier for a document");
define_id!(ReferralProgramId, "Unique identifier for a referral program");
define_id!(WorkflowDefinitionId, "Unique identifier for a workflow definition");
define_id!(WorkflowInstanceId, "Unique identifier for a workflow instance");
define_id!(AutomationRuleId, "Unique identifier for an automation rule");
define_id!(ScheduledTaskId, "Unique identifier for a scheduled task");
define_id!(ReportId, "Unique identifier for a report");
define_id!(SegmentId, "Unique identifier for a CRM segment");
define_id!(TagId, "Unique identifier for a CRM tag");
