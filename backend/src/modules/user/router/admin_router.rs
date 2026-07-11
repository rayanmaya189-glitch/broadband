use axum::routing::{get, put};
use axum::Router;

use crate::app::SharedState;
use crate::common::middleware::rls_setup;
use crate::modules::user::admin::controller;

pub fn admin_routes() -> Router<SharedState> {
    rls_setup::admin_scoped(
        Router::new()
            .route("/", get(controller::list_users).post(controller::create_user))
            .route("/{id}", get(controller::get_user).put(controller::update_user).delete(controller::delete_user))
            .route("/{id}/status", put(controller::update_user_status))
    )
}
