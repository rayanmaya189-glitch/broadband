use axum::{middleware, routing::{delete, get, post}, Router};
use crate::app::SharedState;
use crate::common::middleware::auth_middleware::jwt_middleware;
use crate::modules::document::controller::document_controller;
pub fn document_routes() -> Router<SharedState> {
    Router::new().route("/", get(document_controller::list_documents)).route("/upload-url", post(document_controller::upload_url)).route("/:id", delete(document_controller::delete_document)).layer(middleware::from_fn(jwt_middleware))
}
