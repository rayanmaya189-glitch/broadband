use axum::routing::get;
use axum::Router;

use crate::app::SharedState;
use crate::common::middleware::rls_setup;
use crate::modules::realtime::controller::realtime_controller;

pub fn admin_routes() -> Router<SharedState> {
    rls_setup::admin_scoped(
        Router::new()
            .route("/health", get(realtime_controller::health))
            .route("/channels", get(realtime_controller::channels))
            .route("/stats", get(realtime_controller::stats))
    )
}
