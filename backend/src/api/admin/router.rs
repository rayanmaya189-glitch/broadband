use axum::Router;

use crate::app::SharedState;
use crate::modules::accounting::router::admin_router as accounting_admin;
use crate::modules::audit::router::admin_router as audit_admin;
use crate::modules::audit::router::entity_history_admin_router as entity_history_admin;
use crate::modules::automation::router::automation_router as automation_admin;
use crate::modules::bandwidth::router::admin_router as bandwidth_admin;
use crate::modules::billing::router::admin_router as billing_admin;
use crate::modules::branch::router::admin_router as branch_admin;
use crate::modules::crm::router::crm_router as crm_admin;
use crate::modules::customer::router::admin_router as customer_admin;
use crate::modules::device::router::admin_router as device_admin;
use crate::modules::discovery::router::admin_router as discovery_admin;
use crate::modules::document::router::admin_router as document_admin;
use crate::modules::event::router::admin_router as event_admin;
use crate::modules::installation::router::admin_router as installation_admin;
use crate::modules::inventory::router::admin_router as inventory_admin;
use crate::modules::lead::router::admin_router as lead_admin;
use crate::modules::monitoring::router::monitoring_router as monitoring_admin;
use crate::modules::network::router::admin_router as network_admin;
use crate::modules::notification::router::admin_router as notification_admin;
use crate::modules::payment_gateway::router::admin_router as payment_gateway_admin;
use crate::modules::permission::router::admin_router as permission_admin;
use crate::modules::plan::router::admin_router as plan_admin;
use crate::modules::realtime::router::admin_router as realtime_admin;
use crate::modules::referral::router::admin_router as referral_admin;
use crate::modules::reporting::router::reporting_router as reporting_admin;
use crate::modules::role::router::admin_router as role_admin;
use crate::modules::scheduler::router::scheduler_router as scheduler_admin;
use crate::modules::subscription::router::admin_router as subscription_admin;
use crate::modules::ticket::router::admin_router as ticket_admin;
use crate::modules::traffic::router::traffic_router as traffic_admin;
use crate::modules::user::router::admin_router as user_admin;
use crate::modules::workflow::router::workflow_router as workflow_admin;

/// Aggregates all admin/staff module routes under a single `/api/v1/admin` prefix.
///
/// Each module router handles its own auth via `rls_setup::admin_scoped()`
/// or `rls_setup::admin_branch_scoped()`.
pub fn admin_api_router() -> Router<SharedState> {
    Router::new()
        .nest("/users", user_admin::admin_routes())
        .nest("/subscriptions", subscription_admin::admin_routes())
        .nest("/tickets", ticket_admin::admin_routes())
        .nest("/billing", billing_admin::admin_routes())
        .nest("/plans", plan_admin::admin_routes())
        .nest("/referrals", referral_admin::admin_routes())
        .nest("/roles", role_admin::admin_routes())
        .nest("/permissions", permission_admin::admin_routes())
        .nest("/branches", branch_admin::admin_routes())
        .nest("/customers", customer_admin::admin_routes())
        .nest("/leads", lead_admin::admin_routes())
        .nest("/devices", device_admin::admin_routes())
        .nest("/bandwidth", bandwidth_admin::admin_routes())
        .nest("/network", network_admin::admin_routes())
        .nest("/installations", installation_admin::admin_routes())
        .nest("/inventory", inventory_admin::admin_routes())
        .nest("/notifications", notification_admin::admin_routes())
        .nest("/events", event_admin::admin_routes())
        .nest("/documents", document_admin::admin_routes())
        .nest("/accounting", accounting_admin::admin_routes())
        .nest("/payments", payment_gateway_admin::admin_routes())
        .nest("/discovery", discovery_admin::admin_routes())
        .nest("/realtime", realtime_admin::admin_routes())
        .nest("/audit", audit_admin::admin_routes())
        .nest("/audit/entity-history", entity_history_admin::entity_history_admin_routes())
        .nest("/crm", crm_admin::crm_routes())
        .nest("/reporting", reporting_admin::reporting_routes())
        .nest("/monitoring", monitoring_admin::monitoring_routes())
        .nest("/traffic", traffic_admin::traffic_routes())
        .nest("/automation", automation_admin::automation_routes())
        .nest("/scheduler", scheduler_admin::scheduler_routes())
        .nest("/workflow", workflow_admin::workflow_routes())
}

