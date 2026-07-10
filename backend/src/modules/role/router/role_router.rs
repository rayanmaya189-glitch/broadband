use axum::middleware;
use axum::routing::{get, post};
use axum::Router;

use crate::app::SharedState;
use crate::common::middleware::auth_middleware::jwt_middleware;
use crate::modules::role::controller::role_controller_seaorm;

pub fn roles_routes() -> Router<SharedState> {
    Router::new()
        .route("/", get(role_controller_seaorm::list_roles).post(role_controller_seaorm::create_role))
        .route("/:id", get(role_controller_seaorm::get_role).put(role_controller_seaorm::update_role).delete(role_controller_seaorm::deactivate_role))
        .route("/:id/permissions", post(role_controller_seaorm::assign_permissions).delete(role_controller_seaorm::remove_permission))
        .route("/user/:uid/roles", get(role_controller_seaorm::list_user_roles).post(role_controller_seaorm::assign_role_to_user).delete(role_controller_seaorm::revoke_role_from_user))
        .layer(middleware::from_fn(jwt_middleware))
}
