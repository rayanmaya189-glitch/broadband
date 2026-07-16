/// Gateway business rules and invariants
pub struct GatewayRules;

impl GatewayRules {
    /// Default rate limits (requests per minute)
    pub const UNAUTHENTICATED_LIMIT: u32 = 30;
    pub const AUTHENTICATED_LIMIT: u32 = 100;
    pub const WRITE_LIMIT: u32 = 30;
    pub const UPLOAD_LIMIT: u32 = 10;
    pub const AUTH_ENDPOINT_LIMIT: u32 = 5;

    /// API key prefix
    pub const API_KEY_PREFIX: &'static str = "ak_";

    /// API key length (excluding prefix)
    pub const API_KEY_LENGTH: usize = 32;

    /// Check if rate limit is exceeded
    pub fn is_rate_limited(request_count: u32, limit: u32) -> bool {
        request_count >= limit
    }

    /// Get rate limit for endpoint type
    pub fn rate_limit_for_endpoint(is_write: bool, is_auth: bool, is_upload: bool) -> u32 {
        if is_auth { Self::AUTH_ENDPOINT_LIMIT }
        else if is_upload { Self::UPLOAD_LIMIT }
        else if is_write { Self::WRITE_LIMIT }
        else { Self::AUTHENTICATED_LIMIT }
    }
}
