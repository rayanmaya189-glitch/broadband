//! List addresses query handler.

/// Query to list addresses for a customer.
#[derive(Debug, Clone)]
pub struct ListAddressesQuery {
    pub customer_id: i64,
}
