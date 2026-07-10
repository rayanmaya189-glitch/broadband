use axum::middleware;
use axum::routing::{get, post};
use axum::Router;

use crate::app::SharedState;
use crate::common::middleware::auth_middleware::jwt_middleware;
use crate::modules::role::controller::role_controller;

pub fn roles_routes() -> Router<SharedState> {
    Router::new()
        .route("/", get(role_controller::list_roles).post(role_controller::create_role))
        .route("/:id", get(role_controller::get_role).put(role_controller::update_role).delete(role_controller::deactivate_role))
        .route("/:id/permissions", post(role_controller::assign_permissions).delete(role_controller::remove_permission))
        .route("/user/:uid/roles", get(role_controller::list_user_roles).post(role_controller::assign_role_to_user).delete(role_controller::revoke_role_from_user))
        .layer(middleware::from_fn(jwt_middleware))
}
