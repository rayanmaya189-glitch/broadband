/// Security alerting middleware per §28 Security Design.
/// Monitors request patterns and triggers alerts for suspicious activity.
use axum::http::{Method, StatusCode};
use axum::response::Response;
use chrono::Utc;
use redis::AsyncCommands;
use std::future::Future;
use std::pin::Pin;
use std::sync::Arc;
use tower::{Layer, Service};

use crate::shared::app_state::AppState;

/// Redis key patterns for security counters
const SECURITY_COUNTER_PREFIX: &str = "aeroxe:security:";
const ALERT_THRESHOLDS: SecurityThresholds = SecurityThresholds {
    failed_logins_per_minute: 5,
    forbidden_per_minute: 10,
    rate_limit_hits_per_minute: 20,
};

struct SecurityThresholds {
    failed_logins_per_minute: u32,
    forbidden_per_minute: u32,
    rate_limit_hits_per_minute: u32,
}

/// Layer that adds security alerting to requests.
#[derive(Clone)]
pub struct SecurityAlertLayer {
    state: Arc<AppState>,
}

impl SecurityAlertLayer {
    pub fn new(state: Arc<AppState>) -> Self {
        Self { state }
    }
}

impl<S> Layer<S> for SecurityAlertLayer {
    type Service = SecurityAlertService<S>;

    fn layer(&self, inner: S) -> Self::Service {
        SecurityAlertService {
            inner,
            state: self.state.clone(),
        }
    }
}

#[derive(Clone)]
pub struct SecurityAlertService<S> {
    inner: S,
    state: Arc<AppState>,
}

impl<S, ReqBody> Service<axum::http::Request<ReqBody>> for SecurityAlertService<S>
where
    S: Service<axum::http::Request<ReqBody>, Response = Response> + Send + Clone + 'static,
    S::Future: Send,
    ReqBody: Send + 'static,
{
    type Response = S::Response;
    type Error = S::Error;
    type Future = Pin<Box<dyn Future<Output = Result<Self::Response, Self::Error>> + Send>>;

    fn poll_ready(
        &mut self,
        cx: &mut std::task::Context<'_>,
    ) -> std::task::Poll<Result<(), Self::Error>> {
        self.inner.poll_ready(cx)
    }

    fn call(&mut self, req: axum::http::Request<ReqBody>) -> Self::Future {
        let mut inner = self.inner.clone();
        let state = self.state.clone();
        let method = req.method().clone();
        let path = req.uri().path().to_string();

        Box::pin(async move {
            let response = inner.call(req).await?;
            let status = response.status();

            // Monitor suspicious patterns (non-blocking)
            let _ = check_security_patterns(&state, &method, &path, status).await;

            Ok(response)
        })
    }
}

/// Check request patterns and trigger alerts if thresholds exceeded.
async fn check_security_patterns(
    state: &Arc<AppState>,
    method: &Method,
    path: &str,
    status: StatusCode,
) -> Result<(), ()> {
    let mut redis = state.redis.clone();
    let now = Utc::now().timestamp();
    let _window_key = format!("{}window:{}", SECURITY_COUNTER_PREFIX, now / 60);

    // Track forbidden responses (potential unauthorized access attempts)
    if status == StatusCode::FORBIDDEN {
        let key = format!("{}forbidden:{}", SECURITY_COUNTER_PREFIX, now / 60);
        let count: u32 = redis.incr(&key, 1u32).await.unwrap_or(0);
        let _: () = redis.expire(&key, 120).await.unwrap_or(());

        if count >= ALERT_THRESHOLDS.forbidden_per_minute {
            tracing::error!(
                count = count,
                path = %path,
                method = %method,
                "SECURITY ALERT: Excessive 403 Forbidden responses detected"
            );
            // In production: send to alerting system (PagerDuty, Slack, etc.)
        }
    }

    // Track unauthorized responses
    if status == StatusCode::UNAUTHORIZED {
        let key = format!("{}unauthorized:{}", SECURITY_COUNTER_PREFIX, now / 60);
        let count: u32 = redis.incr(&key, 1u32).await.unwrap_or(0);
        let _: () = redis.expire(&key, 120).await.unwrap_or(());

        if count >= ALERT_THRESHOLDS.failed_logins_per_minute {
            tracing::error!(
                count = count,
                path = %path,
                method = %method,
                "SECURITY ALERT: Excessive 401 Unauthorized responses detected"
            );
        }
    }

    // Track rate limit hits (potential DoS)
    if status == StatusCode::TOO_MANY_REQUESTS {
        let key = format!("{}ratelimit:{}", SECURITY_COUNTER_PREFIX, now / 60);
        let count: u32 = redis.incr(&key, 1u32).await.unwrap_or(0);
        let _: () = redis.expire(&key, 120).await.unwrap_or(());

        if count >= ALERT_THRESHOLDS.rate_limit_hits_per_minute {
            tracing::warn!(
                count = count,
                path = %path,
                method = %method,
                "SECURITY ALERT: Excessive rate limiting triggered"
            );
        }
    }

    Ok(())
}

/// Get security alert statistics (for admin dashboard).
pub async fn get_security_stats(
    redis: &mut redis::aio::ConnectionManager,
) -> Result<serde_json::Value, ()> {
    let now = Utc::now().timestamp();
    let current_minute = now / 60;

    let forbidden_key = format!("{}forbidden:{}", SECURITY_COUNTER_PREFIX, current_minute);
    let unauthorized_key = format!("{}unauthorized:{}", SECURITY_COUNTER_PREFIX, current_minute);
    let ratelimit_key = format!("{}ratelimit:{}", SECURITY_COUNTER_PREFIX, current_minute);

    let forbidden: u32 = redis.get(&forbidden_key).await.unwrap_or(0);
    let unauthorized: u32 = redis.get(&unauthorized_key).await.unwrap_or(0);
    let ratelimit: u32 = redis.get(&ratelimit_key).await.unwrap_or(0);

    Ok(serde_json::json!({
        "current_minute": current_minute,
        "forbidden_count": forbidden,
        "unauthorized_count": unauthorized,
        "rate_limit_count": ratelimit,
        "alerts": {
            "forbidden_threshold": ALERT_THRESHOLDS.forbidden_per_minute,
            "unauthorized_threshold": ALERT_THRESHOLDS.failed_logins_per_minute,
            "rate_limit_threshold": ALERT_THRESHOLDS.rate_limit_hits_per_minute,
        }
    }))
}
