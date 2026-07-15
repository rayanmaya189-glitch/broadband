/// Database configuration helpers.
pub struct DatabaseConfig;

impl DatabaseConfig {
    pub fn default_max_connections() -> u32 {
        20
    }

    pub fn default_connect_timeout_secs() -> u64 {
        30
    }

    pub fn default_idle_timeout_secs() -> u64 {
        600
    }
}
