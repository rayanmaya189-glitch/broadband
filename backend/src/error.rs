use axum::http::StatusCode;
use axum::response::{IntoResponse, Response};
use axum::Json;
use serde_json::json;

/// Unified application error type shared across all modules.
///
/// Every variant maps to an appropriate HTTP status code.
#[derive(Debug, thiserror::Error)]
pub enum AppError {
    // ── Client errors (4xx) ─────────────────────────────────
    #[error("Not found: {0}")]
    NotFound(String),

    #[error("Unauthorized")]
    Unauthorized,

    #[error("Forbidden: {0}")]
    Forbidden(String),

    #[error("Validation error: {0}")]
    Validation(String),

    #[error("Conflict: {0}")]
    Conflict(String),

    #[error("Rate limited")]
    RateLimited,

    #[error("Request timeout")]
    Timeout,

    // ── Server errors (5xx) ─────────────────────────────────
    #[error("Internal server error")]
    Internal(#[from] anyhow::Error),

    #[error("Database error")]
    Database(#[from] sqlx::Error),

    #[error("External service error: {0}")]
    External(String),
}

impl From<validator::ValidationErrors> for AppError {
    fn from(errs: validator::ValidationErrors) -> Self {
        Self::Validation(errs.to_string())
    }
}

impl IntoResponse for AppError {
    fn into_response(self) -> Response {
        let (status, message) = match &self {
            Self::NotFound(m) => (StatusCode::NOT_FOUND, m.clone()),
            Self::Unauthorized => (StatusCode::UNAUTHORIZED, "Unauthorized".into()),
            Self::Forbidden(m) => (StatusCode::FORBIDDEN, m.clone()),
            Self::Validation(m) => (StatusCode::BAD_REQUEST, m.clone()),
            Self::Conflict(m) => (StatusCode::CONFLICT, m.clone()),
            Self::RateLimited => (StatusCode::TOO_MANY_REQUESTS, "Rate limited".into()),
            Self::Timeout => (StatusCode::REQUEST_TIMEOUT, "Request timeout".into()),
            Self::Internal(e) => {
                tracing::error!(error = %e, "Internal server error");
                (
                    StatusCode::INTERNAL_SERVER_ERROR,
                    "Internal server error".into(),
                )
            }
            Self::Database(e) => {
                tracing::error!(error = %e, "Database error");
                (
                    StatusCode::INTERNAL_SERVER_ERROR,
                    "Internal server error".into(),
                )
            }
            Self::External(m) => {
                tracing::error!(message = %m, "External service error");
                (
                    StatusCode::BAD_GATEWAY,
                    "External service error".into(),
                )
            }
        };

        (status, Json(json!({ "error": message }))).into_response()
    }
}

/// Convenience type alias for handler return values.
pub type AppResult<T> = Result<T, AppError>;
