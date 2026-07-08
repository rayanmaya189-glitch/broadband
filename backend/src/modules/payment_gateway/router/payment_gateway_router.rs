use axum::{middleware, routing::{get, post}, Router};
use crate::app::SharedState;
use crate::common::middleware::auth_middleware::jwt_middleware;
use crate::modules::payment_gateway::controller::payment_gateway_controller;

pub fn payment_gateway_routes() -> Router<SharedState> {
    Router::new()
        .route("/gateways", get(payment_gateway_controller::list_gateways).post(payment_gateway_controller::create_gateway))
        .route("/create-link", post(payment_gateway_controller::create_payment_link))
        .layer(middleware::from_fn(jwt_middleware))
}
