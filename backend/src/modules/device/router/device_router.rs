use axum::{middleware, routing::{get, post}, Router};
use crate::app::SharedState;
use crate::common::middleware::auth_middleware::jwt_middleware;
use crate::modules::device::controller::device_controller;

pub fn device_routes() -> Router<SharedState> {
    Router::new()
        // ── Device CRUD ──────────────────────────────────
        .route("/", get(device_controller::list_devices).post(device_controller::create_device))
        .route("/models", get(device_controller::list_models).post(device_controller::create_model))
        .route("/:id", get(device_controller::get_device).put(device_controller::update_device).delete(device_controller::delete_device))
        // ── Device Control ────────────────────────────────
        .route("/:id/restart", post(device_controller::restart_device))
        .route("/:id/shutdown", post(device_controller::shutdown_device))
        // ── Ports ─────────────────────────────────────────
        .route("/:id/ports", get(device_controller::list_ports))
        .route("/:id/ports/:port_id", post(device_controller::update_port_status))
        // ── Firmware ──────────────────────────────────────
        .route("/:id/firmware", get(device_controller::list_firmware_updates))
        .route("/:id/firmware/update", post(device_controller::create_firmware_update))
        .route("/firmware/:update_id/status", post(device_controller::update_firmware_status))
        // ── Metrics ───────────────────────────────────────
        .route("/:id/metrics", get(device_controller::get_device_metrics))
        // ── Logs ──────────────────────────────────────────
        .route("/:id/logs", get(device_controller::list_device_logs).post(device_controller::create_device_log))
        .layer(middleware::from_fn(jwt_middleware))
}
