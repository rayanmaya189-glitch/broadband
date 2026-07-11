use axum::routing::{get, post};
use axum::Router;

use crate::app::SharedState;
use crate::common::middleware::rls_setup;
use crate::modules::role::controller::role_controller;

pub fn admin_routes() -> Router<SharedState> {
    rls_setup::admin_scoped(
        Router::new()
            .route("/", get(role_controller::list_roles).post(role_controller::create_role))
            .route("/{id}", get(role_controller::get_role).put(role_controller::update_role).delete(role_controller::deactivate_role))
            .route("/{id}/permissions", post(role_controller::assign_permissions).delete(role_controller::remove_permission))
            .route("/user/{uid}/roles", get(role_controller::list_user_roles).post(role_controller::assign_role_to_user).delete(role_controller::revoke_role_from_user))
    )
}
