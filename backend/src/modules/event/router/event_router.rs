use axum::{middleware, routing::{delete, get}, Router};
use crate::app::SharedState;
use crate::common::middleware::auth_middleware::jwt_middleware;
use crate::modules::event::controller::event_controller;

pub fn event_routes() -> Router<SharedState> {
    Router::new()
        .route("/", get(event_controller::list_events).post(event_controller::publish_event))
        .route("/stats", get(event_controller::get_stats))
        .route("/subscriptions", get(event_controller::list_subscriptions).post(event_controller::create_subscription))
        .route("/subscriptions/:id", delete(event_controller::delete_subscription))
        .route("/:id", get(event_controller::get_event).post(event_controller::mark_processed))
        .route("/aggregate/:aggregate_type/:aggregate_id", get(event_controller::get_aggregate_events))
        .layer(middleware::from_fn(jwt_middleware))
}
