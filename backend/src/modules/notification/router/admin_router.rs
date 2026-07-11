use axum::routing::{get, post, put};
use axum::Router;

use crate::app::SharedState;
use crate::common::middleware::rls_setup;
use crate::modules::notification::controller::notification_controller;

pub fn admin_routes() -> Router<SharedState> {
    rls_setup::admin_scoped(
        Router::new()
            .route("/templates", get(notification_controller::list_templates).post(notification_controller::create_template))
            .route("/templates/{id}", put(notification_controller::update_template).delete(notification_controller::delete_template))
            .route("/channels", get(notification_controller::list_channels).post(notification_controller::upsert_channel))
            .route("/", get(notification_controller::list_notifications))
            .route("/send", post(notification_controller::send))
            .route("/{id}/retry", post(notification_controller::retry_notification))
            .route("/history", get(notification_controller::list_history))
    )
}
