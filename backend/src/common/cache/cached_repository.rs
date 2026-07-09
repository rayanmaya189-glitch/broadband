use serde::{de::DeserializeOwned, Serialize};

use crate::common::cache::redis::RedisService;
use crate::common::errors::app_error::AppError;

/// Cache-aside helper that provides a clean interface for caching typed data in Redis.
///
/// # Usage
/// ```ignore
/// let cache = CacheHelper::new(&state.redis, "plan", 300); // 5 min TTL
/// let plan: Option<Plan> = cache.get_by_id(plan_id).await?;
/// cache.invalidate_by_id(plan_id).await?;
/// cache.invalidate_prefix().await?;
/// ```
pub struct CacheHelper<'a> {
    redis: &'a RedisService,
    namespace: &'a str,
    default_ttl: u64,
}

impl<'a> CacheHelper<'a> {
    pub fn new(redis: &'a RedisService, namespace: &'a str, default_ttl: u64) -> Self {
        Self { redis, namespace, default_ttl }
    }

    /// Build a cache key from a namespace and ID.
    fn key_by_id(&self, id: i64) -> String {
        format!("{}:{}", self.namespace, id)
    }

    /// Build a cache key from a namespace and string suffix.
    fn key_by_suffix(&self, suffix: &str) -> String {
        format!("{}:{}", self.namespace, suffix)
    }

    /// Retrieve a cached value by integer ID.
    pub async fn get_by_id<T: DeserializeOwned>(&self, id: i64) -> Result<Option<T>, AppError> {
        self.redis.get_json(&self.key_by_id(id)).await
    }

    /// Store a value in cache by integer ID with the default TTL.
    pub async fn set_by_id<T: Serialize>(&self, id: i64, value: &T) -> Result<(), AppError> {
        self.redis.set_json(&self.key_by_id(id), value, self.default_ttl).await
    }

    /// Retrieve a cached value by string key suffix.
    pub async fn get_by_key<T: DeserializeOwned>(&self, suffix: &str) -> Result<Option<T>, AppError> {
        self.redis.get_json(&self.key_by_suffix(suffix)).await
    }

    /// Store a value in cache by string key suffix with the default TTL.
    pub async fn set_by_key<T: Serialize>(&self, suffix: &str, value: &T) -> Result<(), AppError> {
        self.redis.set_json(&self.key_by_suffix(suffix), value, self.default_ttl).await
    }

    /// Invalidate a cached entry by integer ID.
    pub async fn invalidate_by_id(&self, id: i64) -> Result<(), AppError> {
        self.redis.del(&[&self.key_by_id(id)]).await
    }

    /// Invalidate a cached entry by string key suffix.
    pub async fn invalidate_by_key(&self, suffix: &str) -> Result<(), AppError> {
        self.redis.del(&[&self.key_by_suffix(suffix)]).await
    }

    /// Invalidate all cached entries under this namespace (SCAN + DEL).
    pub async fn invalidate_prefix(&self) -> Result<(), AppError> {
        self.redis.del_prefix(&format!("{}:", self.namespace)).await
    }
}
