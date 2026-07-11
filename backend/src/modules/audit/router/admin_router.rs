use axum::routing::get;
use axum::Router;

use crate::app::SharedState;
use crate::common::middleware::rls_setup;
use crate::modules::audit::controller::audit_controller;

pub fn admin_routes() -> Router<SharedState> {
    rls_setup::admin_scoped(
        Router::new()
            .route("/logs", get(audit_controller::list_logs))
            .route("/logs/{id}", get(audit_controller::get_log))
    )
}
