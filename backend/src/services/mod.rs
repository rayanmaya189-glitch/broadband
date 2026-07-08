//! Cross-cutting services — Redis, NATS, storage, email.

pub mod nats;
pub mod redis_client;

pub use nats::NatsService;
pub use redis_client::RedisService;
