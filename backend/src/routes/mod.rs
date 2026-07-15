use axum::routing::get;
use axum::Router;

use crate::shared::app_state::SharedState;

pub fn health_routes() -> Router<SharedState> {
    Router::new()
        .route("/health", get(health_check))
        .route("/ready", get(readiness_check))
}

async fn health_check() -> axum::Json<serde_json::Value> {
    axum::Json(serde_json::json!({
        "status": "healthy",
        "service": "aeroxe-backend",
        "version": env!("CARGO_PKG_VERSION"),
    }))
}

async fn readiness_check() -> axum::Json<serde_json::Value> {
    axum::Json(serde_json::json!({
        "status": "ready",
        "service": "aeroxe-backend",
    }))
}

pub fn v1_routes() -> Router<SharedState> {
    Router::new()
        .nest("/auth", auth_routes())
        .nest("/users", user_routes())
        .nest("/branches", branch_routes())
        .nest("/customers", customer_routes())
        .nest("/plans", plan_routes())
        .nest("/admin/plans", admin_plan_routes())
        .nest("/subscriptions", subscription_routes())
        .nest("/billing", billing_routes())
        .nest("/rbac", rbac_routes())
        .nest("/accounting", accounting_routes())
        .nest("/network", network_routes())
        .nest("/devices", device_routes())
        .nest("/bandwidth", bandwidth_routes())
        .nest("/tickets", ticket_routes())
        .nest("/notifications", notification_routes())
        .nest("/audit", audit_routes())
        .nest("/leads", lead_routes())
        .nest("/referrals", referral_routes())
        .nest("/coverage", coverage_routes())
        .nest("/documents", document_routes())
        .nest("/discovery", discovery_routes())
        .nest("/inventory", inventory_routes())
        .nest("/installations", installation_routes())
        .nest("/payments", payment_routes())
}

fn auth_routes() -> Router<SharedState> {
    Router::new()
        .route(
            "/register",
            axum::routing::post(crate::modules::identity::api::http::register),
        )
        .route(
            "/login",
            axum::routing::post(crate::modules::identity::api::http::login),
        )
        .route(
            "/refresh",
            axum::routing::post(crate::modules::identity::api::http::refresh_token),
        )
}

fn user_routes() -> Router<SharedState> {
    Router::new()
        .route(
            "/",
            axum::routing::get(crate::modules::identity::api::http::list_users),
        )
        .route(
            "/me",
            axum::routing::get(crate::modules::identity::api::http::get_current_user),
        )
}

fn branch_routes() -> Router<SharedState> {
    use crate::modules::branches::api::http;
    Router::new()
        .route(
            "/",
            axum::routing::get(http::list_branches).post(http::create_branch),
        )
        .route(
            "/:id",
            axum::routing::get(http::get_branch)
                .put(http::update_branch)
                .delete(http::delete_branch),
        )
}

fn customer_routes() -> Router<SharedState> {
    use crate::modules::customer::api::http;
    Router::new()
        .route(
            "/",
            axum::routing::get(http::list_customers).post(http::create_customer),
        )
        .route("/:id", axum::routing::get(http::get_customer))
        .route(
            "/:id/status",
            axum::routing::put(http::update_customer_status),
        )
        .route(
            "/:id/addresses",
            axum::routing::get(http::list_addresses).post(http::add_address),
        )
        .route("/:id", axum::routing::delete(http::delete_customer))
}

fn plan_routes() -> Router<SharedState> {
    use crate::modules::plans::api::http;
    Router::new()
        .route("/", axum::routing::get(http::list_plans))
        .route("/:id", axum::routing::get(http::get_plan))
}

fn admin_plan_routes() -> Router<SharedState> {
    use crate::modules::plans::api::http;
    Router::new()
        .route("/", axum::routing::post(http::create_plan))
        .route("/:id/pricing", axum::routing::put(http::update_pricing))
        .route("/:id/approve", axum::routing::post(http::approve_plan))
        .route("/:id", axum::routing::delete(http::deactivate_plan))
}

fn subscription_routes() -> Router<SharedState> {
    use crate::modules::subscription::api::http;
    Router::new()
        .route(
            "/",
            axum::routing::get(http::list_subscriptions).post(http::create_subscription),
        )
        .route(
            "/:id/cancel",
            axum::routing::post(http::cancel_subscription),
        )
        .route(
            "/:id/suspend",
            axum::routing::post(http::suspend_subscription),
        )
}

fn billing_routes() -> Router<SharedState> {
    use crate::modules::billing::api::http;
    Router::new()
        .route(
            "/invoices",
            axum::routing::get(http::list_invoices).post(http::create_invoice),
        )
        .route(
            "/payments",
            axum::routing::get(http::list_payments).post(http::record_payment),
        )
}

fn rbac_routes() -> Router<SharedState> {
    use crate::modules::security::api::http;
    Router::new()
        .route("/roles", axum::routing::get(http::list_roles))
        .route("/permissions", axum::routing::get(http::list_permissions))
        .route("/users/:id/roles", axum::routing::post(http::assign_role))
        .route(
            "/users/:id/roles/:role_id",
            axum::routing::delete(http::revoke_role),
        )
}

fn accounting_routes() -> Router<SharedState> {
    use crate::modules::accounting::api::http;
    Router::new().route("/accounts", axum::routing::get(http::list_accounts))
}

fn network_routes() -> Router<SharedState> {
    use crate::modules::network::api::http;
    Router::new()
        .route(
            "/vlans",
            axum::routing::get(http::list_vlans).post(http::create_vlan),
        )
        .route("/vlans/:id", axum::routing::delete(http::delete_vlan))
        .route(
            "/ip-pools",
            axum::routing::get(http::list_ip_pools).post(http::create_ip_pool),
        )
        .route(
            "/pppoe/sessions",
            axum::routing::get(http::list_pppoe_sessions).post(http::create_pppoe_session),
        )
        .route(
            "/pppoe/sessions/:id/terminate",
            axum::routing::post(http::terminate_pppoe_session),
        )
        .route(
            "/mac-bindings",
            axum::routing::get(http::list_mac_bindings).post(http::create_mac_binding),
        )
}

fn device_routes() -> Router<SharedState> {
    use crate::modules::device::api::http;
    Router::new()
        .route(
            "/",
            axum::routing::get(http::list_devices).post(http::register_device),
        )
        .route("/:id", axum::routing::get(http::get_device))
        .route(
            "/:id/status",
            axum::routing::put(http::update_device_status),
        )
}

fn bandwidth_routes() -> Router<SharedState> {
    use crate::modules::bandwidth::api::http;
    Router::new()
        .route(
            "/profiles",
            axum::routing::get(http::list_profiles).post(http::create_profile),
        )
        .route(
            "/profiles/:id",
            axum::routing::put(http::update_profile).delete(http::delete_profile),
        )
}

fn ticket_routes() -> Router<SharedState> {
    use crate::modules::ticket::api::http;
    Router::new()
        .route(
            "/",
            axum::routing::get(http::list_tickets).post(http::create_ticket),
        )
        .route("/:id", axum::routing::get(http::get_ticket))
        .route("/:id/assign", axum::routing::post(http::assign_ticket))
        .route("/:id/resolve", axum::routing::post(http::resolve_ticket))
}

fn notification_routes() -> Router<SharedState> {
    use crate::modules::notification::api::http;
    Router::new()
        .route(
            "/templates",
            axum::routing::get(http::list_templates).post(http::create_template),
        )
        .route("/send", axum::routing::post(http::send_notification))
}

fn audit_routes() -> Router<SharedState> {
    use crate::modules::audit::api::http;
    Router::new().route("/logs", axum::routing::get(http::list_audit_logs))
}

fn lead_routes() -> Router<SharedState> {
    use crate::modules::lead::api::http;
    Router::new()
        .route(
            "/",
            axum::routing::get(http::list_leads).post(http::create_lead),
        )
        .route("/:id/status", axum::routing::put(http::update_lead_status))
}

fn referral_routes() -> Router<SharedState> {
    use crate::modules::referral::api::http;
    Router::new()
        .route(
            "/",
            axum::routing::get(http::list_referrals).post(http::create_referral),
        )
        .route("/wallet", axum::routing::get(http::get_wallet))
}

fn coverage_routes() -> Router<SharedState> {
    use crate::modules::coverage::api::http;
    Router::new()
        .route(
            "/areas",
            axum::routing::get(http::list_coverage_areas).post(http::create_coverage_area),
        )
        .route("/check", axum::routing::post(http::check_availability))
}

fn document_routes() -> Router<SharedState> {
    use crate::modules::document::api::http;
    Router::new()
        .route("/", axum::routing::get(http::list_documents))
        .route("/presign-upload", axum::routing::post(http::presign_upload))
        .route("/confirm", axum::routing::post(http::confirm_upload))
        .route("/:id", axum::routing::delete(http::delete_document))
}

fn discovery_routes() -> Router<SharedState> {
    use crate::modules::discovery::api::http;
    Router::new()
        .route(
            "/scans",
            axum::routing::get(http::list_scans).post(http::create_scan),
        )
        .route("/results", axum::routing::get(http::list_results))
        .route(
            "/results/:id/approve",
            axum::routing::post(http::approve_result),
        )
}

fn inventory_routes() -> Router<SharedState> {
    use crate::modules::inventory::api::http;
    Router::new()
        .route(
            "/",
            axum::routing::get(http::list_inventory).post(http::create_inventory_item),
        )
        .route(
            "/:id/assign",
            axum::routing::post(http::assign_inventory_item),
        )
}

fn installation_routes() -> Router<SharedState> {
    use crate::modules::installation::api::http;
    Router::new()
        .route(
            "/",
            axum::routing::get(http::list_installations).post(http::create_installation),
        )
        .route(
            "/:id/schedule",
            axum::routing::post(http::schedule_installation),
        )
        .route(
            "/:id/complete",
            axum::routing::post(http::complete_installation),
        )
        .route(
            "/:id/cancel",
            axum::routing::post(http::cancel_installation),
        )
}

fn payment_routes() -> Router<SharedState> {
    use crate::modules::payment::api::http;
    Router::new()
        .route(
            "/create-link",
            axum::routing::post(http::create_payment_link),
        )
        .route("/manual", axum::routing::post(http::record_manual_payment))
        .route("/gateways", axum::routing::get(http::list_gateways))
        .route(
            "/webhook/razorpay",
            axum::routing::post(http::handle_razorpay_webhook),
        )
        .route(
            "/webhook/payu",
            axum::routing::post(http::handle_payu_webhook),
        )
        .route("/:id/retry", axum::routing::post(http::retry_payment))
}
