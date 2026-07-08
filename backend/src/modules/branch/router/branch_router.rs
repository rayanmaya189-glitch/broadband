use axum::middleware;
use axum::routing::get;
use axum::Router;

use crate::app::SharedState;
use crate::common::middleware::auth_middleware::jwt_middleware;
use crate::modules::branch::controller::branch_controller;

pub fn branches_routes() -> Router<SharedState> {
    Router::new()
        .route("/", get(branch_controller::list_branches).post(branch_controller::create_branch))
        .route("/:id", get(branch_controller::get_branch).put(branch_controller::update_branch).delete(branch_controller::deactivate_branch))
        .layer(middleware::from_fn(jwt_middleware))
}
