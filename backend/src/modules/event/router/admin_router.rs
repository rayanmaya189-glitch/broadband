use axum::routing::{get, delete};
use axum::Router;

use crate::app::SharedState;
use crate::common::middleware::rls_setup;
use crate::modules::event::controller::event_controller;

pub fn admin_routes() -> Router<SharedState> {
    rls_setup::admin_scoped(
        Router::new()
            .route("/", get(event_controller::list).post(event_controller::publish_event))
            .route("/subscriptions", get(event_controller::list_subscriptions).post(event_controller::create_subscription))
            .route("/subscriptions/{id}", delete(event_controller::delete_subscription))
            .route("/aggregate/{aggregate_type}/{aggregate_id}", get(event_controller::get_aggregate_events))
            .route("/{id}", get(event_controller::get_by_id).post(event_controller::mark_processed))
    )
}
