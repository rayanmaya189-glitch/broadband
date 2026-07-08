use axum::{middleware, routing::{get, post}, Router};

use crate::app::SharedState;
use crate::common::middleware::auth_middleware::jwt_middleware;
use crate::modules::ticket::controller::ticket_controller;

pub fn ticket_routes() -> Router<SharedState> {
    Router::new()
        .route("/", get(ticket_controller::list_tickets).post(ticket_controller::create_ticket))
        .route("/dashboard", get(ticket_controller::get_dashboard))
        .route("/my-assignments/{user_id}", get(ticket_controller::get_my_assignments))
        .route("/{id}", get(ticket_controller::get_ticket).put(ticket_controller::update_ticket).delete(ticket_controller::delete_ticket))
        .route("/{id}/assign", post(ticket_controller::assign_ticket))
        .route("/{id}/escalate", post(ticket_controller::escalate_ticket))
        .route("/{id}/resolve", post(ticket_controller::resolve_ticket))
        .route("/{id}/close", post(ticket_controller::close_ticket))
        .route("/{id}/reopen", post(ticket_controller::reopen_ticket))
        .route("/{id}/feedback", post(ticket_controller::set_feedback))
        .route("/{id}/comments", get(ticket_controller::get_comments).post(ticket_controller::add_comment))
        .layer(middleware::from_fn(jwt_middleware))
}
