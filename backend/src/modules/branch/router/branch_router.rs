use axum::middleware;
use axum::routing::{get, delete};
use axum::Router;

use crate::app::SharedState;
use crate::common::middleware::auth_middleware::jwt_middleware;
use crate::modules::branch::controller::branch_controller;

pub fn branches_routes() -> Router<SharedState> {
    Router::new()
        .route("/", get(branch_controller::list_branches).post(branch_controller::create_branch))
        .route("/:id", get(branch_controller::get_branch).put(branch_controller::update_branch).delete(branch_controller::deactivate_branch))
        .route("/:id/working-hours", get(branch_controller::get_working_hours).put(branch_controller::update_working_hours))
        .route("/:id/users", get(branch_controller::list_branch_users).post(branch_controller::assign_user))
        .route("/:id/users/:uid", delete(branch_controller::remove_user))
        .route("/:id/stats", get(branch_controller::get_branch_stats))
        .layer(middleware::from_fn(jwt_middleware))
}
