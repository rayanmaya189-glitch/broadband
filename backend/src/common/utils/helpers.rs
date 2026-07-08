use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

/// Paginated list request.
#[derive(Debug, Deserialize, ToSchema)]
pub struct PaginationParams {
    #[serde(default = "default_page")]
    pub page: u32,
    #[serde(default = "default_limit")]
    pub limit: u32,
    pub sort_by: Option<String>,
    pub sort_order: Option<String>,
    pub search: Option<String>,
}

fn default_page() -> u32 {
    1
}
fn default_limit() -> u32 {
    25
}

impl PaginationParams {
    pub fn offset(&self) -> u32 {
        (self.page.saturating_sub(1)) * self.limit
    }

    pub fn limit_i64(&self) -> i64 {
        self.limit.min(100) as i64
    }
}

/// Paginated list response wrapper.
#[derive(Debug, Serialize, ToSchema)]
pub struct PaginatedResponse<T: Serialize> {
    pub data: Vec<T>,
    pub total: i64,
    pub page: u32,
    pub limit: u32,
    pub total_pages: u32,
}

/// Calculate total pages from total records and page size.
pub fn total_pages(total: i64, page_size: u32) -> u32 {
    if page_size == 0 {
        return 0;
    }
    ((total as f64) / (page_size as f64)).ceil() as u32
}

/// Format a chrono DateTime as an ISO 8601 string.
pub fn format_iso(dt: &DateTime<Utc>) -> String {
    dt.to_rfc3339()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_total_pages() {
        assert_eq!(total_pages(0, 25), 0);
        assert_eq!(total_pages(1, 25), 1);
        assert_eq!(total_pages(25, 25), 1);
        assert_eq!(total_pages(26, 25), 2);
        assert_eq!(total_pages(100, 25), 4);
    }
}
