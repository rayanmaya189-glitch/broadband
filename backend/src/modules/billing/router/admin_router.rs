use axum::routing::{get, post};
use axum::Router;

use crate::app::SharedState;
use crate::common::middleware::rls_setup;
use crate::modules::billing::admin::controller;

pub fn admin_routes() -> Router<SharedState> {
    rls_setup::admin_branch_scoped(
        Router::new()
            .route("/invoices", get(controller::list_invoices).post(controller::create_invoice))
            .route("/invoices/{id}", get(controller::get_invoice))
            .route("/invoices/{id}/line-items", get(controller::get_line_items))
            .route("/invoices/{id}/send", post(controller::send_invoice))
            .route("/invoices/{id}/void", post(controller::void_invoice))
            .route("/payments", get(controller::list_payments).post(controller::record_payment))
            .route("/refunds", post(controller::request_refund))
            .route("/refunds/{id}/approve", post(controller::approve_refund))
            .route("/discounts", get(controller::list_discounts).post(controller::create_discount))
            .route("/dunning/config", get(controller::get_dunning_config).put(controller::update_dunning_config))
            .route("/tax/config", get(controller::get_tax_config).put(controller::update_tax_config))
    )
}
