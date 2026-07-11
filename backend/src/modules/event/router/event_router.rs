use axum::{middleware, routing::get, Router};
use crate::app::SharedState;
use crate::common::middleware::auth_middleware::jwt_middleware;
use crate::modules::event::controller::event_controller;

pub fn event_routes() -> Router<SharedState> {
    Router::new()
        .route("/", get(event_controller::list))
        .route("/:id", get(event_controller::get_by_id).post(event_controller::mark_processed))
        .layer(middleware::from_fn(jwt_middleware))
}
