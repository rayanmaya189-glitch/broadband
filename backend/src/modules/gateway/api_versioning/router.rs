//! API version router.
//!
//! Routes requests to the appropriate API version handler.

use axum::Router;

/// Supported API versions.
pub const SUPPORTED_VERSIONS: &[&str] = &["1.0"];

/// Default API version.
pub const DEFAULT_VERSION: &str = "1.0";

/// Create versioned API router.
pub fn versioned_router() -> Router {
    Router::new()
        // Add version-specific routes here
        // Example: .nest("/v1", v1_routes())
}

/// Get the API version from a request header.
pub fn extract_version(header: &str) -> Option<String> {
    let version = super::super::request_validator::schemas::ApiVersion::from_header(header)?;
    if SUPPORTED_VERSIONS.contains(&version.to_string().as_str()) {
        Some(version.to_string())
    } else {
        None
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_extract_version_valid() {
        assert_eq!(extract_version("1.0"), Some("1.0".to_string()));
    }

    #[test]
    fn test_extract_version_invalid() {
        assert_eq!(extract_version("2.0"), None);
    }

    #[test]
    fn test_extract_version_malformed() {
        assert_eq!(extract_version("invalid"), None);
    }
}
