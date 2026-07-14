//! List customers query handler.
//!
//! Handles listing customers with filtering and pagination.



/// Query to list customers.
#[derive(Debug, Clone)]
pub struct ListCustomersQuery {
    pub status: Option<String>,
    pub branch_id: Option<i64>,
    pub search: Option<String>,
    pub page: u32,
    pub limit: u32,
}

impl ListCustomersQuery {
    /// Calculate offset from page and limit.
    pub fn offset(&self) -> u32 {
        (self.page.saturating_sub(1)) * self.limit
    }
}

/// Query handler for listing customers.
pub struct ListCustomersHandler;

impl ListCustomersHandler {
    /// Validate and normalize the list query.
    pub fn normalize(query: &mut ListCustomersQuery) {
        if query.limit == 0 {
            query.limit = 20;
        }
        if query.limit > 100 {
            query.limit = 100;
        }
        if query.page == 0 {
            query.page = 1;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_normalize_default_values() {
        let mut query = ListCustomersQuery {
            status: None,
            branch_id: None,
            search: None,
            page: 0,
            limit: 0,
        };

        ListCustomersHandler::normalize(&mut query);
        assert_eq!(query.page, 1);
        assert_eq!(query.limit, 20);
    }

    #[test]
    fn test_normalize_limit_max() {
        let mut query = ListCustomersQuery {
            status: None,
            branch_id: None,
            search: None,
            page: 1,
            limit: 200,
        };

        ListCustomersHandler::normalize(&mut query);
        assert_eq!(query.limit, 100);
    }

    #[test]
    fn test_offset_calculation() {
        let query = ListCustomersQuery {
            status: None,
            branch_id: None,
            search: None,
            page: 3,
            limit: 10,
        };

        assert_eq!(query.offset(), 20);
    }
}
