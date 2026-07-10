use axum::routing::{get, post, put, delete};
use axum::Router;

use crate::app::SharedState;
use crate::common::middleware::rls_setup;
use crate::modules::customer::controller::customer_controller_seaorm;

pub fn customers_routes() -> Router<SharedState> {
    rls_setup::branch_scoped(
        Router::new()
            // ── Customer CRUD ──────────────────────────────────
            .route("/", get(customer_controller_seaorm::list_customers).post(customer_controller_seaorm::create_customer))
            .route("/:id", get(customer_controller_seaorm::get_customer).put(customer_controller_seaorm::update_customer).delete(customer_controller_seaorm::delete_customer))
            .route("/:id/status", put(customer_controller_seaorm::update_status))
            // ── Customer Profile (KYC) ─────────────────────────
            .route("/:id/profile", get(customer_controller_seaorm::get_profile).put(customer_controller_seaorm::update_profile))
            .route("/:id/kyc/submit", post(customer_controller_seaorm::submit_kyc))
            .route("/:id/kyc/verify", post(customer_controller_seaorm::verify_kyc))
            // ── KYC Documents ──────────────────────────────────
            .route("/:id/kyc/documents", get(customer_controller_seaorm::list_kyc_documents).post(customer_controller_seaorm::upload_kyc_document))
            .route("/:id/kyc/documents/:doc_id", delete(customer_controller_seaorm::delete_kyc_document))
            // ── Customer Addresses ─────────────────────────────
            .route("/:id/addresses", get(customer_controller_seaorm::list_addresses).post(customer_controller_seaorm::create_address))
            .route("/:id/addresses/:addr_id", put(customer_controller_seaorm::update_address).delete(customer_controller_seaorm::delete_address))
    )
}
