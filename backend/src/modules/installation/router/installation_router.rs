use axum::{routing::{get, put}, Router};
use crate::app::SharedState;
use crate::common::middleware::rls_setup;
use crate::modules::installation::controller::installation_controller;

pub fn installation_routes() -> Router<SharedState> {
    rls_setup::branch_scoped(
        Router::new()
            .route("/", get(installation_controller::list).post(installation_controller::create))
            .route("/my-assignments", get(installation_controller::get_my_assignments))
            .route("/:id", get(installation_controller::get_by_id))
            .route("/:id/schedule", put(installation_controller::schedule))
            .route("/:id/start", put(installation_controller::start))
            .route("/:id/complete", put(installation_controller::complete))
            .route("/:id/cancel", put(installation_controller::cancel))
    )
}
