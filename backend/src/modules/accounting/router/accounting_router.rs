use axum::{middleware, routing::{get, post}, Router};
use crate::app::SharedState;
use crate::common::middleware::auth_middleware::jwt_middleware;
use crate::modules::accounting::controller::accounting_controller;
pub fn accounting_routes() -> Router<SharedState> {
    Router::new().route("/accounts", get(accounting_controller::list_accounts).post(accounting_controller::create_account)).route("/journal", get(accounting_controller::list_journal).post(accounting_controller::create_journal)).route("/journal/:id/post", post(accounting_controller::post_journal)).route("/journal/:id/void", post(accounting_controller::void_journal)).layer(middleware::from_fn(jwt_middleware))
}
