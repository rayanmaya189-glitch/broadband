//! Billing application service.
//!
//! Orchestrates billing operations across the module.

use crate::common::errors::app_error::AppError;
use crate::modules::billing::domain::aggregates::invoice::invoice::Invoice;
use crate::modules::billing::domain::aggregates::payment::payment::Payment;

/// Application service for billing operations.
pub struct BillingApplicationService;

impl BillingApplicationService {
    /// Create a new invoice.
    pub async fn create_invoice(
        &self,
        id: i64,
        command: crate::modules::billing::application::commands::create_invoice::CreateInvoiceCommand,
    ) -> Result<Invoice, AppError> {
        crate::modules::billing::application::commands::create_invoice::CreateInvoiceHandler::handle(id, command)
    }

    /// Process a payment.
    pub async fn process_payment(
        &self,
        id: i64,
        invoice: &Invoice,
        command: crate::modules::billing::application::commands::process_payment::ProcessPaymentCommand,
    ) -> Result<Payment, AppError> {
        crate::modules::billing::application::commands::process_payment::ProcessPaymentHandler::handle(id, invoice, command)
    }
}
