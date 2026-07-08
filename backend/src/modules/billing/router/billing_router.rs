use axum::{middleware, routing::{get, post}, Router};

use crate::app::SharedState;
use crate::common::middleware::auth_middleware::jwt_middleware;
use crate::modules::billing::controller::billing_controller;

pub fn billing_routes() -> Router<SharedState> {
    Router::new()
        .route("/invoices", get(billing_controller::list_invoices).post(billing_controller::create_invoice))
        .route("/invoices/:id", get(billing_controller::get_invoice))
        .route("/payments", get(billing_controller::list_payments).post(billing_controller::record_payment))
        .route("/refunds", post(billing_controller::request_refund))
        .route("/refunds/:id/approve", post(billing_controller::approve_refund))
        .route("/discounts", get(billing_controller::list_discounts).post(billing_controller::create_discount))
        .layer(middleware::from_fn(jwt_middleware))
}
