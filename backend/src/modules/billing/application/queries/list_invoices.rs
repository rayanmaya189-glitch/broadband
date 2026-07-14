//! List invoices query handler.

/// Query to list invoices.
#[derive(Debug, Clone)]
pub struct ListInvoicesQuery {
    pub customer_id: Option<i64>,
    pub status: Option<String>,
    pub page: u32,
    pub limit: u32,
}

impl ListInvoicesQuery {
    pub fn offset(&self) -> u32 {
        (self.page.saturating_sub(1)) * self.limit
    }
}
