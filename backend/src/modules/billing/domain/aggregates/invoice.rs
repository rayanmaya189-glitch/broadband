use crate::modules::billing::domain::value_objects::{InvoiceId, InvoiceStatus, Money};

/// Invoice aggregate root
#[derive(Debug, Clone)]
pub struct Invoice {
    pub id: InvoiceId,
    pub invoice_number: String,
    pub customer_id: i64,
    pub branch_id: i64,
    pub subscription_id: i64,
    pub billing_period_start: chrono::NaiveDate,
    pub billing_period_end: chrono::NaiveDate,
    pub subtotal: Money,
    pub discount_amount: Money,
    pub tax_amount: Money,
    pub total_amount: Money,
    pub currency: String,
    pub status: InvoiceStatus,
    pub due_date: chrono::NaiveDate,
    pub paid_at: Option<chrono::DateTime<chrono::Utc>>,
    pub payment_method: Option<String>,
}

/// Domain errors for Invoice aggregate
#[derive(Debug, Clone, PartialEq)]
pub enum InvoiceDomainError {
    /// Invoice cannot be paid if already paid
    AlreadyPaid,
    /// Invoice cannot be cancelled if already paid
    CannotCancelPaidInvoice,
    /// Invoice cannot be voided if already paid
    CannotVoidPaidInvoice,
    /// Payment amount exceeds invoice total
    PaymentExceedsTotal,
    /// Payment amount is zero or negative
    InvalidPaymentAmount,
    /// Invoice is overdue
    Overdue,
    /// Invoice not found
    NotFound(i64),
}

impl std::fmt::Display for InvoiceDomainError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::AlreadyPaid => write!(f, "Invoice is already paid"),
            Self::CannotCancelPaidInvoice => write!(f, "Cannot cancel a paid invoice"),
            Self::CannotVoidPaidInvoice => write!(f, "Cannot void a paid invoice"),
            Self::PaymentExceedsTotal => write!(f, "Payment amount exceeds invoice total"),
            Self::InvalidPaymentAmount => write!(f, "Payment amount must be greater than zero"),
            Self::Overdue => write!(f, "Invoice is overdue"),
            Self::NotFound(id) => write!(f, "Invoice {} not found", id),
        }
    }
}

impl std::error::Error for InvoiceDomainError {}

impl Invoice {
    /// Create a new invoice
    pub fn new(
        invoice_number: String,
        customer_id: i64,
        branch_id: i64,
        subscription_id: i64,
        billing_period_start: chrono::NaiveDate,
        billing_period_end: chrono::NaiveDate,
        subtotal: Money,
        discount_amount: Money,
        tax_amount: Money,
        due_date: chrono::NaiveDate,
    ) -> Result<Self, InvoiceDomainError> {
        let total = subtotal.clone() - discount_amount.clone() + tax_amount.clone();

        Ok(Self {
            id: InvoiceId::new(0),
            invoice_number,
            customer_id,
            branch_id,
            subscription_id,
            billing_period_start,
            billing_period_end,
            subtotal,
            discount_amount,
            tax_amount,
            total_amount: total,
            currency: "INR".to_string(),
            status: InvoiceStatus::Pending,
            due_date,
            paid_at: None,
            payment_method: None,
        })
    }

    /// Mark invoice as paid
    pub fn mark_paid(&mut self, payment_method: &str) -> Result<(), InvoiceDomainError> {
        if self.status == InvoiceStatus::Paid {
            return Err(InvoiceDomainError::AlreadyPaid);
        }
        self.status = InvoiceStatus::Paid;
        self.paid_at = Some(chrono::Utc::now());
        self.payment_method = Some(payment_method.to_string());
        Ok(())
    }

    /// Cancel invoice
    pub fn cancel(&mut self) -> Result<(), InvoiceDomainError> {
        if self.status == InvoiceStatus::Paid {
            return Err(InvoiceDomainError::CannotCancelPaidInvoice);
        }
        self.status = InvoiceStatus::Cancelled;
        Ok(())
    }

    /// Void invoice
    pub fn void(&mut self) -> Result<(), InvoiceDomainError> {
        if self.status == InvoiceStatus::Paid {
            return Err(InvoiceDomainError::CannotVoidPaidInvoice);
        }
        self.status = InvoiceStatus::Void;
        Ok(())
    }

    /// Check if invoice is overdue
    pub fn is_overdue(&self) -> bool {
        self.status == InvoiceStatus::Pending && chrono::Utc::now().date_naive() > self.due_date
    }

    /// Check if invoice can be modified
    pub fn can_be_modified(&self) -> bool {
        matches!(self.status, InvoiceStatus::Pending | InvoiceStatus::Overdue)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new_invoice() {
        let invoice = Invoice::new(
            "INV-001".to_string(),
            1, 1, 1,
            chrono::NaiveDate::from_ymd_opt(2026, 7, 1).unwrap(),
            chrono::NaiveDate::from_ymd_opt(2026, 7, 31).unwrap(),
            Money::new(1000, 0),
            Money::new(0, 0),
            Money::new(180, 0),
            chrono::NaiveDate::from_ymd_opt(2026, 8, 15).unwrap(),
        );
        assert!(invoice.is_ok());
        let invoice = invoice.unwrap();
        assert_eq!(invoice.total_amount.amount(), 1180);
        assert_eq!(invoice.status, InvoiceStatus::Pending);
    }

    #[test]
    fn test_mark_paid() {
        let mut invoice = Invoice::new(
            "INV-001".to_string(),
            1, 1, 1,
            chrono::NaiveDate::from_ymd_opt(2026, 7, 1).unwrap(),
            chrono::NaiveDate::from_ymd_opt(2026, 7, 31).unwrap(),
            Money::new(1000, 0),
            Money::new(0, 0),
            Money::new(180, 0),
            chrono::NaiveDate::from_ymd_opt(2026, 8, 15).unwrap(),
        ).unwrap();
        
        let result = invoice.mark_paid("upi");
        assert!(result.is_ok());
        assert_eq!(invoice.status, InvoiceStatus::Paid);
        assert!(invoice.paid_at.is_some());
    }

    #[test]
    fn test_mark_paid_already_paid() {
        let mut invoice = Invoice::new(
            "INV-001".to_string(),
            1, 1, 1,
            chrono::NaiveDate::from_ymd_opt(2026, 7, 1).unwrap(),
            chrono::NaiveDate::from_ymd_opt(2026, 7, 31).unwrap(),
            Money::new(1000, 0),
            Money::new(0, 0),
            Money::new(180, 0),
            chrono::NaiveDate::from_ymd_opt(2026, 8, 15).unwrap(),
        ).unwrap();
        
        invoice.mark_paid("upi").unwrap();
        let result = invoice.mark_paid("card");
        assert_eq!(result, Err(InvoiceDomainError::AlreadyPaid));
    }
}
