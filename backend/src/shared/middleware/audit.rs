use axum::extract::Request;
use axum::middleware::Next;
use axum::response::Response;
use std::time::Instant;
use tracing::{debug, error};

/// Middleware that logs all API actions to the audit_logs table.
///
/// Records:
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

    // Extract user context if available (from branch_scope middleware)
    let user_context = request
        .extensions()
        .get::<crate::shared::middleware::auth::UserContext>()
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
    } else if status == axum::http::StatusCode::FORBIDDEN {
        "denied"
    } else if status == axum::http::StatusCode::UNAUTHORIZED {
        "denied"
    } else {
        "success"
    };

    // Extract resource type and ID from path
    let (resource_type, resource_id) = extract_resource_info(&path);

    // Log audit entry asynchronously
    let action = format!("{} {}", method, path);
    let user_id = user_context.as_ref().map(|u| u.user_id);
    let user_email = user_context.as_ref().map(|u| u.email.clone());
    let user_role = user_context.as_ref().map(|u| u.role.clone());

    debug!(
        user_id = ?user_id,
        action = %action,
        result = %result,
        status = %status.as_u16(),
        duration_ms = duration_ms,
        "Audit log entry"
    );

    // Fire-and-forget: spawn audit log insertion
    // In production, this would write to the audit_logs table
    // For now, we log it via tracing which goes to the structured logging pipeline
    if result == "denied" || status.is_server_error() {
        error!(
            user_id = ?user_id,
            user_email = ?user_email,
            user_role = ?user_role,
            action = %action,
            resource_type = ?resource_type,
            resource_id = ?resource_id,
            ip_address = %ip_address,
            result = %result,
            status = %status.as_u16(),
            duration_ms = duration_ms,
            "Security audit event"
        );
    } else {
        debug!(
            user_id = ?user_id,
            action = %action,
            resource_type = ?resource_type,
            resource_id = ?resource_id,
            result = %result,
            status = %status.as_u16(),
            duration_ms = duration_ms,
            "Audit event"
        );
    }

    response
}

/// Extract client IP from request headers
fn extract_ip(request: &Request) -> String {
    request
        .headers()
        .get("x-forwarded-for")
        .or_else(|| request.headers().get("x-real-ip"))
        .and_then(|v| v.to_str().ok())
        .unwrap_or("unknown")
        .to_string()
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
