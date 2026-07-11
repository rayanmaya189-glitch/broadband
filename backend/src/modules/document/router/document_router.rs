use axum::{middleware, routing::{get, post, put}, Router};
use crate::app::SharedState;
use crate::common::middleware::auth_middleware::jwt_middleware;
use crate::modules::document::controller::document_controller;

pub fn document_routes() -> Router<SharedState> {
    Router::new()
        .route("/", get(document_controller::list).post(document_controller::upload_url))
        .route("/:id", get(document_controller::get_by_id).delete(document_controller::soft_delete))
        .route("/:id/confirm", post(document_controller::confirm_upload))
        .route("/:id/associate", put(document_controller::associate_entity))
        .route("/:id/access-logs", get(document_controller::get_access_logs))
        .layer(middleware::from_fn(jwt_middleware))
}
