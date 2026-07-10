use axum::{routing::{get, post}, Router};

use crate::app::SharedState;
use crate::common::middleware::rls_setup;
use crate::modules::billing::controller::billing_controller_seaorm;

pub fn billing_routes() -> Router<SharedState> {
    rls_setup::branch_scoped(
        Router::new()
            // ── Invoices ──────────────────────────────────────
            .route("/invoices", get(billing_controller_seaorm::list_invoices).post(billing_controller_seaorm::create_invoice))
            .route("/invoices/:id", get(billing_controller_seaorm::get_invoice))
            .route("/invoices/:id/line-items", get(billing_controller_seaorm::get_line_items))
            .route("/invoices/:id/send", post(billing_controller_seaorm::send_invoice))
            .route("/invoices/:id/void", post(billing_controller_seaorm::void_invoice))
            .route("/invoices/:id/review", post(billing_controller_seaorm::review_invoice))
            // ── Payments ──────────────────────────────────────
            .route("/payments", get(billing_controller_seaorm::list_payments).post(billing_controller_seaorm::record_payment))
            // ── Refunds ───────────────────────────────────────
            .route("/refunds", post(billing_controller_seaorm::request_refund))
            .route("/refunds/:id/approve", post(billing_controller_seaorm::approve_refund))
            // ── Discounts ─────────────────────────────────────
            .route("/discounts", get(billing_controller_seaorm::list_discounts).post(billing_controller_seaorm::create_discount))
            // ── Config ────────────────────────────────────────
            .route("/dunning/config", get(billing_controller_seaorm::get_dunning_config).put(billing_controller_seaorm::update_dunning_config))
            .route("/tax/config", get(billing_controller_seaorm::get_tax_config).put(billing_controller_seaorm::update_tax_config))
    )
}
