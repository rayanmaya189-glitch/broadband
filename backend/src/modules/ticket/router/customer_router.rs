use axum::routing::{get, post};
use axum::Router;

use crate::app::SharedState;
use crate::common::middleware::auth_middleware::jwt_middleware;
use crate::modules::ticket::customer::controller;

pub fn customer_routes() -> Router<SharedState> {
    Router::new()
        .route("/", get(controller::get_my_tickets).post(controller::create))
        .route("/{id}", get(controller::get_by_id))
        .route("/{id}/comments", get(controller::list_comments).post(controller::add_comment))
        .route("/{id}/feedback", post(controller::set_feedback))
        .layer(axum::middleware::from_fn(jwt_middleware))
}
