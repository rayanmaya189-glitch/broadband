use axum::routing::{get, put};
use axum::Router;

use crate::app::SharedState;
use crate::common::middleware::rls_setup;
use crate::modules::referral::admin::controller;

pub fn admin_routes() -> Router<SharedState> {
    rls_setup::admin_scoped(
        Router::new()
            .route("/programs", get(controller::list_programs).post(controller::create_program))
            .route("/programs/{id}", put(controller::update_program))
            .route("/stats/{referrer_id}", get(controller::get_referral_stats))
    )
}
