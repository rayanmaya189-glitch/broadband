use sqlx::PgPool;

use crate::common::errors::app_error::AppError;
use crate::modules::billing::repository::billing_repository::BillingRepository;
use crate::modules::billing::request::billing_request::*;
use crate::modules::billing::response::billing_response::*;

pub struct BillingService<'a> {
    repo: BillingRepository<'a>,
}

impl<'a> BillingService<'a> {
    pub fn new(pool: &'a PgPool) -> Self { Self { repo: BillingRepository::new(pool) } }

    pub async fn list_invoices(&self, query: InvoiceQuery) -> Result<InvoiceListResponse, AppError> {
        let page = query.page.unwrap_or(1);
        let per_page = query.per_page.unwrap_or(20);
        let (invoices, total) = self.repo.list_invoices(query.branch_id, query.status.as_deref(), query.customer_id, page, per_page).await?;
        let responses: Vec<InvoiceResponse> = invoices.iter().map(|i| InvoiceResponse { id: i.id, invoice_number: i.invoice_number.clone(), customer_id: i.customer_id, branch_id: i.branch_id, subscription_id: i.subscription_id, billing_period_start: i.billing_period_start, billing_period_end: i.billing_period_end, subtotal: i.subtotal, discount_amount: i.discount_amount, tax_amount: i.tax_amount, total_amount: i.total_amount, currency: i.currency.clone(), status: i.status.clone(), due_date: i.due_date, paid_at: i.paid_at, payment_method: i.payment_method.clone(), notes: i.notes.clone(), created_at: i.created_at, customer_name: None, branch_name: None }).collect();
        let total_pages = (total as f64 / per_page as f64).ceil() as i64;
        Ok(InvoiceListResponse { invoices: responses, total, page, per_page, total_pages })
    }

    pub async fn get_invoice(&self, id: i64) -> Result<InvoiceResponse, AppError> {
        let i = self.repo.get_invoice_by_id(id).await?.ok_or_else(|| AppError::NotFound("Invoice not found".into()))?;
        Ok(InvoiceResponse { id: i.id, invoice_number: i.invoice_number, customer_id: i.customer_id, branch_id: i.branch_id, subscription_id: i.subscription_id, billing_period_start: i.billing_period_start, billing_period_end: i.billing_period_end, subtotal: i.subtotal, discount_amount: i.discount_amount, tax_amount: i.tax_amount, total_amount: i.total_amount, currency: i.currency, status: i.status, due_date: i.due_date, paid_at: i.paid_at, payment_method: i.payment_method, notes: i.notes, created_at: i.created_at, customer_name: None, branch_name: None })
    }

    pub async fn create_invoice(&self, req: CreateInvoiceRequest) -> Result<InvoiceResponse, AppError> {
        let invoice_number = self.repo.generate_invoice_number().await?;
        let subtotal: rust_decimal::Decimal = req.line_items.iter().map(|li| li.unit_price * li.quantity.unwrap_or(rust_decimal::Decimal::ONE)).sum();
        let tax: rust_decimal::Decimal = req.line_items.iter().map(|li| li.unit_price * li.quantity.unwrap_or(rust_decimal::Decimal::ONE) * li.tax_rate.unwrap_or(rust_decimal::Decimal::ZERO) / rust_decimal::Decimal::from(100)).sum();
        let total = subtotal + tax;
        let invoice = self.repo.create_invoice(&invoice_number, req.customer_id, req.branch_id, req.subscription_id, req.billing_period_start, req.billing_period_end, subtotal, rust_decimal::Decimal::ZERO, tax, total, req.due_date, req.notes.as_deref()).await?;
        Ok(InvoiceResponse { id: invoice.id, invoice_number: invoice.invoice_number, customer_id: invoice.customer_id, branch_id: invoice.branch_id, subscription_id: invoice.subscription_id, billing_period_start: invoice.billing_period_start, billing_period_end: invoice.billing_period_end, subtotal: invoice.subtotal, discount_amount: invoice.discount_amount, tax_amount: invoice.tax_amount, total_amount: invoice.total_amount, currency: invoice.currency, status: invoice.status, due_date: invoice.due_date, paid_at: invoice.paid_at, payment_method: invoice.payment_method, notes: invoice.notes, created_at: invoice.created_at, customer_name: None, branch_name: None })
    }

    pub async fn record_payment(&self, req: RecordPaymentRequest) -> Result<PaymentResponse, AppError> {
        let invoice = self.repo.get_invoice_by_id(req.invoice_id).await?.ok_or_else(|| AppError::NotFound("Invoice not found".into()))?;
        let payment_number = self.repo.generate_payment_number().await?;
        let payment = self.repo.create_payment(&payment_number, req.invoice_id, invoice.customer_id, invoice.branch_id, req.amount, &req.payment_method, req.payment_gateway.as_deref(), req.gateway_transaction_id.as_deref()).await?;
        self.repo.update_invoice_status(req.invoice_id, "paid").await?;
        Ok(PaymentResponse { id: payment.id, payment_number: payment.payment_number, invoice_id: payment.invoice_id, customer_id: payment.customer_id, amount: payment.amount, currency: payment.currency, payment_method: payment.payment_method, payment_gateway: payment.payment_gateway, status: payment.status, processed_at: payment.processed_at, created_at: payment.created_at })
    }

    pub async fn list_payments(&self, query: PaymentQuery) -> Result<PaymentListResponse, AppError> {
        let page = query.page.unwrap_or(1);
        let per_page = query.per_page.unwrap_or(20);
        let (payments, total) = self.repo.list_payments(query.customer_id, query.status.as_deref(), page, per_page).await?;
        let responses: Vec<PaymentResponse> = payments.iter().map(|p| PaymentResponse { id: p.id, payment_number: p.payment_number.clone(), invoice_id: p.invoice_id, customer_id: p.customer_id, amount: p.amount, currency: p.currency.clone(), payment_method: p.payment_method.clone(), payment_gateway: p.payment_gateway.clone(), status: p.status.clone(), processed_at: p.processed_at, created_at: p.created_at }).collect();
        let total_pages = (total as f64 / per_page as f64).ceil() as i64;
        Ok(PaymentListResponse { payments: responses, total, page, per_page, total_pages })
    }

    pub async fn request_refund(&self, req: CreateRefundRequest) -> Result<RefundResponse, AppError> {
        let refund_number = format!("REF-{}", chrono::Utc::now().format("%Y%m%d%H%M%S"));
        let refund = self.repo.create_refund(&refund_number, req.payment_id, 0, 0, req.amount, &req.reason, None).await?;
        Ok(RefundResponse { id: refund.id, refund_number: refund.refund_number, payment_id: refund.payment_id, invoice_id: refund.invoice_id, customer_id: refund.customer_id, amount: refund.amount, reason: refund.reason, status: refund.status, created_at: refund.created_at })
    }

    pub async fn approve_refund(&self, id: i64, approved_by: i64) -> Result<RefundResponse, AppError> {
        let refund = self.repo.approve_refund(id, approved_by).await.map_err(|_| AppError::NotFound("Refund not found".into()))?;
        Ok(RefundResponse { id: refund.id, refund_number: refund.refund_number, payment_id: refund.payment_id, invoice_id: refund.invoice_id, customer_id: refund.customer_id, amount: refund.amount, reason: refund.reason, status: refund.status, created_at: refund.created_at })
    }

    pub async fn list_discounts(&self, page: i64, per_page: i64) -> Result<Vec<DiscountResponse>, AppError> {
        let (discounts, _) = self.repo.list_discounts(page, per_page).await?;
        Ok(discounts.iter().map(|d| DiscountResponse { id: d.id, name: d.name.clone(), code: d.code.clone(), discount_type: d.discount_type.clone(), value: d.value, max_uses: d.max_uses, current_uses: d.current_uses, valid_from: d.valid_from, valid_until: d.valid_until, is_active: d.is_active, created_at: d.created_at }).collect())
    }

    pub async fn create_discount(&self, req: CreateDiscountRequest) -> Result<DiscountResponse, AppError> {
        let discount = self.repo.create_discount(&req.name, req.code.as_deref(), &req.discount_type, req.value, req.max_uses, req.valid_from, req.valid_until).await?;
        Ok(DiscountResponse { id: discount.id, name: discount.name, code: discount.code, discount_type: discount.discount_type, value: discount.value, max_uses: discount.max_uses, current_uses: discount.current_uses, valid_from: discount.valid_from, valid_until: discount.valid_until, is_active: discount.is_active, created_at: discount.created_at })
    }
}
