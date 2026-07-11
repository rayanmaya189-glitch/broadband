use axum::{middleware, routing::get, Router};
use crate::app::SharedState;
use crate::common::middleware::auth_middleware::jwt_middleware;
use crate::modules::payment_gateway::controller::payment_gateway_controller;

pub fn payment_gateway_routes() -> Router<SharedState> {
    Router::new()
        .route("/gateways", get(payment_gateway_controller::list_gateways).post(payment_gateway_controller::create_gateway))
        .route("/", get(payment_gateway_controller::list_transactions).post(payment_gateway_controller::create_transaction))
        .layer(middleware::from_fn(jwt_middleware))
}

/// Webhook routes without JWT middleware (signature verification only)
pub fn payment_webhook_routes() -> Router<SharedState> {
    Router::new()
}
