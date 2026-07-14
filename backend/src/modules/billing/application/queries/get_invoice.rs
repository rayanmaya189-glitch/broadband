//! Get invoice query handler.

use crate::common::errors::app_error::AppError;
use crate::modules::billing::domain::aggregates::invoice::invoice::Invoice;

/// Query to get an invoice by ID.
#[derive(Debug, Clone)]
pub struct GetInvoiceQuery {
    pub invoice_id: i64,
}

/// Query handler for getting an invoice.
pub struct GetInvoiceHandler;

impl GetInvoiceHandler {
    pub fn execute(invoice: Option<Invoice>) -> Result<Invoice, AppError> {
        invoice.ok_or_else(|| AppError::NotFound("Invoice not found".to_string()))
    }
}
