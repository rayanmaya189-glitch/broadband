use axum::{middleware, routing::{delete, get, post}, Router};
use crate::app::SharedState;
use crate::common::middleware::auth_middleware::jwt_middleware;
use crate::modules::coverage::controller::coverage_controller;

pub fn coverage_routes() -> Router<SharedState> {
    Router::new()
        .route("/areas", get(coverage_controller::list_areas).post(coverage_controller::create_area))
        .route("/areas/:id", get(coverage_controller::get_area).put(coverage_controller::update_area).delete(coverage_controller::delete_area))
        .route("/areas/:id/pincodes", get(coverage_controller::list_pincodes).post(coverage_controller::add_pincode))
        .route("/areas/:id/pincodes/:pincode", delete(coverage_controller::remove_pincode))
        .route("/check", post(coverage_controller::check_availability))
        .route("/stats", get(coverage_controller::get_stats))
        .layer(middleware::from_fn(jwt_middleware))
}
