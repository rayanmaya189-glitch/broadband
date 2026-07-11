use axum::{middleware, routing::{get, post, put}, Router};
use crate::app::SharedState;
use crate::common::middleware::auth_middleware::jwt_middleware;
use crate::modules::referral::controller::referral_controller;

pub fn referral_routes() -> Router<SharedState> {
    Router::new()
        .route("/programs", get(referral_controller::list_programs).post(referral_controller::create_program))
        .route("/programs/:id", put(referral_controller::update_program))
        .route("/tracking", get(referral_controller::list_tracking).post(referral_controller::share_referral))
        .route("/stats/:referrer_id", get(referral_controller::get_stats))
        .route("/wallet/:customer_id", get(referral_controller::get_wallet).post(referral_controller::get_or_create_wallet))
        .route("/wallet/:customer_id/credit", post(referral_controller::credit_wallet))
        .route("/wallet/:customer_id/debit", post(referral_controller::debit_wallet))
        .route("/wallet/:customer_id/transactions", get(referral_controller::list_wallet_transactions))
        .layer(middleware::from_fn(jwt_middleware))
}
