use axum::{middleware, routing::{get, post}, Router};
use crate::app::SharedState;
use crate::common::middleware::auth_middleware::jwt_middleware;
use crate::modules::discovery::controller::discovery_controller;

pub fn discovery_routes() -> Router<SharedState> {
    Router::new()
        // ── Scans ──────────────────────────────────────────
        .route("/scans", get(discovery_controller::list_scans).post(discovery_controller::create_scan))
        .route("/scans/:id/start", post(discovery_controller::start_scan))
        .route("/scans/:id/stop", post(discovery_controller::stop_scan))
        // ── Results ─────────────────────────────────────────
        .route("/results", get(discovery_controller::list_results))
        .route("/results/:id/approve", post(discovery_controller::approve_result))
        .route("/results/:id/reject", post(discovery_controller::reject_result))
        // ── Dashboard ──────────────────────────────────────
        .route("/dashboard", get(discovery_controller::dashboard))
        .layer(middleware::from_fn(jwt_middleware))
}
