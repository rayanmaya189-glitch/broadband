use axum::{middleware, routing::get, Router};
use crate::app::SharedState;
use crate::common::middleware::auth_middleware::jwt_middleware;
use crate::modules::device::controller::device_controller;

pub fn device_routes() -> Router<SharedState> {
    Router::new()
        .route("/", get(device_controller::list_devices).post(device_controller::create_device))
        .route("/models", get(device_controller::list_models).post(device_controller::create_model))
        .route("/:id", get(device_controller::get_device).put(device_controller::update_device).delete(device_controller::delete_device))
        .layer(middleware::from_fn(jwt_middleware))
}
