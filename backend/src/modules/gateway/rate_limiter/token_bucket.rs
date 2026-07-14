//! Token bucket rate limiter.
//!
//! Implements the token bucket algorithm for rate limiting.

use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::RwLock;

/// Rate limit configuration.
#[derive(Debug, Clone)]
pub struct RateLimitConfig {
    /// Maximum number of tokens (burst capacity)
    pub max_tokens: u32,
    /// Token refill rate (tokens per second)
    pub refill_rate: f64,
    /// Refill interval in milliseconds
    pub refill_interval_ms: u64,
}

impl Default for RateLimitConfig {
    fn default() -> Self {
        Self {
            max_tokens: 100,
            refill_rate: 10.0,
            refill_interval_ms: 1000,
        }
    }
}

/// Token bucket state.
#[derive(Debug, Clone)]
struct TokenBucket {
    /// Current number of tokens
    tokens: f64,
    /// Maximum tokens
    max_tokens: f64,
    /// Refill rate (tokens per second)
    refill_rate: f64,
    /// Last refill timestamp (Unix timestamp in seconds)
    last_refill: f64,
}

impl TokenBucket {
    /// Try to consume a token.
    fn try_consume(&mut self, now: f64) -> bool {
        // Refill tokens based on elapsed time
        let elapsed = now - self.last_refill;
        let tokens_to_add = elapsed * self.refill_rate;
        self.tokens = (self.tokens + tokens_to_add).min(self.max_tokens);
        self.last_refill = now;

        // Try to consume one token
        if self.tokens >= 1.0 {
            self.tokens -= 1.0;
            true
        } else {
            false
        }
    }
}

/// Rate limiter using token bucket algorithm.
pub struct TokenBucketRateLimiter {
    buckets: Arc<RwLock<HashMap<String, TokenBucket>>>,
    config: RateLimitConfig,
}

impl TokenBucketRateLimiter {
    /// Create a new rate limiter with the given configuration.
    pub fn new(config: RateLimitConfig) -> Self {
        Self {
            buckets: Arc::new(RwLock::new(HashMap::new())),
            config,
        }
    }

    /// Check if a request is allowed for the given key.
    pub async fn check(&self, key: &str) -> bool {
        let now = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap()
            .as_secs_f64();

        let mut buckets = self.buckets.write().await;
        let bucket = buckets.entry(key.to_string()).or_insert_with(|| TokenBucket {
            tokens: self.config.max_tokens as f64,
            max_tokens: self.config.max_tokens as f64,
            refill_rate: self.config.refill_rate,
            last_refill: now,
        });

        bucket.try_consume(now)
    }

    /// Get the current token count for a key.
    pub async fn tokens_remaining(&self, key: &str) -> f64 {
        let now = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap()
            .as_secs_f64();

        let mut buckets = self.buckets.write().await;
        let bucket = buckets.entry(key.to_string()).or_insert_with(|| TokenBucket {
            tokens: self.config.max_tokens as f64,
            max_tokens: self.config.max_tokens as f64,
            refill_rate: self.config.refill_rate,
            last_refill: now,
        });

        // Refill tokens
        let elapsed = now - bucket.last_refill;
        let tokens_to_add = elapsed * bucket.refill_rate;
        bucket.tokens = (bucket.tokens + tokens_to_add).min(bucket.max_tokens);
        bucket.last_refill = now;

        bucket.tokens
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_rate_limiter_allows_requests() {
        let limiter = TokenBucketRateLimiter::new(RateLimitConfig {
            max_tokens: 5,
            refill_rate: 10.0,
            refill_interval_ms: 1000,
        });

        // First 5 requests should be allowed
        for _ in 0..5 {
            assert!(limiter.check("test_key").await);
        }
    }

    #[tokio::test]
    async fn test_rate_limiter_blocks_excess_requests() {
        let limiter = TokenBucketRateLimiter::new(RateLimitConfig {
            max_tokens: 2,
            refill_rate: 0.0, // No refill
            refill_interval_ms: 1000,
        });

        // First 2 requests should be allowed
        assert!(limiter.check("test_key").await);
        assert!(limiter.check("test_key").await);

        // Third request should be blocked
        assert!(!limiter.check("test_key").await);
    }

    #[tokio::test]
    async fn test_rate_limiter_separate_keys() {
        let limiter = TokenBucketRateLimiter::new(RateLimitConfig {
            max_tokens: 1,
            refill_rate: 0.0,
            refill_interval_ms: 1000,
        });

        assert!(limiter.check("key1").await);
        assert!(limiter.check("key2").await);

        // Both keys should be exhausted
        assert!(!limiter.check("key1").await);
        assert!(!limiter.check("key2").await);
    }
}
