use redis::aio::ConnectionManager;

/// Create a Redis connection pool.
pub async fn create_redis_pool(redis_url: &str) -> anyhow::Result<ConnectionManager> {
    let client = redis::Client::open(redis_url)?;
    let manager = ConnectionManager::new(client).await?;
    Ok(manager)
}

/// Cache key patterns for the application.
pub struct CacheKeys;

impl CacheKeys {
    pub fn session(user_id: i64) -> String {
        format!("session:{}", user_id)
    }

    pub fn otp(phone: &str) -> String {
        format!("otp:{}", phone)
    }

    pub fn rate_limit(identifier: &str, window: &str) -> String {
        format!("rate:{}:{}", identifier, window)
    }

    pub fn cache(key: &str) -> String {
        format!("cache:{}", key)
    }

    pub fn active_plans() -> String {
        "cache:plans:active".to_string()
    }

    pub fn customer_profile(customer_id: i64) -> String {
        format!("cache:customer:{}:profile", customer_id)
    }

    pub fn rbac_permissions(user_id: i64) -> String {
        format!("cache:rbac:permissions:{}", user_id)
    }

    pub fn blacklist(token_jti: &str) -> String {
        format!("blacklist:{}", token_jti)
    }

    pub fn lock(resource: &str, id: &str) -> String {
        format!("lock:{}:{}", resource, id)
    }
}
