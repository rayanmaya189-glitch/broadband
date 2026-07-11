use axum::routing::{get, post};
use axum::Router;

use crate::app::SharedState;
use crate::common::middleware::rls_setup;
use crate::modules::discovery::controller::discovery_controller;

pub fn admin_routes() -> Router<SharedState> {
    rls_setup::admin_branch_scoped(
        Router::new()
            .route("/scans", get(discovery_controller::list_scans).post(discovery_controller::create_scan))
            .route("/scans/{id}/start", post(discovery_controller::start_scan))
            .route("/scans/{id}/stop", post(discovery_controller::stop_scan))
            .route("/results", get(discovery_controller::list_results))
            .route("/results/{id}/approve", post(discovery_controller::approve_result))
            .route("/results/{id}/reject", post(discovery_controller::reject_result))
            .route("/dashboard", get(discovery_controller::dashboard))
    )
}
