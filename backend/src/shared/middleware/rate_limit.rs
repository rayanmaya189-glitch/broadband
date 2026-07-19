use axum::extract::Request;
use axum::http::StatusCode;
use axum::middleware::Next;
use axum::response::Response;
use redis::aio::ConnectionManager;
use std::sync::Arc;
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

    pub fn public() -> Self {
        Self {
            max_requests: 30,
            window_seconds: 60,
        }
    }

    pub fn admin_read() -> Self {
        Self {
            max_requests: 200,
            window_seconds: 60,
        }
    }

    pub fn admin_write() -> Self {
        Self {
            max_requests: 100,
            window_seconds: 60,
        }
    }

    pub fn customer_read() -> Self {
        Self {
            max_requests: 50,
            window_seconds: 60,
        }
    }

    pub fn customer_write() -> Self {
        Self {
            max_requests: 20,
            window_seconds: 60,
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

/// Redis-backed rate limiter store using sorted sets (sliding window).
///
/// Key format: `rate:{identifier}:{window}`
/// Score: timestamp in microseconds
/// Value: unique request ID
///
/// This matches the §02-redis.md design exactly.
#[derive(Clone)]
pub struct RateLimitStore {
    redis: ConnectionManager,
    /// In-memory fallback when Redis is unavailable
    fallback: std::sync::Arc<tokio::sync::RwLock<std::collections::HashMap<String, Vec<i64>>>>,
}

impl RateLimitStore {
    pub fn new(redis: ConnectionManager) -> Self {
        Self {
            redis,
            fallback: std::sync::Arc::new(tokio::sync::RwLock::new(
                std::collections::HashMap::new(),
            )),
        }
    }

    /// Check if a request is allowed and record it using Redis sorted sets.
    ///
    /// Implements the sliding window algorithm:
    /// 1. Remove entries outside the window
    /// 2. Count remaining entries
    /// 3. If under limit, add new entry
    /// 4. Set TTL on the key
    pub async fn check_and_record(&self, key: &str, tier: &RateLimitTier) -> RateLimitInfo {
        let window_key = format!("rate:{}:{}s", key, tier.window_seconds);
        let now_micros = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap()
            .as_micros() as i64;
        let window_start = now_micros - (tier.window_seconds as i64 * 1_000_000);
        let request_id = format!("{}:{}", now_micros, rand_id());

        // Try Redis first, fall back to in-memory
        match self
            .try_redis_check(&window_key, now_micros, window_start, &request_id, tier)
            .await
        {
            Some(info) => info,
            None => self.fallback_check(key, tier).await,
        }
    }

    /// Try rate limiting via Redis sorted sets
    async fn try_redis_check(
        &self,
        window_key: &str,
        now_micros: i64,
        window_start: i64,
        request_id: &str,
        tier: &RateLimitTier,
    ) -> Option<RateLimitInfo> {
        let mut conn = self.redis.clone();

        // Atomic Lua script for sliding window rate limiting
        // This ensures correctness even under concurrent requests
        let script = r#"
            local key = KEYS[1]
            local window_start = tonumber(ARGV[1])
            local now = tonumber(ARGV[2])
            local request_id = ARGV[3]
            local max_requests = tonumber(ARGV[4])
            local window_seconds = tonumber(ARGV[5])

            -- Remove expired entries
            redis.call('ZREMRANGEBYSCORE', key, '-inf', window_start)

            -- Count current entries
            local count = redis.call('ZCARD', key)

            if count < max_requests then
                -- Add the new request
                redis.call('ZADD', key, now, request_id)
                -- Set TTL on the key (window + 10s buffer)
                redis.call('EXPIRE', key, window_seconds + 10)
                return {1, max_requests - count - 1, 0}
            else
                -- Rate limited - get the oldest entry to calculate retry_after
                local oldest = redis.call('ZRANGE', key, 0, 0, 'WITHSCORES')
                local retry_after = 0
                if #oldest >= 2 then
                    local oldest_score = tonumber(oldest[2])
                    local expires_at_micros = oldest_score + (window_seconds * 1000000)
                    retry_after = math.ceil((expires_at_micros - now) / 1000000)
                    if retry_after < 1 then
                        retry_after = 1
                    end
                end
                return {0, 0, retry_after}
            end
        "#;

        let result: Result<Vec<i64>, _> = redis::cmd("eval")
            .arg(script)
            .arg(1) // number of KEYS
            .arg(window_key)
            .arg(window_start)
            .arg(now_micros)
            .arg(request_id)
            .arg(tier.max_requests)
            .arg(tier.window_seconds)
            .query_async(&mut conn)
            .await;

        match result {
            Ok(vals) if vals.len() >= 3 => {
                let allowed = vals[0] == 1;
                let remaining = vals[1] as i32;
                let retry_after = if vals[2] > 0 { Some(vals[2]) } else { None };

                Some(RateLimitInfo {
                    allowed,
                    remaining,
                    retry_after,
                })
            }
            _ => {
                // Redis unavailable, fall back to in-memory
                warn!("Redis rate limit check failed, falling back to in-memory");
                None
            }
        }
    }

    /// In-memory fallback when Redis is unavailable
    async fn fallback_check(&self, key: &str, tier: &RateLimitTier) -> RateLimitInfo {
        let mut requests = self.fallback.write().await;
        let now = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap()
            .as_secs() as i64;
        let window_start = now - tier.window_seconds as i64;

        let timestamps = requests.entry(key.to_string()).or_default();

        // Remove expired entries
        timestamps.retain(|ts| *ts > window_start);

        let count = timestamps.len() as i32;
        let allowed = count < tier.max_requests;

        if allowed {
            timestamps.push(now);
        }

        let remaining = (tier.max_requests - timestamps.len() as i32).max(0);
        let retry_after = if !allowed {
            timestamps.first().map(|ts| {
                let expires_at = ts + tier.window_seconds as i64;
                (expires_at - now).max(1)
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

/// Generate a random ID for request deduplication
fn rand_id() -> u64 {
    rand::random::<u64>()
}

/// Middleware function for rate limiting.
///
/// Uses the shared `RateLimitStore` from request extensions (injected via AppState).
pub async fn rate_limit_middleware(request: Request, next: Next) -> Result<Response, StatusCode> {
    // Extract client identifier (IP address or API key)
    let client_id = extract_client_id(&request);
    let path = request.uri().path().to_string();

    // Determine rate limit tier based on path
    let tier = determine_tier(&path, &request);

    debug!(client_id = %client_id, path = %path, tier = ?tier, "Rate limit check");

    // Get the shared store from request extensions
    let store = match request.extensions().get::<Arc<RateLimitStore>>().cloned() {
        Some(store) => store,
        None => {
            tracing::error!(
                "RateLimitStore not found in request extensions. \
                 Ensure rate_limit_layer is applied."
            );
            return Err(StatusCode::INTERNAL_SERVER_ERROR);
        }
    };

    let info = store.check_and_record(&client_id, &tier).await;

    if !info.allowed {
        warn!(
            client_id = %client_id,
            retry_after = ?info.retry_after,
            "Rate limit exceeded"
        );

        let mut response = Response::new(axum::body::Body::from(
            r#"{"error":"rate_limit_exceeded","message":"Too many requests"}"#,
        ));
        *response.status_mut() = StatusCode::TOO_MANY_REQUESTS;
        response
            .headers_mut()
            .insert("content-type", "application/json".parse().unwrap());
        if let Some(retry) = info.retry_after {
            response
                .headers_mut()
                .insert("retry-after", retry.to_string().parse().unwrap());
        }
        response.headers_mut().insert(
            "x-ratelimit-limit",
            tier.max_requests.to_string().parse().unwrap(),
        );
        return Ok(response);
    }

    let mut response = next.run(request).await;

    // Add rate limit headers to response
    response.headers_mut().insert(
        "x-ratelimit-limit",
        tier.max_requests.to_string().parse().unwrap(),
    );
    response.headers_mut().insert(
        "x-ratelimit-remaining",
        info.remaining.to_string().parse().unwrap(),
    );
    if let Some(retry) = info.retry_after {
        response.headers_mut().insert(
            "x-ratelimit-retry-after",
            retry.to_string().parse().unwrap(),
        );
    }

    Ok(response)
}

/// Extract client identifier from request headers
fn extract_client_id(request: &Request) -> String {
    // Prefer X-Forwarded-For, then X-Real-IP, then fallback
    request
        .headers()
        .get("x-forwarded-for")
        .and_then(|v| v.to_str().ok())
        .map(|v| {
            // Take first IP from comma-separated list
            v.split(',').next().unwrap_or(v).trim().to_string()
        })
        .or_else(|| {
            request
                .headers()
                .get("x-real-ip")
                .and_then(|v| v.to_str().ok())
                .map(|s| s.to_string())
        })
        .unwrap_or_else(|| "unknown".to_string())
}

/// Determine rate limit tier based on request path, method, and user role.
fn determine_tier(path: &str, request: &Request) -> RateLimitTier {
    // Health checks and websocket: no rate limit
    if path == "/health" || path == "/ready" || path == "/ws" {
        return RateLimitTier {
            max_requests: i32::MAX,
            window_seconds: 1,
        };
    }

    // Auth endpoints: strict limit regardless of role
    if path.starts_with("/api/v1/auth") {
        return RateLimitTier::auth();
    }

    // Upload endpoints: strict limit
    if path.starts_with("/api/v1/documents") || path.starts_with("/api/v1/notifications") {
        return RateLimitTier::upload();
    }

    // Extract user role from request extensions for role-based limiting
    let role = request
        .extensions()
        .get::<crate::shared::middleware::auth::UserContext>()
        .map(|u| u.role.as_str());

    match role {
        // Admin/staff roles get higher limits
        Some("super_admin" | "admin" | "finance_manager" | "billing_operator") => {
            if request.method().is_safe() {
                RateLimitTier::admin_read()
            } else {
                RateLimitTier::admin_write()
            }
        }
        // Customer role gets lower limits
        Some("customer") => {
            if request.method().is_safe() {
                RateLimitTier::customer_read()
            } else {
                RateLimitTier::customer_write()
            }
        }
        // Field technician, support agent, etc: standard limits
        Some(_) => {
            if request.method().is_safe() {
                RateLimitTier::api_read()
            } else {
                RateLimitTier::api_write()
            }
        }
        // Unauthenticated: standard public limits
        None => {
            if request.method().is_safe() {
                RateLimitTier::api_read()
            } else {
                RateLimitTier::api_write()
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_rate_limit_tiers() {
        let auth = RateLimitTier::auth();
        assert_eq!(auth.max_requests, 5);
        assert_eq!(auth.window_seconds, 60);

        let api_read = RateLimitTier::api_read();
        assert_eq!(api_read.max_requests, 100);

        let api_write = RateLimitTier::api_write();
        assert_eq!(api_write.max_requests, 30);

        let upload = RateLimitTier::upload();
        assert_eq!(upload.max_requests, 10);

        let public = RateLimitTier::public();
        assert_eq!(public.max_requests, 30);

        let admin_read = RateLimitTier::admin_read();
        assert_eq!(admin_read.max_requests, 200);

        let admin_write = RateLimitTier::admin_write();
        assert_eq!(admin_write.max_requests, 100);

        let customer_read = RateLimitTier::customer_read();
        assert_eq!(customer_read.max_requests, 50);

        let customer_write = RateLimitTier::customer_write();
        assert_eq!(customer_write.max_requests, 20);
    }

    #[test]
    fn test_extract_client_id_prefers_forwarded_for() {
        let mut request = Request::builder()
            .uri("/api/v1/customers")
            .body(axum::body::Body::empty())
            .unwrap();
        request
            .headers_mut()
            .insert("x-forwarded-for", "10.0.0.1, 10.0.0.2".parse().unwrap());
        assert_eq!(extract_client_id(&request), "10.0.0.1");
    }

    #[test]
    fn test_extract_client_id_falls_back_to_real_ip() {
        let mut request = Request::builder()
            .uri("/api/v1/customers")
            .body(axum::body::Body::empty())
            .unwrap();
        request
            .headers_mut()
            .insert("x-real-ip", "192.168.1.1".parse().unwrap());
        assert_eq!(extract_client_id(&request), "192.168.1.1");
    }

    #[test]
    fn test_extract_client_id_unknown() {
        let request = Request::builder()
            .uri("/api/v1/customers")
            .body(axum::body::Body::empty())
            .unwrap();
        assert_eq!(extract_client_id(&request), "unknown");
    }

    #[test]
    fn test_determine_tier_auth() {
        let request = Request::builder()
            .uri("/api/v1/auth/login")
            .body(axum::body::Body::empty())
            .unwrap();
        let tier = determine_tier("/api/v1/auth/login", &request);
        assert_eq!(tier.max_requests, 5);
    }

    #[test]
    fn test_determine_tier_health_no_limit() {
        let request = Request::builder()
            .uri("/health")
            .body(axum::body::Body::empty())
            .unwrap();
        let tier = determine_tier("/health", &request);
        assert_eq!(tier.max_requests, i32::MAX);
    }
}
