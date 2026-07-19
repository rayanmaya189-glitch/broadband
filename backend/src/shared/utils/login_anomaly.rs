/// Login anomaly detection per §28 Security Design.
/// Tracks IP addresses per user, detects new IPs/locations, triggers alerts.
use chrono::Utc;
use redis::AsyncCommands;
use tracing::warn;

use crate::shared::errors::AppError;

/// Redis key pattern: aeroxe:login:ips:{user_id} → sorted set of IPs with timestamps
const LOGIN_IPS_PREFIX: &str = "aeroxe:login:ips:";
/// Redis key pattern: aeroxe:login:anomaly:{user_id}:{ip} → flag to avoid duplicate alerts
const ANOMALY_ALERT_PREFIX: &str = "aeroxe:login:anomaly:";
/// Maximum IPs to track per user (sliding window)
const MAX_TRACKED_IPS: usize = 20;
/// Alert cooldown per user+IP (avoid spamming)
const ALERT_COOLDOWN_SECS: u64 = 3600; // 1 hour

/// Result of anomaly detection check
#[derive(Debug)]
pub struct AnomalyCheck {
    pub is_new_ip: bool,
    pub is_anomaly: bool,
    pub recent_ip_count: usize,
    pub alert_key: String,
}

/// Check login for anomalies against known IP history.
/// Returns AnomalyCheck indicating whether this is a new/suspicious login.
pub async fn check_login_anomaly(
    redis: &mut redis::aio::ConnectionManager,
    user_id: i64,
    ip_address: &str,
) -> Result<AnomalyCheck, AppError> {
    let key = format!("{}{}", LOGIN_IPS_PREFIX, user_id);
    let now = Utc::now().timestamp();

    // Get all known IPs for this user
    let known_ips: Vec<String> = redis.zrange(&key, 0, -1).await.unwrap_or_default();

    let is_new_ip = !known_ips.contains(&ip_address.to_string());

    // Add current IP with timestamp as score (sorted set)
    let _: () = redis.zadd(&key, ip_address, now).await.unwrap_or(());

    // Trim to keep only the most recent MAX_TRACKED_IPS
    let count: usize = redis.zcard(&key).await.unwrap_or(0);
    if count > MAX_TRACKED_IPS {
        let _: () = redis
            .zremrangebyrank(&key, 0, (count - MAX_TRACKED_IPS - 1) as isize)
            .await
            .unwrap_or(());
    }

    // Set TTL on the key (30 days)
    let _: () = redis.expire(&key, 30 * 24 * 3600).await.unwrap_or(());

    // Check if we already alerted for this IP recently
    let alert_key = format!("{}{}:{}", ANOMALY_ALERT_PREFIX, user_id, ip_address);
    let already_alerted: bool = redis.exists(&alert_key).await.unwrap_or(false);

    let is_anomaly = is_new_ip && !already_alerted;

    // If anomaly detected, set cooldown flag
    if is_anomaly {
        let _: () = redis
            .set_ex(&alert_key, "1", ALERT_COOLDOWN_SECS)
            .await
            .unwrap_or(());
    }

    let recent_ip_count = known_ips.len();

    if is_new_ip {
        warn!(
            user_id = user_id,
            ip = ip_address,
            known_ips = recent_ip_count,
            "New login IP detected for user"
        );
    }

    Ok(AnomalyCheck {
        is_new_ip,
        is_anomaly,
        recent_ip_count,
        alert_key,
    })
}

/// Mask IP for logging (privacy: mask last octet)
pub fn mask_ip(ip: &str) -> String {
    let parts: Vec<&str> = ip.split('.').collect();
    if parts.len() == 4 {
        format!("{}.{}.{}.***", parts[0], parts[1], parts[2])
    } else {
        ip.to_string()
    }
}

/// Format anomaly alert message for notifications
pub fn format_anomaly_alert(user_email: &str, ip: &str, user_agent: &str) -> String {
    format!(
        "Security Alert: New login detected for {} from IP {} ({}). \
         If this wasn't you, please change your password immediately.",
        user_email,
        mask_ip(ip),
        user_agent
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_mask_ip() {
        assert_eq!(mask_ip("192.168.1.100"), "192.168.1.***");
        assert_eq!(mask_ip("10.0.0.1"), "10.0.0.***");
        assert_eq!(mask_ip("::1"), "::1"); // IPv6 unchanged
    }

    #[test]
    fn test_format_anomaly_alert() {
        let msg = format_anomaly_alert("user@test.com", "192.168.1.100", "Mozilla/5.0");
        assert!(msg.contains("Security Alert"));
        assert!(msg.contains("user@test.com"));
        assert!(msg.contains("192.168.1.***"));
    }
}
