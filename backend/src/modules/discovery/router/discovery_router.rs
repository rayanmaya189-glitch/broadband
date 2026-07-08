use axum::{middleware, routing::get, Router};
use crate::app::SharedState;
use crate::common::middleware::auth_middleware::jwt_middleware;
use crate::modules::discovery::controller::discovery_controller;

pub fn discovery_routes() -> Router<SharedState> {
    Router::new()
        .route("/scans", get(discovery_controller::list_scans).post(discovery_controller::create_scan))
        .route("/results", get(discovery_controller::list_results))
        .layer(middleware::from_fn(jwt_middleware))
}
