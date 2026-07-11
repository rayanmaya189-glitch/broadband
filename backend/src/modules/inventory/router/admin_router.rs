use axum::routing::{get, post};
use axum::Router;

use crate::app::SharedState;
use crate::common::middleware::rls_setup;
use crate::modules::inventory::controller::inventory_controller;

pub fn admin_routes() -> Router<SharedState> {
    rls_setup::admin_branch_scoped(
        Router::new()
            .route("/", get(inventory_controller::list).post(inventory_controller::create))
            .route("/reports", get(inventory_controller::get_report))
            .route("/alerts", get(inventory_controller::get_warranty_alerts))
            .route("/{id}", get(inventory_controller::get_by_id).delete(inventory_controller::delete))
            .route("/{id}/status", post(inventory_controller::update_status))
            .route("/{id}/assign", post(inventory_controller::assign))
            .route("/{id}/install", post(inventory_controller::install))
            .route("/{id}/return", post(inventory_controller::return_item))
            .route("/{id}/transfer", post(inventory_controller::transfer))
            .route("/{id}/scrap", post(inventory_controller::scrap))
            .route("/{id}/movements", get(inventory_controller::list_movements))
    )
}
