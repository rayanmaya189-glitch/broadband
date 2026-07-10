use axum::middleware;
use axum::routing::{get, delete};
use axum::Router;

use crate::app::SharedState;
use crate::common::middleware::auth_middleware::jwt_middleware;
use crate::modules::branch::controller::branch_controller_seaorm;

pub fn branches_routes() -> Router<SharedState> {
    Router::new()
        .route("/", get(branch_controller_seaorm::list_branches).post(branch_controller_seaorm::create_branch))
        .route("/:id", get(branch_controller_seaorm::get_branch).put(branch_controller_seaorm::update_branch).delete(branch_controller_seaorm::deactivate_branch))
        .route("/:id/working-hours", get(branch_controller_seaorm::get_working_hours).put(branch_controller_seaorm::update_working_hours))
        .route("/:id/users", get(branch_controller_seaorm::list_branch_users).post(branch_controller_seaorm::assign_user))
        .route("/:id/users/:uid", delete(branch_controller_seaorm::remove_user))
        .route("/:id/stats", get(branch_controller_seaorm::get_branch_stats))
        .layer(middleware::from_fn(jwt_middleware))
}
