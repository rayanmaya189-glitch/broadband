use axum::{routing::{get, post}, Router};

use crate::app::SharedState;
use crate::common::middleware::rls_setup;
use crate::modules::ticket::controller::ticket_controller;

pub fn ticket_routes() -> Router<SharedState> {
    rls_setup::branch_scoped(
        Router::new()
            .route("/", get(ticket_controller::list).post(ticket_controller::create))
            .route("/{id}", get(ticket_controller::get_by_id))
            .route("/{id}/status", post(ticket_controller::update_status))
            .route("/{id}/assign", post(ticket_controller::assign))
            .route("/{id}/comments", get(ticket_controller::list_comments).post(ticket_controller::add_comment))
    )
}
