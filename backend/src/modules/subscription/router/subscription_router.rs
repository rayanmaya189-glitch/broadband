use axum::middleware;
use axum::routing::{get, post};
use axum::Router;

use crate::app::SharedState;
use crate::common::middleware::auth_middleware::jwt_middleware;
use crate::modules::subscription::controller::subscription_controller;

pub fn subscriptions_routes() -> Router<SharedState> {
    Router::new()
        .route("/", get(subscription_controller::list_subscriptions).post(subscription_controller::create_subscription))
        .route("/:id", get(subscription_controller::get_subscription))
        .route("/:id/suspend", post(subscription_controller::suspend_subscription))
        .route("/:id/reactivate", post(subscription_controller::reactivate_subscription))
        .route("/:id/cancel", post(subscription_controller::cancel_subscription))
        .layer(middleware::from_fn(jwt_middleware))
}
