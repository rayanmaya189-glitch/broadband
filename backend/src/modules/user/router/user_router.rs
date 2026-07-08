use axum::middleware;
use axum::routing::{get, post};
use axum::Router;

use crate::app::SharedState;
use crate::common::middleware::auth_middleware::jwt_middleware;
use crate::modules::user::controller::user_controller;

/// Build auth routes — public routes have no JWT layer.
pub fn auth_routes() -> Router<SharedState> {
    let public = Router::new()
        .route("/login", post(user_controller::login))
        .route("/register", post(user_controller::register))
        .route("/refresh", post(user_controller::refresh_token));

    let protected = Router::new()
        .route("/logout", post(user_controller::logout))
        .route("/logout/all", post(user_controller::logout_all))
        .route("/me", get(user_controller::me))
        .route("/password/change", post(user_controller::change_password))
        .route("/sessions", get(user_controller::list_sessions))
        .layer(middleware::from_fn(jwt_middleware));

    public.merge(protected)
}

/// Build users routes — all require authentication.
pub fn users_routes() -> Router<SharedState> {
    Router::new()
        .route("/", get(user_controller::list_users).post(user_controller::create_user))
        .route(
            "/:id",
            get(user_controller::get_user)
                .put(user_controller::update_user)
                .delete(user_controller::delete_user),
        )
        .route("/:id/status", axum::routing::put(user_controller::update_user_status))
        .route("/me", get(user_controller::get_me).put(user_controller::update_me))
        .layer(middleware::from_fn(jwt_middleware))
}
