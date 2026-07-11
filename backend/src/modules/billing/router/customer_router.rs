use axum::routing::{get, post};
use axum::Router;

use crate::app::SharedState;
use crate::common::middleware::auth_middleware::jwt_middleware;
use crate::modules::billing::customer::controller;

pub fn customer_routes() -> Router<SharedState> {
    Router::new()
        .route("/invoices", get(controller::get_my_invoices))
        .route("/invoices/{id}", get(controller::get_invoice))
        .route("/invoices/{id}/line-items", get(controller::get_line_items))
        .route("/payments", get(controller::get_my_payments))
        .route("/pay", post(controller::make_payment))
        .layer(axum::middleware::from_fn(jwt_middleware))
}
