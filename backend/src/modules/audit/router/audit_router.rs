use axum::{middleware, routing::get, Router};
use crate::app::SharedState;
use crate::common::middleware::auth_middleware::jwt_middleware;
use crate::modules::audit::controller::audit_controller;

pub fn audit_routes() -> Router<SharedState> {
    Router::new()
        .route("/logs", get(audit_controller::list_logs).post(audit_controller::export_logs))
        .route("/logs/:id", get(audit_controller::get_log))
        .route("/user/:user_id", get(audit_controller::get_user_activity))
        .route("/resource/:resource_type/:resource_id", get(audit_controller::get_resource_history))
        .layer(middleware::from_fn(jwt_middleware))
}
