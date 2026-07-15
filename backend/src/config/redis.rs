/// Redis configuration helpers.
pub struct RedisConfig;

impl RedisConfig {
    pub fn default_url() -> String {
        "redis://127.0.0.1:6379".to_string()
    }
}
