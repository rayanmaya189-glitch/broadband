//! List KYC documents query handler.

/// Query to list KYC documents for a customer.
#[derive(Debug, Clone)]
pub struct ListKycDocumentsQuery {
    pub customer_id: i64,
}
