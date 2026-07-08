use axum::{middleware, routing::{delete, get, post}, Router};
use crate::app::SharedState;
use crate::common::middleware::auth_middleware::jwt_middleware;
use crate::modules::coverage::controller::coverage_controller;

pub fn coverage_routes() -> Router<SharedState> {
    Router::new()
        .route("/areas", get(coverage_controller::list_areas).post(coverage_controller::create_area))
        .route("/areas/:id", delete(coverage_controller::delete_area))
        .route("/check", post(coverage_controller::check_availability))
        .layer(middleware::from_fn(jwt_middleware))
}
