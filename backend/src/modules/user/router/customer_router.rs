use axum::routing::{get, post};
use axum::Router;

use crate::app::SharedState;
use crate::common::middleware::auth_middleware::jwt_middleware;
use crate::modules::user::customer::controller;

pub fn customer_routes() -> Router<SharedState> {
    Router::new()
        .route("/me", get(controller::get_me))
        .route("/me/update", post(controller::update_me))
        .route("/password/change", post(controller::change_password))
        .route("/sessions", get(controller::list_sessions))
        .route("/logout", post(controller::logout))
        .route("/logout/all", post(controller::logout_all))
        .route("/2fa/enable", post(controller::enable_2fa))
        .route("/2fa/confirm", post(controller::confirm_2fa))
        .route("/2fa/disable", post(controller::disable_2fa))
        .layer(axum::middleware::from_fn(jwt_middleware))
}
