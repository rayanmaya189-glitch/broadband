/// NATS configuration helpers.
pub struct NatsConfig;

impl NatsConfig {
    pub fn default_url() -> String {
        "nats://127.0.0.1:4222".to_string()
    }
}
