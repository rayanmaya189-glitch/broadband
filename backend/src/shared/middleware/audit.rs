use axum::extract::Request;
use axum::middleware::Next;
use axum::response::Response;
use sea_orm::DatabaseConnection;
use std::sync::Arc;
use std::time::Instant;
use tracing::{debug, error};

use super::request_id::RequestId;

/// Middleware that logs all API actions to the audit_logs table.
///
/// Records:
/// - Request ID (for distributed tracing correlation)
/// - User ID, email, role (from JWT)
/// - HTTP method and URI
/// - IP address and user agent
/// - Response status code
/// - Request duration
///
/// Audit logging is done asynchronously (fire-and-forget) to avoid
/// impacting request latency.
pub async fn audit_middleware(request: Request, next: Next) -> Response {
    let start = Instant::now();
    let method = request.method().clone();
    let uri = request.uri().clone();
    let path = uri.path().to_string();

    // Extract client info
    let ip_address = extract_ip(&request);

    // Extract request ID from extensions (set by request_id_middleware)
    let request_id = request
        .extensions()
        .get::<RequestId>()
        .map(|r| r.as_str().to_string());

    // Extract user context if available (from branch_scope middleware)
    let user_context = request
        .extensions()
        .get::<crate::shared::middleware::auth::UserContext>()
        .cloned();

    // Extract DB connection for persistence
    let db = request
        .extensions()
        .get::<Arc<DatabaseConnection>>()
        .cloned();

    // Run the request
    let response = next.run(request).await;

    let duration_ms = start.elapsed().as_millis();
    let status = response.status();

    // Skip audit logging for health checks and metrics
    if should_skip_audit(&path) {
        return response;
    }

    // Determine audit result
    let result = if status.is_success() {
        "granted"
    } else {
        "denied"
    };

    // Extract resource type and ID from path
    let (resource_type, resource_id) = extract_resource_info(&path);

    // Log audit entry asynchronously
    let action = format!("{} {}", method, path);
    let user_id = user_context.as_ref().map(|u| u.user_id);
    let user_email = user_context.as_ref().map(|u| u.email.clone());
    let user_role = user_context.as_ref().map(|u| u.role.clone());

    debug!(
        request_id = ?request_id,
        user_id = ?user_id,
        action = %action,
        result = %result,
        status = %status.as_u16(),
        duration_ms = duration_ms,
        "Audit log entry"
    );

    // Fire-and-forget: persist audit log to database
    if let Some(db) = db {
        let request_id_clone = request_id.clone();
        let action_clone = action.clone();
        tokio::spawn(async move {
            if let Err(e) =
                crate::modules::audit::application::services::AuditService::record_action(
                    &db,
                    user_id,
                    user_email,
                    user_role,
                    action_clone,
                    resource_type,
                    resource_id,
                    Some(ip_address),
                    result.to_string(),
                    None,
                    None,
                )
                .await
            {
                error!(
                    request_id = ?request_id_clone,
                    error = %e,
                    "Failed to persist audit log"
                );
            }
        });
    }

    // Also log denied/server errors via tracing for alerting
    if result == "denied" || status.is_server_error() {
        error!(
            request_id = ?request_id,
            user_id = ?user_id,
            action = %action,
            status = %status.as_u16(),
            duration_ms = duration_ms,
            "Security audit event"
        );
    }

    response
}

/// Extract client IP from request.
///
/// Priority: real TCP peer IP > (trusted-proxy headers) > "unknown"
/// SECURITY: X-Forwarded-For / X-Real-IP are IGNORED unless TRUST_PROXY=true is
/// explicitly configured, preventing client-controlled IP spoofing.
/// This mirrors the logic in `rate_limit::extract_client_id`.
fn extract_ip(request: &Request) -> String {
    // 1. Prefer the real peer IP from the TCP connection
    if let Some(connect_info) = request
        .extensions()
        .get::<axum::extract::connect_info::ConnectInfo<std::net::SocketAddr>>()
    {
        return connect_info.0.ip().to_string();
    }

    // 2. Only trust forwarding headers when the operator explicitly opted in
    let trust_proxy = std::env::var("TRUST_PROXY")
        .map(|v| v.eq_ignore_ascii_case("true") || v == "1")
        .unwrap_or(false);
    if trust_proxy {
        let ip = request
            .headers()
            .get("x-forwarded-for")
            .and_then(|v| v.to_str().ok())
            .map(|v| v.split(',').next().unwrap_or(v).trim().to_string())
            .or_else(|| {
                request
                    .headers()
                    .get("x-real-ip")
                    .and_then(|v| v.to_str().ok())
                    .map(|s| s.to_string())
            });
        if let Some(ip) = ip {
            return ip;
        }
    }

    "unknown".to_string()
}

#[allow(dead_code)]
/// Extract user agent from request headers (available for future audit persistence)
fn extract_user_agent(request: &Request) -> String {
    request
        .headers()
        .get("user-agent")
        .and_then(|v| v.to_str().ok())
        .unwrap_or("unknown")
        .to_string()
}

/// Determine if audit logging should be skipped for this path
fn should_skip_audit(path: &str) -> bool {
    path == "/health"
        || path == "/ready"
        || path == "/api/v1/metrics"
        || path == "/api/v1/metrics/summary"
        || path.starts_with("/ws")
}

/// Extract resource type and ID from the URI path
fn extract_resource_info(path: &str) -> (Option<String>, Option<String>) {
    let segments: Vec<&str> = path.split('/').filter(|s| !s.is_empty()).collect();

    // Skip "api" and "v1" prefix
    if segments.len() < 3 {
        return (None, None);
    }

    let resource = segments[2]; // e.g., "customers", "invoices", "devices"

    let resource_type = match resource {
        "customers" => Some("customer".to_string()),
        "invoices" => Some("invoice".to_string()),
        "payments" => Some("payment".to_string()),
        "devices" => Some("device".to_string()),
        "subscriptions" => Some("subscription".to_string()),
        "tickets" => Some("ticket".to_string()),
        "plans" => Some("plan".to_string()),
        "branches" => Some("branch".to_string()),
        "users" => Some("user".to_string()),
        "roles" => Some("role".to_string()),
        "leads" => Some("lead".to_string()),
        "referrals" => Some("referral".to_string()),
        "installations" => Some("installation".to_string()),
        "notifications" => Some("notification".to_string()),
        "audit" => Some("audit".to_string()),
        "auth" => Some("auth".to_string()),
        _ => Some(resource.to_string()),
    };

    // Try to extract ID (next segment if it looks like a number)
    let resource_id = if segments.len() > 3 {
        let potential_id = segments[3];
        if potential_id.parse::<i64>().is_ok() {
            Some(potential_id.to_string())
        } else {
            None
        }
    } else {
        None
    };

    (resource_type, resource_id)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_should_skip_audit() {
        assert!(should_skip_audit("/health"));
        assert!(should_skip_audit("/ready"));
        assert!(should_skip_audit("/api/v1/metrics"));
        assert!(!should_skip_audit("/api/v1/customers"));
        assert!(!should_skip_audit("/api/v1/invoices"));
    }

    #[test]
    fn test_extract_resource_info() {
        let (rtype, rid) = extract_resource_info("/api/v1/customers/42");
        assert_eq!(rtype, Some("customer".to_string()));
        assert_eq!(rid, Some("42".to_string()));

        let (rtype, rid) = extract_resource_info("/api/v1/plans");
        assert_eq!(rtype, Some("plan".to_string()));
        assert_eq!(rid, None);

        let (rtype, rid) = extract_resource_info("/api/v1/invoices/overdue");
        assert_eq!(rtype, Some("invoice".to_string()));
        assert_eq!(rid, None); // "overdue" is not a number
    }
}
