use axum::routing::{get, post};
use axum::Router;

use crate::app::SharedState;
use crate::common::middleware::rls_setup;
use crate::modules::ticket::admin::controller;

pub fn admin_routes() -> Router<SharedState> {
    rls_setup::admin_branch_scoped(
        Router::new()
            .route("/", get(controller::list).post(controller::create))
            .route("/dashboard", get(controller::get_dashboard))
            .route("/my-assignments", get(controller::get_my_assignments))
            .route("/{id}", get(controller::get_by_id).put(controller::update).delete(controller::delete))
            .route("/{id}/status", post(controller::update_status))
            .route("/{id}/assign", post(controller::assign))
            .route("/{id}/escalate", post(controller::escalate))
            .route("/{id}/resolve", post(controller::resolve))
            .route("/{id}/close", post(controller::close))
            .route("/{id}/reopen", post(controller::reopen))
            .route("/{id}/escalations", get(controller::get_escalations))
            .route("/{id}/status-history", get(controller::get_status_history))
    )
}
