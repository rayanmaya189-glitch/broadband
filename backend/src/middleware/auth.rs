//! JWT authentication middleware — token extraction and validation.

use axum::extract::{FromRequestParts, Request};
use axum::http::header;
use axum::http::request::Parts;
use axum::middleware::Next;
use axum::response::Response;
use jsonwebtoken::{decode, Algorithm, DecodingKey, Validation};
use serde::{Deserialize, Serialize};

use crate::config::Config;
use crate::error::AppError;
use crate::shared::types::UserContext;

/// JWT claims structure matching the architecture spec.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Claims {
    pub sub: i64,
    pub email: String,
    pub role: String,
    pub branch_id: Option<i64>,
    pub is_company_wide: bool,
    pub permissions: Vec<String>,
    pub jti: String,
    pub exp: usize,
    pub iat: usize,
}

/// Extract the `UserContext` from a request (axum extractor).
///
/// Use in handler signatures: `user: UserContext`
impl<S> FromRequestParts<S> for UserContext
where
    S: Send + Sync,
{
    type Rejection = AppError;

    async fn from_request_parts(parts: &mut Parts, _state: &S) -> Result<Self, Self::Rejection> {
        parts
            .extensions
            .get::<UserContext>()
            .cloned()
            .ok_or(AppError::Unauthorized)
    }
}

/// Tower middleware: extract JWT from Authorization header, validate it,
/// and insert `UserContext` + `Claims` into request extensions.
pub async fn jwt_middleware(mut req: Request, next: Next) -> Result<Response, AppError> {
    let auth_header = req
        .headers()
        .get(header::AUTHORIZATION)
        .and_then(|v| v.to_str().ok())
        .ok_or(AppError::Unauthorized)?;

    let token = auth_header
        .strip_prefix("Bearer ")
        .ok_or(AppError::Unauthorized)?;

    let config = Config::get();
    let mut validation = Validation::new(Algorithm::HS256);
    validation.validate_exp = true;

    let token_data = decode::<Claims>(
        token,
        &DecodingKey::from_secret(config.jwt_secret.as_bytes()),
        &validation,
    )
    .map_err(|e| {
        tracing::warn!(error = %e, "JWT validation failed");
        AppError::Unauthorized
    })?;

    let claims = token_data.claims;

    let user_ctx = UserContext {
        user_id: claims.sub,
        email: claims.email.clone(),
        role: claims.role.clone(),
        branch_id: claims.branch_id,
        is_company_wide: claims.is_company_wide,
        permissions: claims.permissions.clone(),
    };

    req.extensions_mut().insert(user_ctx);
    req.extensions_mut().insert(claims);

    Ok(next.run(req).await)
}
