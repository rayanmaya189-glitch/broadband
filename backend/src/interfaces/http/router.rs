//! Router assembly — builds the full Axum application router.

use axum::middleware as axum_mw;
use axum::routing::get;
use axum::Router;

use crate::app::SharedState;
use crate::interfaces::http::health;
use crate::middleware::{auth, audit, branch_scope, cors};

/// Build the application router with all routes and middleware layers.
pub fn create_router(state: SharedState) -> Router {
    // ── Health routes (no auth) ─────────────────────────────
    let health_routes = Router::new()
        .route("/health", get(health::liveness))
        .route("/health/ready", get(health::readiness));

    // ── API v1 routes ───────────────────────────────────────
    let api_v1 = Router::new()
        // Auth (public — no JWT)
        .route("/api/v1/auth/login", get(|| async { "TODO: login" }))
        .route("/api/v1/auth/register", get(|| async { "TODO: register" }))
        .route("/api/v1/auth/refresh", get(|| async { "TODO: refresh" }))
        .route("/api/v1/auth/otp/send", get(|| async { "TODO: OTP send" }))
        .route("/api/v1/auth/otp/verify", get(|| async { "TODO: OTP verify" }))

        // Auth (authenticated)
        .route("/api/v1/auth/logout", get(|| async { "TODO: logout" }))
        .route("/api/v1/auth/sessions", get(|| async { "TODO: sessions" }))

        // Users (authenticated)
        .route("/api/v1/users", get(|| async { "TODO: list users" }))
        .route("/api/v1/users/me", get(|| async { "TODO: current user" }))

        // Branches (authenticated)
        .route("/api/v1/branches", get(|| async { "TODO: list branches" }))

        // Customers (authenticated)
        .route("/api/v1/customers", get(|| async { "TODO: list customers" }))

        // Plans (authenticated)
        .route("/api/v1/plans", get(|| async { "TODO: list plans" }))

        // Billing (authenticated)
        .route("/api/v1/billing/invoices", get(|| async { "TODO: list invoices" }))
        .route("/api/v1/billing/payments", get(|| async { "TODO: list payments" }))

        // Network (authenticated)
        .route("/api/v1/network/vlans", get(|| async { "TODO: list VLANs" }))
        .route("/api/v1/network/ip-pools", get(|| async { "TODO: list IP pools" }))

        // Devices (authenticated)
        .route("/api/v1/devices", get(|| async { "TODO: list devices" }))

        // Tickets (authenticated)
        .route("/api/v1/tickets", get(|| async { "TODO: list tickets" }))

        // Admin dashboard
        .route("/api/v1/admin/dashboard", get(|| async { "TODO: dashboard" }))

        // ── Protected routes layer (JWT + audit + branch scope) ──
        .layer(axum_mw::from_fn(branch_scope::branch_scope_middleware))
        .layer(axum_mw::from_fn(audit::audit_middleware))
        .layer(axum_mw::from_fn(auth::jwt_middleware));

    // ── Assemble full router ────────────────────────────────
    health_routes
        .merge(api_v1)
        .layer(cors::build_cors())
        .with_state(state)
}
