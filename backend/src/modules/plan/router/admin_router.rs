use axum::routing::{get, post};
use axum::Router;

use crate::app::SharedState;
use crate::common::middleware::rls_setup;
use crate::modules::plan::admin::controller;

pub fn admin_routes() -> Router<SharedState> {
    rls_setup::admin_scoped(
        Router::new()
            .route("/", get(controller::list_plans).post(controller::create_plan))
            .route("/{id}", get(controller::get_plan).put(controller::update_plan).delete(controller::delete_plan))
            .route("/{id}/publish", post(controller::publish_plan))
            .route("/{id}/unpublish", post(controller::unpublish_plan))
            .route("/{id}/clone", post(controller::clone_plan))
            .route("/{id}/speed-profile", get(controller::get_speed_profile).post(controller::create_speed_profile))
            .route("/{id}/speed-profile/delete", post(controller::delete_speed_profile))
            .route("/{id}/pricing", get(controller::list_plan_pricing).put(controller::update_plan_pricing))
    )
}
