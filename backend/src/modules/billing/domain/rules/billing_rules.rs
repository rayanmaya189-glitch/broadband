//! Billing business rules.

use rust_decimal::Decimal;

use crate::modules::billing::domain::aggregates::invoice::invoice::InvoiceError;

/// Validate invoice amount.
pub fn validate_invoice_amount(
    subtotal: Decimal,
    tax_rate: Decimal,
) -> Result<(), InvoiceError> {
    if subtotal <= Decimal::ZERO {
        return Err(InvoiceError::Validation(
            "Subtotal must be positive".to_string(),
        ));
    }

    if tax_rate < Decimal::ZERO || tax_rate > Decimal::ONE {
        return Err(InvoiceError::Validation(
            "Tax rate must be between 0 and 1".to_string(),
        ));
    }

    Ok(())
}

/// Validate payment amount.
pub fn validate_payment_amount(
    payment_amount: Decimal,
    invoice_amount: Decimal,
) -> Result<(), String> {
    if payment_amount <= Decimal::ZERO {
        return Err("Payment amount must be positive".to_string());
    }

    if payment_amount > invoice_amount {
        return Err("Payment amount cannot exceed invoice amount".to_string());
    }

    Ok(())
}

/// Calculate GST (Goods and Services Tax).
pub fn calculate_gst(amount: Decimal, gst_rate: Decimal) -> Decimal {
    amount * gst_rate
}

/// Calculate TDS (Tax Deducted at Source).
pub fn calculate_tds(amount: Decimal, tds_rate: Decimal) -> Decimal {
    amount * tds_rate
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_validate_invoice_amount_valid() {
        assert!(validate_invoice_amount(Decimal::new(1000, 0), Decimal::new(18, 2)).is_ok());
    }

    #[test]
    fn test_validate_invoice_amount_zero() {
        assert!(validate_invoice_amount(Decimal::ZERO, Decimal::new(18, 2)).is_err());
    }

    #[test]
    fn test_validate_payment_amount() {
        assert!(validate_payment_amount(Decimal::new(500, 0), Decimal::new(1000, 0)).is_ok());
    }

    #[test]
    fn test_validate_payment_exceeds_invoice() {
        assert!(validate_payment_amount(Decimal::new(1500, 0), Decimal::new(1000, 0)).is_err());
    }

    #[test]
    fn test_calculate_gst() {
        let gst = calculate_gst(Decimal::new(1000, 0), Decimal::new(18, 2));
        assert_eq!(gst, Decimal::new(1800, 2));
    }
}
