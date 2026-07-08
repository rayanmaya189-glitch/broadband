use axum::{middleware, routing::get, Router};
use crate::app::SharedState;
use crate::common::middleware::auth_middleware::jwt_middleware;
use crate::modules::bandwidth::controller::bandwidth_controller;

pub fn bandwidth_routes() -> Router<SharedState> {
    Router::new()
        .route("/profiles", get(bandwidth_controller::list_profiles).post(bandwidth_controller::create_profile))
        .route("/profiles/:id", get(bandwidth_controller::get_profile).put(bandwidth_controller::update_profile).delete(bandwidth_controller::delete_profile))
        .layer(middleware::from_fn(jwt_middleware))
}
