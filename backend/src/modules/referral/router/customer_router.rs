use axum::routing::get;
use axum::Router;

use crate::app::SharedState;
use crate::common::middleware::auth_middleware::jwt_middleware;
use crate::modules::referral::customer::controller;

pub fn customer_routes() -> Router<SharedState> {
    Router::new()
        .route("/info", get(controller::get_my_referral_info))
        .route("/stats", get(controller::get_my_stats))
        .route("/wallet", get(controller::get_my_wallet))
        .route("/transactions", get(controller::get_my_transactions))
        .layer(axum::middleware::from_fn(jwt_middleware))
}
