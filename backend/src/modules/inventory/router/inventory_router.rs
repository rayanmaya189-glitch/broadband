use axum::{routing::{get, post}, Router};
use crate::app::SharedState;
use crate::common::middleware::rls_setup;
use crate::modules::inventory::controller::inventory_controller;

pub fn inventory_routes() -> Router<SharedState> {
    rls_setup::branch_scoped(
        Router::new()
            .route("/", get(inventory_controller::list_items).post(inventory_controller::create_item))
            .route("/reports", get(inventory_controller::get_report))
            .route("/alerts", get(inventory_controller::get_warranty_alerts))
            .route("/:id", get(inventory_controller::get_item).delete(inventory_controller::delete_item))
            .route("/:id/assign", post(inventory_controller::assign_item))
            .route("/:id/install", post(inventory_controller::install_item))
            .route("/:id/return", post(inventory_controller::return_item))
            .route("/:id/transfer", post(inventory_controller::transfer_item))
            .route("/:id/scrap", post(inventory_controller::scrap_item))
            .route("/:id/movements", get(inventory_controller::list_movements))
    )
}
