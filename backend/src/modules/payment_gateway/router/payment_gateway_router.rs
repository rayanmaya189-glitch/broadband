use axum::{middleware, routing::{get, post, put}, Router};
use crate::app::SharedState;
use crate::common::middleware::auth_middleware::jwt_middleware;
use crate::modules::payment_gateway::controller::payment_gateway_controller;

pub fn payment_gateway_routes() -> Router<SharedState> {
    Router::new()
        .route("/gateways", get(payment_gateway_controller::list_gateways).post(payment_gateway_controller::create_gateway))
        .route("/gateways/:id", put(payment_gateway_controller::update_gateway))
        .route("/create-link", post(payment_gateway_controller::create_payment_link))
        .route("/", get(payment_gateway_controller::list_transactions))
        .route("/retry", post(payment_gateway_controller::retry_payment))
        .layer(middleware::from_fn(jwt_middleware))
}

/// Webhook routes without JWT middleware (signature verification only)
pub fn payment_webhook_routes() -> Router<SharedState> {
    Router::new()
        .route("/razorpay", post(payment_gateway_controller::process_webhook_razorpay))
        .route("/payu", post(payment_gateway_controller::process_webhook_payu))
        .route("/instamojo", post(payment_gateway_controller::process_webhook_instamojo))
}
