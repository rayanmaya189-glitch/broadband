use axum::routing::{get, put};
use axum::Router;

use crate::app::SharedState;
use crate::common::middleware::rls_setup;
use crate::modules::payment_gateway::controller::payment_gateway_controller;

pub fn admin_routes() -> Router<SharedState> {
    rls_setup::admin_scoped(
        Router::new()
            .route("/gateways", get(payment_gateway_controller::list_gateways).post(payment_gateway_controller::create_gateway))
            .route("/gateways/{id}", put(payment_gateway_controller::update_gateway))
            .route("/transactions", get(payment_gateway_controller::list_transactions).post(payment_gateway_controller::create_transaction))
            .route("/transactions/{id}", get(payment_gateway_controller::get_transaction))
    )
}
