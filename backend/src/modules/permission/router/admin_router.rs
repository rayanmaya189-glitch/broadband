use axum::routing::{delete, get};
use axum::Router;

use crate::app::SharedState;
use crate::common::middleware::rls_setup;
use crate::modules::permission::controller::permission_controller;

pub fn admin_routes() -> Router<SharedState> {
    rls_setup::admin_scoped(
        Router::new()
            .route("/", get(permission_controller::list_permissions).post(permission_controller::create_permission))
            .route("/{id}", delete(permission_controller::delete_permission))
    )
}
