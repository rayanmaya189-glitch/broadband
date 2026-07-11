use axum::routing::{get, delete};
use axum::Router;

use crate::app::SharedState;
use crate::common::middleware::rls_setup;
use crate::modules::branch::controller::branch_controller;

pub fn admin_routes() -> Router<SharedState> {
    rls_setup::admin_scoped(
        Router::new()
            .route("/", get(branch_controller::list_branches).post(branch_controller::create_branch))
            .route("/{id}", get(branch_controller::get_branch).put(branch_controller::update_branch).delete(branch_controller::deactivate_branch))
            .route("/{id}/working-hours", get(branch_controller::get_working_hours).put(branch_controller::update_working_hours))
            .route("/{id}/users", get(branch_controller::list_branch_users).post(branch_controller::assign_user))
            .route("/{id}/users/{uid}", delete(branch_controller::remove_user))
            .route("/{id}/stats", get(branch_controller::get_branch_stats))
    )
}
