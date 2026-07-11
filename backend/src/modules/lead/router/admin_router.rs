use axum::routing::{get, post};
use axum::Router;

use crate::app::SharedState;
use crate::common::middleware::rls_setup;
use crate::modules::lead::controller::lead_controller;

pub fn admin_routes() -> Router<SharedState> {
    rls_setup::admin_branch_scoped(
        Router::new()
            .route("/", get(lead_controller::list).post(lead_controller::create))
            .route("/pipeline", get(lead_controller::get_pipeline))
            .route("/stats", get(lead_controller::get_stats))
            .route("/{id}", get(lead_controller::get_by_id).put(lead_controller::update).delete(lead_controller::delete))
            .route("/{id}/status", post(lead_controller::update_status))
            .route("/{id}/assign", post(lead_controller::assign))
            .route("/{id}/activities", get(lead_controller::list_activities).post(lead_controller::add_activity))
            .route("/{id}/convert", post(lead_controller::convert))
    )
}
