use axum::{middleware, routing::get, Router};
use crate::app::SharedState;
use crate::common::middleware::auth_middleware::jwt_middleware;
use crate::modules::referral::controller::referral_controller;

pub fn referral_routes() -> Router<SharedState> {
    Router::new()
        .route("/programs", get(referral_controller::list_programs).post(referral_controller::create_program))
        .route("/wallet/:customer_id", get(referral_controller::get_wallet))
        .layer(middleware::from_fn(jwt_middleware))
}
