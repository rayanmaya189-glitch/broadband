use axum::extract::Request;
use axum::http::StatusCode;
use axum::middleware::Next;
use axum::response::Response;
use chrono::{DateTime, Duration, Utc};
use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::RwLock;
use tracing::{debug, warn};

/// Rate limit tier configuration
#[derive(Debug, Clone)]
pub struct RateLimitTier {
    pub max_requests: i32,
    pub window_seconds: i32,
}

impl RateLimitTier {
    pub fn auth() -> Self {
        Self {
            max_requests: 5,
            window_seconds: 60,
        }
    }

    pub fn api_read() -> Self {
        Self {
            max_requests: 100,
            window_seconds: 60,
        }
    }

    pub fn api_write() -> Self {
        Self {
            max_requests: 30,
            window_seconds: 60,
        }
    }

    pub fn upload() -> Self {
        Self {
            max_requests: 10,
            window_seconds: 300,
        }
    }
}

/// Rate limit info to include in response headers
#[derive(Debug, Clone)]
pub struct RateLimitInfo {
    pub allowed: bool,
    pub remaining: i32,
    pub retry_after: Option<i64>,
}


/// In-memory rate limiter store (shared across all requests via AppState)
#[derive(Clone)]
pub struct RateLimitStore {
    requests: Arc<RwLock<HashMap<String, Vec<DateTime<Utc>>>>>,
}

impl RateLimitStore {
    pub fn new() -> Self {
        Self {
            requests: Arc::new(RwLock::new(HashMap::new())),
        }
    }

    /// Check if a request is allowed and record it
    pub async fn check_and_record(&self, key: &str, tier: &RateLimitTier) -> RateLimitInfo {
        let mut requests = self.requests.write().await;
        let now = Utc::now();
        let window_start = now - Duration::seconds(tier.window_seconds as i64);

        // Get or create entry for this key
        let timestamps = requests.entry(key.to_string()).or_default();

        // Remove expired entries
        timestamps.retain(|ts| *ts > window_start);

        let count = timestamps.len() as i32;
        let allowed = count < tier.max_requests;

        if allowed {
            timestamps.push(now);
        }

        let remaining = (tier.max_requests - count).max(0);
        let retry_after = if !allowed {
            timestamps.first().map(|ts| {
                let expires_at = *ts + Duration::seconds(tier.window_seconds as i64);
                (expires_at - now).num_seconds().max(1)
            })
        } else {
            None
        };

        RateLimitInfo {
            allowed,
            remaining,
            retry_after,
        }
    }
}

impl Default for RateLimitStore {
    fn default() -> Self {
        Self::new()
    }
}

/// Middleware function for rate limiting.
///
/// Uses the shared `RateLimitStore` from request extensions (injected via AppState).
pub async fn rate_limit_middleware(
    request: Request,
    next: Next,
) -> Result<Response, StatusCode> {
    // Extract client identifier (IP address or API key)
    let client_id = request
        .headers()
        .get("x-forwarded-for")
        .or_else(|| request.headers().get("x-real-ip"))
        .and_then(|v| v.to_str().ok())
        .unwrap_or("unknown")
        .to_string();

    let path = request.uri().path().to_string();

    // Determine rate limit tier based on path
    let tier = if path.starts_with("/api/auth") {
        RateLimitTier::auth()
    } else if path.starts_with("/api/upload") {
        RateLimitTier::upload()
    } else if request.method().is_safe() {
        RateLimitTier::api_read()
    } else {
        RateLimitTier::api_write()
    };

    debug!(client_id = %client_id, path = %path, tier = ?tier, "Rate limit check");

    // Get the shared store from request extensions (set by AppState injection)
    let store = request
        .extensions()
        .get::<Arc<RateLimitStore>>()
        .cloned()
        .unwrap_or_default();

    let info = store.check_and_record(&client_id, &tier).await;

    if !info.allowed {
        warn!(client_id = %client_id, retry_after = ?info.retry_after, "Rate limit exceeded");
        return Err(StatusCode::TOO_MANY_REQUESTS);
    }

    let mut response = next.run(request).await;

    // Add rate limit headers to response
    if let Ok(val) = info.remaining.to_string().parse() {
        response.headers_mut().insert("x-ratelimit-remaining", val);
    }
    if let Some(retry) = info.retry_after {
        if let Ok(val) = retry.to_string().parse() {
            response.headers_mut().insert("x-ratelimit-retry-after", val);
        }
    }

    Ok(response)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_rate_limit_tiers() {
        let auth = RateLimitTier::auth();
        assert_eq!(auth.max_requests, 5);
        assert_eq!(auth.window_seconds, 60);

        let api_read = RateLimitTier::api_read();
        assert_eq!(api_read.max_requests, 100);

        let api_write = RateLimitTier::api_write();
        assert_eq!(api_write.max_requests, 30);
    }

    #[tokio::test]
    async fn test_rate_limit_store() {
        let store = RateLimitStore::new();
        let tier = RateLimitTier {
            max_requests: 2,
            window_seconds: 60,
        };

        // First request should be allowed
        let info1 = store.check_and_record("test-key", &tier).await;
        assert!(info1.allowed);
        assert_eq!(info1.remaining, 1);

        // Second request should be allowed
        let info2 = store.check_and_record("test-key", &tier).await;
        assert!(info2.allowed);
        assert_eq!(info2.remaining, 0);

        // Third request should be blocked
        let info3 = store.check_and_record("test-key", &tier).await;
        assert!(!info3.allowed);
        assert!(info3.retry_after.is_some());
    }
}
