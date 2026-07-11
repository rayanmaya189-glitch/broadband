use axum::{routing::{get, post}, Router};
use crate::app::SharedState;
use crate::common::middleware::rls_setup;
use crate::modules::inventory::controller::inventory_controller;

pub fn inventory_routes() -> Router<SharedState> {
    rls_setup::branch_scoped(
        Router::new()
            .route("/", get(inventory_controller::list).post(inventory_controller::create))
            .route("/:id/status", post(inventory_controller::update_status))
            .route("/:id/assign", post(inventory_controller::assign))
            .route("/:id/install", post(inventory_controller::install))
            .route("/:id/return", post(inventory_controller::return_item))
            .route("/:id/transfer", post(inventory_controller::transfer))
            .route("/:id/scrap", post(inventory_controller::scrap))
    )
}
