use axum::{routing::{get, post}, Router};
use crate::app::SharedState;
use crate::common::middleware::rls_setup;
use crate::modules::bandwidth::controller::bandwidth_controller;

pub fn bandwidth_routes() -> Router<SharedState> {
    rls_setup::branch_scoped(
        Router::new()
            // ── Profiles ──────────────────────────────────────
            .route("/profiles", get(bandwidth_controller::list_profiles).post(bandwidth_controller::create_profile))
            .route("/profiles/:id", get(bandwidth_controller::get_profile).put(bandwidth_controller::update_profile).delete(bandwidth_controller::delete_profile))
            .route("/profiles/:id/apply", post(bandwidth_controller::apply_to_subscription))
            // ── Applications ───────────────────────────────────
            .route("/applications", get(bandwidth_controller::list_applications))
            // ── Usage ──────────────────────────────────────────
            .route("/usage/:subscription_id", get(bandwidth_controller::get_usage))
    )
}
