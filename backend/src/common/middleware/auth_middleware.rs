use axum::extract::{FromRequestParts, Request};
use axum::http::header;
use axum::http::request::Parts;
use axum::middleware::Next;
use axum::response::Response;
use jsonwebtoken::{decode, Algorithm, DecodingKey, Validation};
use serde::{Deserialize, Serialize};

use crate::common::config::config::Config;
use crate::common::errors::app_error::AppError;

/// JWT claims structure.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Claims {
    pub sub: i64,
    pub email: String,
    pub role: String,
    pub role_id: i64,
    pub branch_id: Option<i64>,
    pub is_company_wide: bool,
    pub permissions: Vec<String>,
    pub jti: String,
    pub exp: usize,
    pub iat: usize,
}

/// Current user context extracted from a valid JWT.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UserContext {
    pub user_id: i64,
    pub email: String,
    pub role: String,
    pub branch_id: Option<i64>,
    pub is_company_wide: bool,
    pub permissions: Vec<String>,
    pub jti: String,
}

/// Branch filter injected by the branch-scoping middleware.
#[derive(Debug, Clone)]
pub struct BranchFilter {
    pub branch_ids: Vec<i64>,
    pub is_company_wide: bool,
}

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

impl<S> FromRequestParts<S> for Claims
where
    S: Send + Sync,
{
    type Rejection = AppError;

    async fn from_request_parts(parts: &mut Parts, _state: &S) -> Result<Self, Self::Rejection> {
        parts
            .extensions
            .get::<Claims>()
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
        jti: claims.jti.clone(),
    };

    req.extensions_mut().insert(user_ctx);
    req.extensions_mut().insert(claims);

    Ok(next.run(req).await)
}
