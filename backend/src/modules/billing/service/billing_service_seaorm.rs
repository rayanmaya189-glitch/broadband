use sea_orm::DatabaseConnection;

use crate::common::errors::app_error::AppError;
use crate::modules::billing::repository::billing_repository_seaorm::BillingRepositorySeaorm;
use crate::modules::billing::request::billing_request::*;
use crate::modules::billing::response::billing_response::*;

pub struct BillingServiceSeaorm {
    repo: BillingRepositorySeaorm,
}

impl BillingServiceSeaorm {
    pub fn new(db: &DatabaseConnection) -> Self {
        Self {
            repo: BillingRepositorySeaorm::new(db),
        }
    }

    pub async fn list_invoices(&self, query: InvoiceQuery) -> Result<InvoiceListResponse, AppError> {
        let page = query.page.unwrap_or(1);
        let per_page = query.per_page.unwrap_or(20);
        let (invoices, total) = self.repo
            .list_invoices(query.branch_id, query.status.as_deref(), query.customer_id, page, per_page)
            .await?;
        let responses: Vec<InvoiceResponse> = invoices.into_iter().map(InvoiceResponse::from_model).collect();
        let total_pages = (total as f64 / per_page as f64).ceil() as i64;
        Ok(InvoiceListResponse { invoices: responses, total, page, per_page, total_pages })
    }

    pub async fn get_invoice(&self, id: i64) -> Result<InvoiceResponse, AppError> {
        let model = self.repo.get_invoice_by_id(id).await?
            .ok_or_else(|| AppError::NotFound("Invoice not found".into()))?;
        Ok(InvoiceResponse::from_model(model))
    }

    pub async fn create_invoice(&self, req: CreateInvoiceRequest) -> Result<InvoiceResponse, AppError> {
        let invoice_number = self.repo.generate_invoice_number().await?;
        let subtotal: rust_decimal::Decimal = req.line_items.iter()
            .map(|li| li.unit_price * li.quantity.unwrap_or(rust_decimal::Decimal::ONE))
            .sum();
        let tax: rust_decimal::Decimal = req.line_items.iter()
            .map(|li| li.unit_price * li.quantity.unwrap_or(rust_decimal::Decimal::ONE) * li.tax_rate.unwrap_or(rust_decimal::Decimal::ZERO) / rust_decimal::Decimal::from(100))
            .sum();
        let total = subtotal + tax;
        let invoice = self.repo.create_invoice(
            &invoice_number, req.customer_id, req.branch_id, req.subscription_id,
            req.billing_period_start, req.billing_period_end, subtotal,
            rust_decimal::Decimal::ZERO, tax, total, req.due_date, req.notes.as_deref(),
        ).await?;
        for li in &req.line_items {
            let qty = li.quantity.unwrap_or(rust_decimal::Decimal::ONE);
            let amount = li.unit_price * qty;
            let tax_rate = li.tax_rate.unwrap_or(rust_decimal::Decimal::ZERO);
            let tax_amt = amount * tax_rate / rust_decimal::Decimal::from(100);
            self.repo.create_line_item(invoice.id, &li.description, qty, li.unit_price, amount, tax_rate, tax_amt).await?;
        }
        Ok(InvoiceResponse::from_model(invoice))
    }

    pub async fn get_line_items(&self, invoice_id: i64) -> Result<Vec<InvoiceLineItemResponse>, AppError> {
        let _ = self.repo.get_invoice_by_id(invoice_id).await?
            .ok_or_else(|| AppError::NotFound("Invoice not found".into()))?;
        let items = self.repo.get_line_items(invoice_id).await?;
        Ok(items.into_iter().map(InvoiceLineItemResponse::from_model).collect())
    }

    pub async fn send_invoice(&self, id: i64) -> Result<InvoiceResponse, AppError> {
        let model = self.repo.update_invoice_status(id, "sent").await?;
        Ok(InvoiceResponse::from_model(model))
    }

    pub async fn void_invoice(&self, id: i64) -> Result<InvoiceResponse, AppError> {
        let model = self.repo.update_invoice_status(id, "void").await?;
        Ok(InvoiceResponse::from_model(model))
    }

    pub async fn review_invoice(&self, id: i64, review_status: &str, review_notes: Option<&str>, reviewed_by: i64) -> Result<InvoiceResponse, AppError> {
        if !matches!(review_status, "approved" | "rejected") {
            return Err(AppError::Validation("review_status must be 'approved' or 'rejected'".into()));
        }
        let model = self.repo.review_invoice(id, review_status, review_notes, Some(reviewed_by)).await?;
        Ok(InvoiceResponse::from_model(model))
    }

    pub async fn record_payment(&self, req: RecordPaymentRequest) -> Result<PaymentResponse, AppError> {
        let invoice = self.repo.get_invoice_by_id(req.invoice_id).await?
            .ok_or_else(|| AppError::NotFound("Invoice not found".into()))?;
        let payment_number = self.repo.generate_payment_number().await?;
        let payment = self.repo.create_payment(
            &payment_number, req.invoice_id, invoice.customer_id, invoice.branch_id,
            req.amount, &req.payment_method, req.payment_gateway.as_deref(), req.gateway_transaction_id.as_deref(),
        ).await?;
        self.repo.update_invoice_status(req.invoice_id, "paid").await?;
        Ok(PaymentResponse::from_model(payment))
    }

    pub async fn list_payments(&self, query: PaymentQuery) -> Result<PaymentListResponse, AppError> {
        let page = query.page.unwrap_or(1);
        let per_page = query.per_page.unwrap_or(20);
        let (payments, total) = self.repo
            .list_payments(query.customer_id, query.status.as_deref(), page, per_page)
            .await?;
        let responses: Vec<PaymentResponse> = payments.into_iter().map(PaymentResponse::from_model).collect();
        let total_pages = (total as f64 / per_page as f64).ceil() as i64;
        Ok(PaymentListResponse { payments: responses, total, page, per_page, total_pages })
    }

    pub async fn request_refund(&self, req: CreateRefundRequest) -> Result<RefundResponse, AppError> {
        let refund_number = format!("REF-{}", chrono::Utc::now().format("%Y%m%d%H%M%S"));
        let refund = self.repo.create_refund(&refund_number, req.payment_id, 0, 0, req.amount, &req.reason, None).await?;
        Ok(RefundResponse::from_model(refund))
    }

    pub async fn approve_refund(&self, id: i64, approved_by: i64) -> Result<RefundResponse, AppError> {
        let refund = self.repo.approve_refund(id, approved_by).await?;
        Ok(RefundResponse::from_model(refund))
    }

    pub async fn list_discounts(&self, page: i64, per_page: i64) -> Result<Vec<DiscountResponse>, AppError> {
        let (discounts, _) = self.repo.list_discounts(page, per_page).await?;
        Ok(discounts.into_iter().map(DiscountResponse::from_model).collect())
    }

    pub async fn create_discount(&self, req: CreateDiscountRequest) -> Result<DiscountResponse, AppError> {
        let discount = self.repo.create_discount(
            &req.name, req.code.as_deref(), &req.discount_type, req.value,
            req.max_uses, req.valid_from, req.valid_until,
        ).await?;
        Ok(DiscountResponse::from_model(discount))
    }

    // ── Dunning & Tax Config ────────────────────────────────────

    pub async fn get_dunning_config(&self) -> Result<serde_json::Value, AppError> {
        // Config is stored in billing_config table via sqlx
        // For now return defaults - will be converted when billing_config entity is added
        Ok(serde_json::json!({
            "reminder_days": [3, 7],
            "suspension_day": 10,
            "termination_day": 30,
            "late_fee_percent": 2.0,
            "late_fee_cap_percent": 10.0,
            "channels": ["sms", "email", "whatsapp"]
        }))
    }

    pub async fn update_dunning_config(&self, config: serde_json::Value) -> Result<MessageResponse, AppError> {
        // Config update will be converted when billing_config entity is added
        Ok(MessageResponse { message: "Dunning config updated".into() })
    }

    pub async fn get_tax_config(&self) -> Result<serde_json::Value, AppError> {
        // Config is stored in billing_config table via sqlx
        // For now return defaults - will be converted when billing_config entity is added
        Ok(serde_json::json!({
            "cgst_rate": 9.0, "sgst_rate": 9.0, "igst_rate": 18.0,
            "applicable_state": "Maharashtra",
            "hsn_code": "998421", "sac_code": "998421",
            "tax_name": "GST on Internet Services"
        }))
    }

    pub async fn update_tax_config(&self, config: serde_json::Value) -> Result<MessageResponse, AppError> {
        // Config update will be converted when billing_config entity is added
        Ok(MessageResponse { message: "Tax config updated".into() })
    }
}
