//! Process payment command handler.

use rust_decimal::Decimal;

use crate::common::errors::app_error::AppError;
use crate::modules::billing::domain::aggregates::invoice::invoice::Invoice;
use crate::modules::billing::domain::aggregates::payment::payment::{Payment, PaymentMethod, PaymentEvent};
use crate::modules::billing::domain::rules::billing_rules;

/// Command to process a payment.
#[derive(Debug, Clone)]
pub struct ProcessPaymentCommand {
    pub invoice_id: i64,
    pub customer_id: i64,
    pub amount: Decimal,
    pub currency: String,
    pub payment_method: String,
    pub gateway_transaction_id: Option<String>,
}

/// Command handler for processing payments.
pub struct ProcessPaymentHandler;

impl ProcessPaymentHandler {
    pub fn handle(
        id: i64,
        invoice: &Invoice,
        command: ProcessPaymentCommand,
    ) -> Result<Payment, AppError> {
        billing_rules::validate_payment_amount(command.amount, invoice.total_amount)
            .map_err(|e| AppError::Validation(e))?;

        let payment_method = PaymentMethod::from_str(&command.payment_method)
            .map_err(|e| AppError::Validation(e.to_string()))?;

        let mut payment = Payment::create(
            id,
            command.invoice_id,
            command.customer_id,
            command.amount,
            command.currency,
            payment_method,
        )
        .map_err(|e| AppError::Validation(e.to_string()))?;

        payment.start_processing()
            .map_err(|e| AppError::Validation(e.to_string()))?;

        payment.complete(command.gateway_transaction_id, None)
            .map_err(|e| AppError::Validation(e.to_string()))?;

        Ok(payment)
    }
}
