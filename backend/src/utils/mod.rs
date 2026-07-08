//! General-purpose utility functions.

use chrono::{DateTime, Utc};

/// Format a chrono DateTime as an ISO 8601 string.
pub fn format_iso(dt: &DateTime<Utc>) -> String {
    dt.to_rfc3339()
}

/// Calculate total pages from total records and page size.
pub fn total_pages(total: i64, page_size: u32) -> u32 {
    if page_size == 0 {
        return 0;
    }
    ((total as f64) / (page_size as f64)).ceil() as u32
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
