use axum::{middleware, routing::{get, post}, Router};
use crate::app::SharedState;
use crate::common::middleware::auth_middleware::jwt_middleware;
use crate::modules::notification::controller::notification_controller;
pub fn notification_routes() -> Router<SharedState> {
    Router::new().route("/templates", get(notification_controller::list_templates).post(notification_controller::create_template)).route("/send", post(notification_controller::send_notification)).layer(middleware::from_fn(jwt_middleware))
}
