use axum::{routing::{get, post}, Router};
use crate::app::SharedState;
use crate::common::middleware::rls_setup;
use crate::modules::device::controller::device_controller_seaorm;

pub fn device_routes() -> Router<SharedState> {
    rls_setup::branch_scoped(
        Router::new()
            // ── Device CRUD ──────────────────────────────────
            .route("/", get(device_controller_seaorm::list_devices).post(device_controller_seaorm::create_device))
            .route("/models", get(device_controller_seaorm::list_models).post(device_controller_seaorm::create_model))
            .route("/:id", get(device_controller_seaorm::get_device).put(device_controller_seaorm::update_device).delete(device_controller_seaorm::delete_device))
            // ── Device Control ────────────────────────────────
            .route("/:id/restart", post(device_controller_seaorm::restart_device))
            .route("/:id/shutdown", post(device_controller_seaorm::shutdown_device))
            // ── Ports ─────────────────────────────────────────
            .route("/:id/ports", get(device_controller_seaorm::list_ports))
            .route("/:id/ports/:port_id", post(device_controller_seaorm::update_port_status))
            // ── Firmware ──────────────────────────────────────
            .route("/:id/firmware", get(device_controller_seaorm::list_firmware_updates))
            .route("/:id/firmware/update", post(device_controller_seaorm::create_firmware_update))
            .route("/firmware/:update_id/status", post(device_controller_seaorm::update_firmware_status))
            // ── Metrics ───────────────────────────────────────
            .route("/:id/metrics", get(device_controller_seaorm::get_device_metrics))
            // ── Logs ──────────────────────────────────────────
            .route("/:id/logs", get(device_controller_seaorm::list_device_logs).post(device_controller_seaorm::create_device_log))
    )
}
