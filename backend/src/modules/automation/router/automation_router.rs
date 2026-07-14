use axum::{routing::{get, post, delete}, Router};
use crate::app::SharedState;
use crate::common::middleware::rls_setup;
use crate::modules::automation::controller::automation_controller;

pub fn automation_routes() -> Router<SharedState> {
    rls_setup::admin_branch_scoped(
        Router::new()
            .route("/rules", get(automation_controller::list_rules).post(automation_controller::create_rule))
            .route("/rules/{rule_id}/triggers", post(automation_controller::add_trigger))
            .route("/rules/{rule_id}/actions", post(automation_controller::add_action))
            .route("/rules/{id}", delete(automation_controller::delete_rule))
            .route("/executions", get(automation_controller::list_executions))
    )
}
