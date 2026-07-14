use axum::{routing::{get, post}, Router};

use crate::app::SharedState;
use crate::common::middleware::rls_setup;
use crate::modules::crm::controller::crm_controller;

pub fn crm_routes() -> Router<SharedState> {
    rls_setup::admin_branch_scoped(
        Router::new()
            .route("/interactions", get(crm_controller::list_interactions).post(crm_controller::create_interaction))
            .route("/customers/{customer_id}/notes", get(crm_controller::list_notes).post(crm_controller::create_note))
            .route("/tags", get(crm_controller::list_tags).post(crm_controller::create_tag))
            .route("/customers/{customer_id}/tags/{tag_id}", post(crm_controller::assign_tag))
            .route("/segments", get(crm_controller::list_segments).post(crm_controller::create_segment))
    )
}
