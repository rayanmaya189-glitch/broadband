//! Validation schemas for API requests.

use serde::{Deserialize, Serialize};

/// Common pagination request schema.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PaginationRequest {
    pub page: Option<u32>,
    pub limit: Option<u32>,
}

impl PaginationRequest {
    /// Get offset from page and limit.
    pub fn offset(&self) -> u32 {
        let page = self.page.unwrap_or(1).max(1);
        let limit = self.limit().min(100);
        (page - 1) * limit
    }

    /// Get limit with default and max.
    pub fn limit(&self) -> u32 {
        self.limit.unwrap_or(20).min(100)
    }
}

/// Common search request schema.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SearchRequest {
    pub query: Option<String>,
    pub filters: Option<Vec<FilterRequest>>,
}

/// Filter request schema.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FilterRequest {
    pub field: String,
    pub operator: FilterOperator,
    pub value: String,
}

/// Filter operators.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum FilterOperator {
    Eq,
    Ne,
    Like,
    Gt,
    Lt,
    Gte,
    Lte,
    In,
}

/// API version header.
#[derive(Debug, Clone)]
pub struct ApiVersion {
    pub major: u32,
    pub minor: u32,
}

impl ApiVersion {
    /// Parse API version from header.
    pub fn from_header(header: &str) -> Option<Self> {
        let parts: Vec<&str> = header.split('.').collect();
        if parts.len() != 2 {
            return None;
        }
        let major = parts[0].parse().ok()?;
        let minor = parts[1].parse().ok()?;
        Some(Self { major, minor })
    }

    /// Get version string.
    pub fn to_string(&self) -> String {
        format!("{}.{}", self.major, self.minor)
    }
}

impl std::fmt::Display for ApiVersion {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}.{}", self.major, self.minor)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_pagination_offset() {
        let req = PaginationRequest { page: Some(3), limit: Some(10) };
        assert_eq!(req.offset(), 20);
    }

    #[test]
    fn test_pagination_default_values() {
        let req = PaginationRequest { page: None, limit: None };
        assert_eq!(req.offset(), 0);
        assert_eq!(req.limit(), 20);
    }

    #[test]
    fn test_api_version_parse() {
        let version = ApiVersion::from_header("1.0").unwrap();
        assert_eq!(version.major, 1);
        assert_eq!(version.minor, 0);
    }

    #[test]
    fn test_api_version_invalid() {
        assert!(ApiVersion::from_header("invalid").is_none());
    }
}
