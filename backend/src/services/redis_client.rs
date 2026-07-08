//! Redis service — caching, sessions, rate limiting, pub/sub.

use redis::aio::ConnectionManager;
use redis::AsyncCommands;

use crate::error::AppError;

/// Redis service wrapping the connection manager with convenience methods.
#[derive(Clone)]
pub struct RedisService {
    conn: ConnectionManager,
}

impl RedisService {
    pub fn new(conn: ConnectionManager) -> Self {
        Self { conn }
    }

    /// Set a key with optional TTL (seconds).
    pub async fn set(
        &self,
        key: &str,
        value: &str,
        ttl_secs: Option<u64>,
    ) -> Result<(), AppError> {
        let mut conn = self.conn.clone();
        if let Some(ttl) = ttl_secs {
            let _: () = conn.set_ex(key, value, ttl).await.map_err(|e| {
                AppError::External(format!("Redis SET failed: {e}"))
            })?;
        } else {
            let _: () = conn.set(key, value).await.map_err(|e| {
                AppError::External(format!("Redis SET failed: {e}"))
            })?;
        }
        Ok(())
    }

    /// Get a value by key.
    pub async fn get(&self, key: &str) -> Result<Option<String>, AppError> {
        let mut conn = self.conn.clone();
        let value: Option<String> = conn.get(key).await.map_err(|e| {
            AppError::External(format!("Redis GET failed: {e}"))
        })?;
        Ok(value)
    }

    /// Delete one or more keys.
    pub async fn del(&self, keys: &[&str]) -> Result<(), AppError> {
        let mut conn = self.conn.clone();
        let _: () = conn.del(keys).await.map_err(|e| {
            AppError::External(format!("Redis DEL failed: {e}"))
        })?;
        Ok(())
    }

    /// Check if a key exists.
    pub async fn exists(&self, key: &str) -> Result<bool, AppError> {
        let mut conn = self.conn.clone();
        let count: u64 = conn.exists(key).await.map_err(|e| {
            AppError::External(format!("Redis EXISTS failed: {e}"))
        })?;
        Ok(count > 0)
    }

    /// Increment a counter (sliding window rate limiting).
    pub async fn incr(&self, key: &str, ttl_secs: u64) -> Result<u64, AppError> {
        let mut conn = self.conn.clone();
        let count: u64 = conn.incr(key, 1u64).await.map_err(|e| {
            AppError::External(format!("Redis INCR failed: {e}"))
        })?;
        if count == 1 {
            let _: () = conn.expire(key, ttl_secs as i64).await.map_err(|e| {
                AppError::External(format!("Redis EXPIRE failed: {e}"))
            })?;
        }
        Ok(count)
    }

    /// Publish a message to a Redis Pub/Sub channel.
    pub async fn publish(
        &self,
        channel: &str,
        message: &str,
    ) -> Result<(), AppError> {
        let mut conn = self.conn.clone();
        let _: () = conn.publish(channel, message).await.map_err(|e| {
            AppError::External(format!("Redis PUBLISH failed: {e}"))
        })?;
        Ok(())
    }

    /// Acquire a distributed lock (SET NX EX).
    pub async fn lock(
        &self,
        resource: &str,
        owner: &str,
        ttl_secs: u64,
    ) -> Result<bool, AppError> {
        let mut conn = self.conn.clone();
        let key = format!("lock:{resource}");
        let result: Option<String> = conn
            .set_nx(&key, owner)
            .await
            .map_err(|e| AppError::External(format!("Redis LOCK failed: {e}")))?;
        if result.is_some() {
            let _: () = conn.expire(&key, ttl_secs as i64)
                .await
                .map_err(|e| AppError::External(format!("Redis EXPIRE failed: {e}")))?;
        }
        Ok(result.is_some())
    }

    /// Release a distributed lock (only if owned by the given owner).
    pub async fn unlock(
        &self,
        resource: &str,
        owner: &str,
    ) -> Result<(), AppError> {
        let mut conn = self.conn.clone();
        let key = format!("lock:{resource}");
        let script = "
            if redis.call('get', KEYS[1]) == ARGV[1] then
                return redis.call('del', KEYS[1])
            else
                return 0
            end
        ";
        let _: () = redis::cmd("EVAL")
            .arg(script)
            .arg(1)
            .arg(&key)
            .arg(owner)
            .query_async(&mut conn)
            .await
            .map_err(|e| AppError::External(format!("Redis UNLOCK failed: {e}")))?;
        Ok(())
    }

    /// Get the raw connection manager for advanced usage.
    pub fn inner(&self) -> &ConnectionManager {
        &self.conn
    }
}
