use axum::{routing::{get, post}, Router};

use crate::app::SharedState;
use crate::common::middleware::rls_setup;
use crate::modules::ticket::controller::ticket_controller;

pub fn ticket_routes() -> Router<SharedState> {
    rls_setup::branch_scoped(
        Router::new()
            .route("/", get(ticket_controller::list).post(ticket_controller::create))
            .route("/dashboard", get(ticket_controller::get_dashboard))
            .route("/my-assignments", get(ticket_controller::get_my_assignments))
            .route("/{id}", get(ticket_controller::get_by_id).put(ticket_controller::update).delete(ticket_controller::delete))
            .route("/{id}/status", post(ticket_controller::update_status))
            .route("/{id}/assign", post(ticket_controller::assign))
            .route("/{id}/escalate", post(ticket_controller::escalate))
            .route("/{id}/resolve", post(ticket_controller::resolve))
            .route("/{id}/close", post(ticket_controller::close))
            .route("/{id}/reopen", post(ticket_controller::reopen))
            .route("/{id}/feedback", post(ticket_controller::set_feedback))
            .route("/{id}/comments", get(ticket_controller::list_comments).post(ticket_controller::add_comment))
            .route("/{id}/escalations", get(ticket_controller::get_escalations))
            .route("/{id}/status-history", get(ticket_controller::get_status_history))
    )
}
