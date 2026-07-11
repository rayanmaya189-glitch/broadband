use axum::routing::{get, post};
use axum::Router;

use crate::app::SharedState;
use crate::common::middleware::auth_middleware::jwt_middleware;
use crate::modules::subscription::customer::controller;

pub fn customer_routes() -> Router<SharedState> {
    Router::new()
        .route("/", get(controller::get_my_subscriptions))
        .route("/{id}", get(controller::get_subscription))
        .route("/{id}/upgrade", post(controller::upgrade_subscription))
        .route("/{id}/downgrade", post(controller::downgrade_subscription))
        .route("/{id}/history", get(controller::get_subscription_history))
        .layer(axum::middleware::from_fn(jwt_middleware))
}
