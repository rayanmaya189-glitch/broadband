use axum::middleware;
use axum::routing::get;
use axum::Router;

use crate::app::SharedState;
use crate::common::middleware::auth_middleware::jwt_middleware;
use crate::modules::role::controller::role_controller;

pub fn roles_routes() -> Router<SharedState> {
    Router::new()
        .route("/", get(role_controller::list_roles).post(role_controller::create_role))
        .route("/:id", get(role_controller::get_role).put(role_controller::update_role).delete(role_controller::deactivate_role))
        .layer(middleware::from_fn(jwt_middleware))
}
