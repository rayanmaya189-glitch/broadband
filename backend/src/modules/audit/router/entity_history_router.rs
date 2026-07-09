use axum::{middleware, routing::{get, post}, Router};
use crate::app::SharedState;
use crate::common::middleware::auth_middleware::jwt_middleware;
use crate::modules::audit::controller::entity_history_controller;

pub fn entity_history_routes() -> Router<SharedState> {
    Router::new()
        .route("/", get(entity_history_controller::search_history))
        .route("/stats", get(entity_history_controller::get_stats))
        .route("/rollback", post(entity_history_controller::rollback))
        .route("/:id", get(entity_history_controller::get_history_entry))
        .route("/entity/:entity_type/:entity_id", get(entity_history_controller::get_entity_history))
        .layer(middleware::from_fn(jwt_middleware))
}
