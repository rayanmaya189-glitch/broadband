use axum::routing::{get, post};
use axum::Router;

use crate::app::SharedState;
use crate::common::middleware::rls_setup;
use crate::modules::subscription::controller::subscription_controller;

pub fn subscriptions_routes() -> Router<SharedState> {
    rls_setup::branch_scoped(
        Router::new()
            .route("/", get(subscription_controller::list_subscriptions).post(subscription_controller::create_subscription))
            .route("/:id", get(subscription_controller::get_subscription))
            .route("/:id/suspend", post(subscription_controller::suspend_subscription))
            .route("/:id/reactivate", post(subscription_controller::reactivate_subscription))
            .route("/:id/cancel", post(subscription_controller::cancel_subscription))
            .route("/:id/upgrade", post(subscription_controller::upgrade_subscription))
            .route("/:id/downgrade", post(subscription_controller::downgrade_subscription))
            .route("/:id/history", get(subscription_controller::get_subscription_history))
    )
}
