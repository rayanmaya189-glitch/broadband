use axum::{middleware, routing::{get, post}, Router};

use crate::app::SharedState;
use crate::common::middleware::auth_middleware::jwt_middleware;
use crate::modules::billing::controller::billing_controller;

pub fn billing_routes() -> Router<SharedState> {
    Router::new()
        // ── Invoices ──────────────────────────────────────
        .route("/invoices", get(billing_controller::list_invoices).post(billing_controller::create_invoice))
        .route("/invoices/:id", get(billing_controller::get_invoice))
        .route("/invoices/:id/line-items", get(billing_controller::get_line_items))
        .route("/invoices/:id/send", post(billing_controller::send_invoice))
        .route("/invoices/:id/void", post(billing_controller::void_invoice))
        .route("/invoices/:id/review", post(billing_controller::review_invoice))
        // ── Payments ──────────────────────────────────────
        .route("/payments", get(billing_controller::list_payments).post(billing_controller::record_payment))
        // ── Refunds ───────────────────────────────────────
        .route("/refunds", post(billing_controller::request_refund))
        .route("/refunds/:id/approve", post(billing_controller::approve_refund))
        // ── Discounts ─────────────────────────────────────
        .route("/discounts", get(billing_controller::list_discounts).post(billing_controller::create_discount))
        // ── Config ────────────────────────────────────────
        .route("/dunning/config", get(billing_controller::get_dunning_config).put(billing_controller::update_dunning_config))
        .route("/tax/config", get(billing_controller::get_tax_config).put(billing_controller::update_tax_config))
        .layer(middleware::from_fn(jwt_middleware))
}
