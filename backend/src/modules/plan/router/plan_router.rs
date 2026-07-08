use axum::middleware;
use axum::routing::get;
use axum::Router;

use crate::app::SharedState;
use crate::common::middleware::auth_middleware::jwt_middleware;
use crate::modules::plan::controller::plan_controller;

pub fn plans_routes() -> Router<SharedState> {
    Router::new()
        .route("/", get(plan_controller::list_plans).post(plan_controller::create_plan))
        .route("/:id", get(plan_controller::get_plan).put(plan_controller::update_plan).delete(plan_controller::delete_plan))
        .layer(middleware::from_fn(jwt_middleware))
}
