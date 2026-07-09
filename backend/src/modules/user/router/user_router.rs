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
        .route("/login/otp/send", post(user_controller::send_otp))
        .route("/login/otp/verify", post(user_controller::verify_otp))
        .route("/register", post(user_controller::register))
        .route("/refresh", post(user_controller::refresh_token))
        .route("/password/reset/request", post(user_controller::request_password_reset))
        .route("/password/reset/confirm", post(user_controller::confirm_password_reset));

    let protected = Router::new()
        .route("/logout", post(user_controller::logout))
        .route("/logout/all", post(user_controller::logout_all))
        .route("/me", get(user_controller::me))
        .route("/password/change", post(user_controller::change_password))
        .route("/sessions", get(user_controller::list_sessions))
        .route("/2fa/enable", post(user_controller::enable_2fa))
        .route("/2fa/confirm", post(user_controller::confirm_2fa))
        .route("/2fa/disable", post(user_controller::disable_2fa))
        .layer(middleware::from_fn(jwt_middleware));

    // 2FA login verification (public — requires temp_token, not JWT)
    let public_2fa = Router::new()
        .route("/2fa/verify", post(user_controller::verify_2fa_login));

    public.merge(protected).merge(public_2fa)
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
