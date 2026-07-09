use axum::{middleware, routing::{get, post, put}, Router};
use crate::app::SharedState;
use crate::common::middleware::auth_middleware::jwt_middleware;
use crate::modules::installation::controller::installation_controller;

pub fn installation_routes() -> Router<SharedState> {
    Router::new()
        .route("/", get(installation_controller::list_installations).post(installation_controller::create_installation))
        .route("/my-assignments", get(installation_controller::get_my_assignments))
        .route("/:id", get(installation_controller::get_installation))
        .route("/:id/schedule", put(installation_controller::schedule_installation))
        .route("/:id/reschedule", put(installation_controller::reschedule_installation))
        .route("/:id/start", put(installation_controller::start_installation))
        .route("/:id/complete", put(installation_controller::complete_installation))
        .route("/:id/cancel", put(installation_controller::cancel_installation))
        .route("/:id/photos", post(installation_controller::upload_photo))
        .layer(middleware::from_fn(jwt_middleware))
}
