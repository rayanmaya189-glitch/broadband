use axum::{routing::get, Router};
use crate::app::SharedState;
use crate::common::middleware::rls_setup;
use crate::modules::traffic::controller::traffic_controller;

pub fn traffic_routes() -> Router<SharedState> {
    rls_setup::admin_branch_scoped(
        Router::new()
            .route("/samples", get(traffic_controller::list_samples).post(traffic_controller::record_sample))
            .route("/policies", get(traffic_controller::list_policies).post(traffic_controller::create_policy))
            .route("/aggregates", get(traffic_controller::list_aggregates))
    )
}
