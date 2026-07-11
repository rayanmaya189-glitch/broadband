use axum::Router;

use crate::app::SharedState;
use crate::modules::user::router::customer_router as user_customer;
use crate::modules::subscription::router::customer_router as subscription_customer;
use crate::modules::ticket::router::customer_router as ticket_customer;
use crate::modules::billing::router::customer_router as billing_customer;
use crate::modules::referral::router::customer_router as referral_customer;

/// Aggregates all customer self-service module routes under a single `/api/v1/customer` prefix.
///
/// Each module router handles its own auth via `rls_setup::customer_scoped()` or direct `jwt_middleware`.
/// Note: Plan browsing is public (see `/api/v1/plans` in main.rs public routes).
pub fn customer_api_router() -> Router<SharedState> {
    Router::new()
        .nest("/profile", user_customer::customer_routes())
        .nest("/subscriptions", subscription_customer::customer_routes())
        .nest("/tickets", ticket_customer::customer_routes())
        .nest("/billing", billing_customer::customer_routes())
        .nest("/referrals", referral_customer::customer_routes())
}
