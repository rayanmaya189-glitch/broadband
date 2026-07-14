//! Create invoice command handler.

use chrono::NaiveDate;
use rust_decimal::Decimal;

use crate::common::errors::app_error::AppError;
use crate::modules::billing::domain::aggregates::invoice::invoice::{Invoice, InvoiceError};
use crate::modules::billing::domain::rules::billing_rules;

/// Command to create an invoice.
#[derive(Debug, Clone)]
pub struct CreateInvoiceCommand {
    pub customer_id: i64,
    pub subscription_id: i64,
    pub branch_id: i64,
    pub invoice_number: String,
    pub subtotal: Decimal,
    pub tax_rate: Decimal,
    pub due_date: NaiveDate,
}

/// Command handler for creating invoices.
pub struct CreateInvoiceHandler;

impl CreateInvoiceHandler {
    pub fn handle(id: i64, command: CreateInvoiceCommand) -> Result<Invoice, AppError> {
        billing_rules::validate_invoice_amount(command.subtotal, command.tax_rate)
            .map_err(|e| AppError::Validation(e.to_string()))?;

        let invoice = Invoice::create(
            id,
            command.invoice_number,
            command.customer_id,
            command.subscription_id,
            command.branch_id,
            command.subtotal,
            command.tax_rate,
            command.due_date,
        )
        .map_err(|e| AppError::Validation(e.to_string()))?;

        Ok(invoice)
    }
}
