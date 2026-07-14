use axum::{routing::{get, post}, Router};
use crate::app::SharedState;
use crate::common::middleware::rls_setup;
use crate::modules::workflow::controller::workflow_controller;

pub fn workflow_routes() -> Router<SharedState> {
    rls_setup::admin_branch_scoped(
        Router::new()
            .route("/definitions", get(workflow_controller::list_definitions).post(workflow_controller::create_definition))
            .route("/definitions/{definition_id}/steps", post(workflow_controller::add_step))
            .route("/instances", get(workflow_controller::list_instances).post(workflow_controller::start_instance))
            .route("/instances/{instance_id}/steps/{step_id}/approve", post(workflow_controller::approve_step))
            .route("/instances/{instance_id}/steps/{step_id}/reject", post(workflow_controller::reject_step))
    )
}
