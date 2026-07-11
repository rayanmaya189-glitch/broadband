use axum::routing::{get, post};
use axum::Router;

use crate::app::SharedState;
use crate::common::middleware::rls_setup;
use crate::modules::subscription::admin::controller;

pub fn admin_routes() -> Router<SharedState> {
    rls_setup::admin_branch_scoped(
        Router::new()
            .route("/", get(controller::list_subscriptions).post(controller::create_subscription))
            .route("/{id}", get(controller::get_subscription))
            .route("/{id}/suspend", post(controller::suspend_subscription))
            .route("/{id}/reactivate", post(controller::reactivate_subscription))
            .route("/{id}/cancel", post(controller::cancel_subscription))
            .route("/{id}/upgrade", post(controller::upgrade_subscription))
            .route("/{id}/downgrade", post(controller::downgrade_subscription))
            .route("/{id}/history", get(controller::get_subscription_history))
    )
}
