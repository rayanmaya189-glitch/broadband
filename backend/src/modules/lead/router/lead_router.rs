use axum::{routing::{get, post}, Router};

use crate::app::SharedState;
use crate::common::middleware::rls_setup;
use crate::modules::lead::controller::lead_controller;

pub fn lead_routes() -> Router<SharedState> {
    rls_setup::branch_scoped(
        Router::new()
            .route("/", get(lead_controller::list_leads).post(lead_controller::create_lead))
            .route("/pipeline", get(lead_controller::get_pipeline))
            .route("/stats", get(lead_controller::get_stats))
            .route("/{id}", get(lead_controller::get_lead).put(lead_controller::update_lead).delete(lead_controller::delete_lead))
            .route("/{id}/status", post(lead_controller::update_status))
            .route("/{id}/assign", post(lead_controller::assign_lead))
            .route("/{id}/activities", get(lead_controller::get_activities).post(lead_controller::add_activity))
            .route("/{id}/convert", post(lead_controller::convert_lead))
    )
}
