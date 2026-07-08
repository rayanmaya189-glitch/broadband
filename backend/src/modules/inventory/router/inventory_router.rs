use axum::{middleware, routing::get, Router};
use crate::app::SharedState;
use crate::common::middleware::auth_middleware::jwt_middleware;
use crate::modules::inventory::controller::inventory_controller;

pub fn inventory_routes() -> Router<SharedState> {
    Router::new()
        .route("/", get(inventory_controller::list_items).post(inventory_controller::create_item))
        .route("/:id", get(inventory_controller::get_item).delete(inventory_controller::delete_item))
        .layer(middleware::from_fn(jwt_middleware))
}
