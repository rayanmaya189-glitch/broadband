use axum::middleware;
use axum::routing::{get, post};
use axum::Router;

use crate::app::SharedState;
use crate::common::middleware::auth_middleware::jwt_middleware;
use crate::modules::plan::controller::plan_controller;

pub fn plans_routes() -> Router<SharedState> {
    Router::new()
        .route("/", get(plan_controller::list_plans).post(plan_controller::create_plan))
        .route("/:id", get(plan_controller::get_plan).put(plan_controller::update_plan).delete(plan_controller::delete_plan))
        .route("/:id/publish", post(plan_controller::publish_plan))
        .route("/:id/unpublish", post(plan_controller::unpublish_plan))
        .route("/:id/clone", post(plan_controller::clone_plan))
        .route("/:id/speed-profile", get(plan_controller::get_speed_profile).post(plan_controller::create_speed_profile))
        .route("/:id/speed-profile/delete", post(plan_controller::delete_speed_profile))
        .route("/:id/pricing", get(plan_controller::list_plan_pricing).put(plan_controller::update_plan_pricing))
        .layer(middleware::from_fn(jwt_middleware))
}
