use axum::{middleware, routing::{get, post}, Router};
use crate::app::SharedState;
use crate::common::middleware::auth_middleware::jwt_middleware;
use crate::modules::accounting::controller::accounting_controller;

pub fn accounting_routes() -> Router<SharedState> {
    Router::new()
        .route("/accounts", get(accounting_controller::list_accounts).post(accounting_controller::create_account))
        .route("/journal", get(accounting_controller::list_journal).post(accounting_controller::create_journal))
        .route("/journal/:id/lines", get(accounting_controller::get_entry_lines))
        .route("/journal/:id/post", post(accounting_controller::post_journal))
        .route("/journal/:id/void", post(accounting_controller::void_journal))
        .route("/trial-balance", get(accounting_controller::trial_balance))
        .route("/profit-loss", get(accounting_controller::profit_loss))
        .route("/balance-sheet", get(accounting_controller::balance_sheet))
        .route("/cash-flow", get(accounting_controller::cash_flow))
        .route("/gst/:return_type", get(accounting_controller::gst_return_data))
        .layer(middleware::from_fn(jwt_middleware))
}
