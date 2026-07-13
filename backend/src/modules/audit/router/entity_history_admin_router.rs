use axum::routing::{get, post};
use axum::Router;

use crate::app::SharedState;
use crate::common::middleware::rls_setup;
use crate::modules::audit::controller::entity_history_controller;

pub fn entity_history_admin_routes() -> Router<SharedState> {
    rls_setup::admin_scoped(
        Router::new()
            .route("/", get(entity_history_controller::search_history))
            .route("/{id}", get(entity_history_controller::get_history_entry))
            .route("/{id}/rollback", post(entity_history_controller::rollback))
            .route("/entity/{entity_type}/{entity_id}", get(entity_history_controller::get_entity_history))
    )
}
