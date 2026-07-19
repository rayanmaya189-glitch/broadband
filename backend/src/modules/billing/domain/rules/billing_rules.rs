//! Billing domain business rules per §12-billing.md.
//! Enforces invariants for invoices, payments, refunds, and dunning.

use crate::shared::errors::AppError;
use rust_decimal::Decimal;

/// Invoice status transitions
pub fn validate_invoice_status_transition(current: &str, target: &str) -> Result<(), AppError> {
    let allowed = matches!(
        (current, target),
        ("draft", "sent")
            | ("sent", "paid")
            | ("sent", "overdue")
            | ("sent", "voided")
            | ("overdue", "paid")
            | ("overdue", "voided")
    );

    if allowed {
        Ok(())
    } else {
        Err(AppError::Conflict(format!(
            "Cannot transition invoice from '{}' to '{}'",
            current, target
        )))
    }
}

/// Validate invoice number format: INV-{YYYY}-{MM}-{SEQUENCE}
pub fn validate_invoice_number(number: &str) -> Result<(), AppError> {
    let parts: Vec<&str> = number.split('-').collect();
    if parts.len() != 4 || parts[0] != "INV" {
        return Err(AppError::Validation(
            "Invoice number must have format INV-{YYYY}-{MM}-{SEQ}".into(),
        ));
    }
    Ok(())
}

/// Validate payment amount is positive and within bounds
pub fn validate_payment_amount(amount: Decimal) -> Result<(), AppError> {
    if amount <= Decimal::ZERO {
        return Err(AppError::Validation(
            "Payment amount must be positive".into(),
        ));
    }
    if amount > Decimal::from(10_000_000) {
        return Err(AppError::Validation(
            "Payment amount exceeds maximum limit".into(),
        ));
    }
    Ok(())
}

/// Validate refund amount does not exceed original payment
pub fn validate_refund_amount(
    refund_amount: Decimal,
    original_amount: Decimal,
    already_refunded: Decimal,
) -> Result<(), AppError> {
    if refund_amount <= Decimal::ZERO {
        return Err(AppError::Validation(
            "Refund amount must be positive".into(),
        ));
    }
    let remaining = original_amount - already_refunded;
    if refund_amount > remaining {
        return Err(AppError::Validation(format!(
            "Refund amount {} exceeds remaining refundable amount {}",
            refund_amount, remaining
        )));
    }
    Ok(())
}

/// Validate discount code applicability
pub fn validate_discount(
    discount_type: &str,
    discount_value: Decimal,
    max_uses: Option<i32>,
    current_uses: i32,
    valid_from: chrono::NaiveDate,
    valid_until: chrono::NaiveDate,
) -> Result<(), AppError> {
    let today = chrono::Utc::now().date_naive();
    if today < valid_from || today > valid_until {
        return Err(AppError::Validation(
            "Discount code is not currently valid".into(),
        ));
    }
    if let Some(max) = max_uses {
        if current_uses >= max {
            return Err(AppError::Validation(
                "Discount code has reached maximum usage".into(),
            ));
        }
    }
    match discount_type {
        "percentage" => {
            if discount_value <= Decimal::ZERO || discount_value > Decimal::from(100) {
                return Err(AppError::Validation(
                    "Percentage discount must be between 0 and 100".into(),
                ));
            }
        }
        "fixed" => {
            if discount_value <= Decimal::ZERO {
                return Err(AppError::Validation(
                    "Fixed discount must be positive".into(),
                ));
            }
        }
        _ => {
            return Err(AppError::Validation(
                "Discount type must be 'percentage' or 'fixed'".into(),
            ));
        }
    }
    Ok(())
}

/// Validate dunning escalation timing
pub fn validate_dunning_stage(days_overdue: i32, current_stage: &str) -> Result<String, AppError> {
    let next_stage = match (current_stage, days_overdue) {
        ("none", d) if d >= 3 => "first_reminder",
        ("first_reminder", d) if d >= 7 => "second_reminder",
        ("second_reminder", d) if d >= 10 => "suspended",
        ("suspended", d) if d >= 30 => "terminated",
        _ => current_stage,
    };
    Ok(next_stage.to_string())
}

/// Calculate GST (CGST 9% + SGST 9% for intra-state Maharashtra)
pub fn calculate_gst(subtotal: Decimal, is_intra_state: bool) -> (Decimal, Decimal, Decimal) {
    let gst_rate = Decimal::new(9, 2); // 9%
    if is_intra_state {
        let cgst = (subtotal * gst_rate).round_dp(2);
        let sgst = (subtotal * gst_rate).round_dp(2);
        (cgst, sgst, cgst + sgst)
    } else {
        let igst = (subtotal * gst_rate * Decimal::from(2)).round_dp(2);
        (Decimal::ZERO, Decimal::ZERO, igst)
    }
}

/// Validate late fee calculation
pub fn validate_late_fee(late_fee: Decimal, invoice_amount: Decimal) -> Result<(), AppError> {
    let max_fee = invoice_amount * Decimal::new(2, 2); // Max 2% of invoice
    if late_fee > max_fee {
        return Err(AppError::Validation(
            "Late fee exceeds maximum allowed (2% of invoice)".into(),
        ));
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_invoice_status_transition() {
        assert!(validate_invoice_status_transition("draft", "sent").is_ok());
        assert!(validate_invoice_status_transition("sent", "paid").is_ok());
        assert!(validate_invoice_status_transition("paid", "sent").is_err());
    }

    #[test]
    fn test_payment_amount_validation() {
        assert!(validate_payment_amount(Decimal::from(100)).is_ok());
        assert!(validate_payment_amount(Decimal::ZERO).is_err());
        assert!(validate_payment_amount(Decimal::from(-50)).is_err());
    }

    #[test]
    fn test_gst_calculation() {
        let (cgst, sgst, total) = calculate_gst(Decimal::from(1000), true);
        assert_eq!(cgst, Decimal::new(90, 0));
        assert_eq!(sgst, Decimal::new(90, 0));
        assert_eq!(total, Decimal::new(180, 0));

        let (cgst, sgst, igst) = calculate_gst(Decimal::from(1000), false);
        assert_eq!(cgst, Decimal::ZERO);
        assert_eq!(sgst, Decimal::ZERO);
        assert_eq!(igst, Decimal::new(180, 0));
    }

    #[test]
    fn test_dunning_escalation() {
        assert_eq!(validate_dunning_stage(5, "none").unwrap(), "first_reminder");
        assert_eq!(
            validate_dunning_stage(8, "first_reminder").unwrap(),
            "second_reminder"
        );
        assert_eq!(
            validate_dunning_stage(12, "second_reminder").unwrap(),
            "suspended"
        );
        assert_eq!(
            validate_dunning_stage(35, "suspended").unwrap(),
            "terminated"
        );
    }
}
