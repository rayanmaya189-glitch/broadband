use axum::middleware;
use axum::routing::{delete, get};
use axum::Router;

use crate::app::SharedState;
use crate::common::middleware::auth_middleware::jwt_middleware;
use crate::modules::permission::controller::permission_controller;

pub fn permissions_routes() -> Router<SharedState> {
    Router::new()
        .route("/", get(permission_controller::list_permissions).post(permission_controller::create_permission))
        .route("/:id", delete(permission_controller::delete_permission))
        .layer(middleware::from_fn(jwt_middleware))
}
