use redis::aio::ConnectionManager;
use redis::AsyncCommands;
use crate::shared::errors::AppError;

/// Redis-based sliding window rate limiter.
pub struct RateLimiter {
    redis: ConnectionManager,
}

impl RateLimiter {
    pub fn new(redis: ConnectionManager) -> Self {
        Self { redis }
    }

    /// Check if a request is allowed under the rate limit.
    /// Returns (allowed, remaining_requests, retry_after_seconds).
    pub async fn check_rate_limit(
        &self,
        key: &str,
        max_requests: i32,
        window_seconds: i32,
    ) -> Result<(bool, i32, Option<i64>), AppError> {
        let mut redis = self.redis.clone();
        let now = chrono::Utc::now().timestamp();
        let window_start = now - window_seconds as i64;
        let redis_key = format!("rate_limit:{}", key);

        // Use a sorted set with timestamps as scores for sliding window
        // Remove old entries outside the window
        let _: () = redis
            .zremrangebyscore(&redis_key, 0, window_start)
            .await
            .map_err(|e| AppError::Internal(anyhow::anyhow!("Redis error: {}", e)))?;

        // Count current requests in window
        let count: i64 = redis
            .zcard(&redis_key)
            .await
            .map_err(|e| AppError::Internal(anyhow::anyhow!("Redis error: {}", e)))?;

        if count >= max_requests as i64 {
            // Get the oldest entry to calculate retry_after
            let oldest: Option<(String, f64)> = redis
                .zrange_withscores(&redis_key, 0, 0)
                .await
                .map_err(|e| AppError::Internal(anyhow::anyhow!("Redis error: {}", e)))?;

            let retry_after = oldest
                .and_then(|v| v.into_iter().next())
                .map(|(_, score)| (score as i64 + window_seconds as i64) - now)
                .unwrap_or(window_seconds as i64);

            return Ok((false, 0, Some(retry_after.max(1))));
        }

        // Add current request
        let _: () = redis
            .zadd(&redis_key, format!("{}", now), now)
            .await
            .map_err(|e| AppError::Internal(anyhow::anyhow!("Redis error: {}", e)))?;

        // Set expiry on the key
        let _: () = redis
            .expire(&redis_key, window_seconds as i64)
            .await
            .map_err(|e| AppError::Internal(anyhow::anyhow!("Redis error: {}", e)))?;

        let remaining = max_requests - (count as i32 + 1);
        Ok((true, remaining, None))
    }
}
