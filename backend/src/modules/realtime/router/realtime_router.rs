use axum::{middleware, routing::get, Router};
use crate::app::SharedState;
use crate::common::middleware::auth_middleware::jwt_middleware;
use crate::modules::realtime::controller::realtime_controller;

pub fn realtime_routes() -> Router<SharedState> {
    Router::new()
        .route("/health", get(realtime_controller::health))
        .route("/channels", get(realtime_controller::channels))
        .layer(middleware::from_fn(jwt_middleware))
}
