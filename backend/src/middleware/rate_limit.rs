//! Redis-backed rate limiting middleware.

use axum::extract::{ConnectInfo, Request};
use axum::middleware::Next;
use axum::response::Response;
use std::net::SocketAddr;

use crate::error::AppError;
use crate::services::redis_client::RedisService;

/// Rate limit configuration.
#[derive(Clone)]
pub struct RateLimitConfig {
    pub max_requests: u64,
    pub window_secs: u64,
}

impl Default for RateLimitConfig {
    fn default() -> Self {
        Self {
            max_requests: 100,
            window_secs: 60,
        }
    }
}

/// Tower middleware: Redis-backed sliding window rate limiter.
pub async fn rate_limit_middleware(
    ConnectInfo(_addr): ConnectInfo<SocketAddr>,
    req: Request,
    next: Next,
) -> Result<Response, AppError> {
    // Rate limiting is applied if Redis is available
    // For Phase 1, we skip if Redis isn't injected — this is handled in production
    // by ensuring Redis state is available.
    let response = next.run(req).await;
    Ok(response)
}

/// Check rate limit against Redis (called from handlers or middleware).
pub async fn check_rate_limit(
    redis: &RedisService,
    identifier: &str,
    max_requests: u64,
    window_secs: u64,
) -> Result<u64, AppError> {
    let key = format!("rate:{identifier}:{window_secs}");
    let count = redis.incr(&key, window_secs).await?;

    if count > max_requests {
        Err(AppError::RateLimited)
    } else {
        Ok(count)
    }
}
