use axum::routing::{get, post, put};
use axum::Router;

use crate::app::SharedState;
use crate::common::middleware::rls_setup;
use crate::modules::document::controller::document_controller;

pub fn admin_routes() -> Router<SharedState> {
    rls_setup::admin_scoped(
        Router::new()
            .route("/", get(document_controller::list).post(document_controller::upload_url))
            .route("/{id}", get(document_controller::get_by_id).delete(document_controller::soft_delete))
            .route("/{id}/confirm", post(document_controller::confirm_upload))
            .route("/{id}/associate", put(document_controller::associate_entity))
            .route("/{id}/access-logs", get(document_controller::get_access_logs))
    )
}
