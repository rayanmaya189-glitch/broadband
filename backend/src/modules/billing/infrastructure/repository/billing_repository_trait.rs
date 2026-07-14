//! Billing repository trait.

use async_trait::async_trait;

use crate::common::errors::app_error::AppError;
use crate::modules::billing::domain::aggregates::invoice::invoice::Invoice;
use crate::modules::billing::domain::aggregates::payment::payment::Payment;

/// Repository trait for billing operations.
#[async_trait]
pub trait BillingRepositoryTrait: Send + Sync {
    /// Find invoice by ID.
    async fn find_invoice_by_id(&self, id: i64) -> Result<Option<Invoice>, AppError>;

    /// Save an invoice.
    async fn save_invoice(&self, invoice: &mut Invoice) -> Result<(), AppError>;

    /// Update an invoice.
    async fn update_invoice(&self, invoice: &Invoice) -> Result<(), AppError>;

    /// Find payment by ID.
    async fn find_payment_by_id(&self, id: i64) -> Result<Option<Payment>, AppError>;

    /// Save a payment.
    async fn save_payment(&self, payment: &mut Payment) -> Result<(), AppError>;

    /// List invoices for a customer.
    async fn list_invoices(
        &self,
        customer_id: Option<i64>,
        status: Option<&str>,
        offset: u32,
        limit: u32,
    ) -> Result<Vec<Invoice>, AppError>;
}
