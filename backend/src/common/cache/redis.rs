use redis::aio::ConnectionManager;
use redis::AsyncCommands;
use serde::{de::DeserializeOwned, Serialize};

use crate::common::errors::app_error::AppError;

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
    pub async fn set(&self, key: &str, value: &str, ttl_secs: Option<u64>) -> Result<(), AppError> {
        let mut conn = self.conn.clone();
        if let Some(ttl) = ttl_secs {
            let _: () = conn
                .set_ex(key, value, ttl)
                .await
                .map_err(|e| AppError::External(format!("Redis SET failed: {e}")))?;
        } else {
            let _: () = conn
                .set(key, value)
                .await
                .map_err(|e| AppError::External(format!("Redis SET failed: {e}")))?;
        }
        Ok(())
    }

    /// Get a value by key.
    pub async fn get(&self, key: &str) -> Result<Option<String>, AppError> {
        let mut conn = self.conn.clone();
        let value: Option<String> = conn
            .get(key)
            .await
            .map_err(|e| AppError::External(format!("Redis GET failed: {e}")))?;
        Ok(value)
    }

    /// Delete one or more keys.
    pub async fn del(&self, keys: &[&str]) -> Result<(), AppError> {
        let mut conn = self.conn.clone();
        let _: () = conn
            .del(keys)
            .await
            .map_err(|e| AppError::External(format!("Redis DEL failed: {e}")))?;
        Ok(())
    }

    /// Check if a key exists.
    pub async fn exists(&self, key: &str) -> Result<bool, AppError> {
        let mut conn = self.conn.clone();
        let count: u64 = conn
            .exists(key)
            .await
            .map_err(|e| AppError::External(format!("Redis EXISTS failed: {e}")))?;
        Ok(count > 0)
    }

    /// Increment a counter with auto-expiry.
    pub async fn incr(&self, key: &str, ttl_secs: u64) -> Result<u64, AppError> {
        let mut conn = self.conn.clone();
        let count: u64 = conn
            .incr(key, 1u64)
            .await
            .map_err(|e| AppError::External(format!("Redis INCR failed: {e}")))?;
        if count == 1 {
            let _: () = conn
                .expire(key, ttl_secs as i64)
                .await
                .map_err(|e| AppError::External(format!("Redis EXPIRE failed: {e}")))?;
        }
        Ok(count)
    }

    /// Publish a message to a Redis Pub/Sub channel.
    pub async fn publish(&self, channel: &str, message: &str) -> Result<(), AppError> {
        let mut conn = self.conn.clone();
        let _: () = conn
            .publish(channel, message)
            .await
            .map_err(|e| AppError::External(format!("Redis PUBLISH failed: {e}")))?;
        Ok(())
    }

    /// Acquire a distributed lock (SET NX EX).
    pub async fn lock(&self, resource: &str, owner: &str, ttl_secs: u64) -> Result<bool, AppError> {
        let mut conn = self.conn.clone();
        let key = format!("lock:{resource}");
        let result: Option<String> = conn
            .set_nx(&key, owner)
            .await
            .map_err(|e| AppError::External(format!("Redis LOCK failed: {e}")))?;
        if result.is_some() {
            let _: () = conn
                .expire(&key, ttl_secs as i64)
                .await
                .map_err(|e| AppError::External(format!("Redis EXPIRE failed: {e}")))?;
        }
        Ok(result.is_some())
    }

    /// Release a distributed lock (only if owned by the given owner).
    pub async fn unlock(&self, resource: &str, owner: &str) -> Result<(), AppError> {
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

    // ── JSON cache helpers ──────────────────────────────────

    /// Serialize and cache a value as JSON.
    pub async fn set_json<T: Serialize>(&self, key: &str, value: &T, ttl_secs: u64) -> Result<(), AppError> {
        let json = serde_json::to_string(value)
            .map_err(|e| AppError::External(format!("Cache serialize failed: {e}")))?;
        self.set(key, &json, Some(ttl_secs)).await
    }

    /// Deserialize a cached JSON value.
    pub async fn get_json<T: DeserializeOwned>(&self, key: &str) -> Result<Option<T>, AppError> {
        match self.get(key).await? {
            Some(json) => serde_json::from_str(&json)
                .map(Some)
                .map_err(|e| AppError::External(format!("Cache deserialize failed: {e}"))),
            None => Ok(None),
        }
    }

    /// Delete all keys matching a prefix pattern using cursor-based SCAN.
    pub async fn del_prefix(&self, prefix: &str) -> Result<(), AppError> {
        let pattern = format!("{prefix}*");
        let mut cursor: u64 = 0;
        loop {
            let mut conn = self.conn.clone();
            let (next_cursor, keys): (u64, Vec<String>) = redis::cmd("SCAN")
                .arg(cursor)
                .arg("MATCH")
                .arg(&pattern)
                .arg("COUNT")
                .arg(100)
                .query_async(&mut conn)
                .await
                .map_err(|e| AppError::External(format!("Redis SCAN failed: {e}")))?;
            if !keys.is_empty() {
                let key_refs: Vec<&str> = keys.iter().map(|s| s.as_str()).collect();
                let mut conn2 = self.conn.clone();
                let _: () = conn2.del(&key_refs).await
                    .map_err(|e| AppError::External(format!("Redis DEL failed: {e}")))?;
            }
            cursor = next_cursor;
            if cursor == 0 {
                break;
            }
        }
        Ok(())
    }
}
