use crate::common::errors::app_error::AppError;
use crate::common::cache::redis::RedisService;

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

/// Check rate limit against Redis.
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
