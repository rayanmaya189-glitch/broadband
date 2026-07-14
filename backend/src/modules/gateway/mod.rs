//! API Gateway module.
//!
//! Provides cross-cutting API concerns: authentication, rate limiting,
//! request validation, and API versioning.

pub mod auth;
pub mod rate_limiter;
pub mod request_validator;
pub mod api_versioning;
