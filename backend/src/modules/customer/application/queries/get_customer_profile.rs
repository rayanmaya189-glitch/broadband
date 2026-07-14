//! Get customer profile query handler.

/// Query to get customer profile.
#[derive(Debug, Clone)]
pub struct GetCustomerProfileQuery {
    pub customer_id: i64,
}
