use axum::{middleware, routing::get, Router};

use crate::app::SharedState;
use crate::common::middleware::auth_middleware::jwt_middleware;
use crate::modules::realtime::controller::realtime_controller;

pub fn realtime_routes() -> Router<SharedState> {
    Router::new()
        .route("/health", get(realtime_controller::health))
        .route("/channels", get(realtime_controller::channels))
        .route("/stats", get(realtime_controller::stats))
        .layer(middleware::from_fn(jwt_middleware))
}

/// WebSocket endpoint (separate from REST routes, no middleware)
pub fn ws_routes() -> Router<SharedState> {
    Router::new()
        .route("/", get(crate::modules::realtime::service::realtime_service::ws_handler))
}
