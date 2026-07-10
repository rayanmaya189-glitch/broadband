use axum::middleware;
use axum::routing::{get, post};
use axum::Router;

use crate::app::SharedState;
use crate::common::middleware::auth_middleware::jwt_middleware;
use crate::modules::plan::controller::plan_controller_seaorm;

pub fn plans_routes() -> Router<SharedState> {
    Router::new()
        .route("/", get(plan_controller_seaorm::list_plans).post(plan_controller_seaorm::create_plan))
        .route("/:id", get(plan_controller_seaorm::get_plan).put(plan_controller_seaorm::update_plan).delete(plan_controller_seaorm::delete_plan))
        .route("/:id/publish", post(plan_controller_seaorm::publish_plan))
        .route("/:id/unpublish", post(plan_controller_seaorm::unpublish_plan))
        .route("/:id/clone", post(plan_controller_seaorm::clone_plan))
        .route("/:id/speed-profile", get(plan_controller_seaorm::get_speed_profile).post(plan_controller_seaorm::create_speed_profile))
        .route("/:id/speed-profile/delete", post(plan_controller_seaorm::delete_speed_profile))
        .route("/:id/pricing", get(plan_controller_seaorm::list_plan_pricing).put(plan_controller_seaorm::update_plan_pricing))
        .layer(middleware::from_fn(jwt_middleware))
}
