use axum::http::StatusCode;
use axum::response::{IntoResponse, Response};
use serde_json::json;

/// Unified error type for the AeroXe backend.
#[derive(Debug, thiserror::Error)]
pub enum AppError {
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

    #[error("Bad request: {0}")]
    BadRequest(String),

    #[error("Internal error: {0}")]
    Internal(#[from] anyhow::Error),

    #[error("External service error: {0}")]
    External(String),

    #[error("Rate limited")]
    RateLimited,

    #[error("Account locked")]
    AccountLocked,

    #[error("Two-factor authentication required")]
    TwoFactorRequired,
}

impl From<sea_orm::DbErr> for AppError {
    fn from(err: sea_orm::DbErr) -> Self {
        match err {
            sea_orm::DbErr::RecordNotFound(s) => AppError::NotFound(s),
            _ => {
                tracing::error!("Database error: {:?}", err);
                AppError::Internal(anyhow::anyhow!("Database error"))
            }
        }
    }
}

/// Detect a Postgres unique-constraint violation (SQLSTATE 23505) inside a
/// sea_orm error chain. Used for DB-backed idempotency, where a concurrent
/// duplicate insert must be treated as an already-processed success.
pub fn is_unique_violation(err: &sea_orm::DbErr) -> bool {
    match err {
        sea_orm::DbErr::Exec(sea_orm::RuntimeErr::SqlxError(sea_orm::sqlx::Error::Database(
            db,
        ))) => db.code().map(|c| c == "23505").unwrap_or(false),
        _ => false,
    }
}

impl IntoResponse for AppError {
    fn into_response(self) -> Response {
        let (status, error_message) = match &self {
            AppError::NotFound(msg) => (StatusCode::NOT_FOUND, msg.clone()),
            AppError::Unauthorized => (StatusCode::UNAUTHORIZED, "Unauthorized".to_string()),
            AppError::Forbidden(msg) => (StatusCode::FORBIDDEN, msg.clone()),
            AppError::Validation(msg) => (StatusCode::BAD_REQUEST, msg.clone()),
            AppError::Conflict(msg) => (StatusCode::CONFLICT, msg.clone()),
            AppError::BadRequest(msg) => (StatusCode::BAD_REQUEST, msg.clone()),
            AppError::Internal(err) => {
                tracing::error!("Internal error: {:?}", err);
                (
                    StatusCode::INTERNAL_SERVER_ERROR,
                    "Internal server error".to_string(),
                )
            }
            AppError::External(msg) => (
                StatusCode::BAD_GATEWAY,
                format!("External service error: {}", msg),
            ),
            AppError::RateLimited => (
                StatusCode::TOO_MANY_REQUESTS,
                "Rate limit exceeded".to_string(),
            ),
            AppError::AccountLocked => (StatusCode::FORBIDDEN, "Account is locked".to_string()),
            AppError::TwoFactorRequired => (
                StatusCode::OK,
                "Two-factor authentication required".to_string(),
            ),
        };

        let body = json!({
            "error": error_message,
            "status": status.as_u16(),
        });

        (status, axum::Json(body)).into_response()
    }
}
