use axum::routing::get;
use axum::Router;

use crate::app::SharedState;
use crate::modules::plan::customer::controller;

/// Customer-facing plan routes (public - no auth needed to view plans).
pub fn customer_routes() -> Router<SharedState> {
    Router::new()
        .route("/", get(controller::list_plans))
        .route("/{id}", get(controller::get_plan))
}
