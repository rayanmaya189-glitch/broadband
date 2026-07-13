use sea_orm::*;

use crate::common::errors::app_error::AppError;
use crate::modules::billing::model::invoice_entity::{self, Model as InvoiceModel};
use crate::modules::billing::model::invoice_line_item_entity::{self, Model as LineItemModel};
use crate::modules::billing::model::payment_entity::{self, Model as PaymentModel};
use crate::modules::billing::model::refund_entity::{self, Model as RefundModel};
use crate::modules::billing::model::discount_entity::{self, Model as DiscountModel};
use crate::modules::billing::model::billing_config_entity::{self, Model as BillingConfigModel};

pub struct BillingRepository {
    db: DatabaseConnection,
}

impl BillingRepository {
    pub fn new(db: &DatabaseConnection) -> Self {
        Self { db: db.clone() }
    }

    // ──── Invoices ────

    pub async fn list_invoices(
        &self,
        branch_id: Option<i64>,
        status: Option<&str>,
        customer_id: Option<i64>,
        page: i64,
        per_page: i64,
    ) -> Result<(Vec<InvoiceModel>, i64), AppError> {
        let page_size = per_page.max(1) as u64;
        let page_num = page.max(1) as u64;

        let mut select = invoice_entity::Entity::find();
        if let Some(bid) = branch_id {
            select = select.filter(invoice_entity::Column::BranchId.eq(bid));
        }
        if let Some(s) = status {
            select = select.filter(invoice_entity::Column::Status.eq(s));
        }
        if let Some(cid) = customer_id {
            select = select.filter(invoice_entity::Column::CustomerId.eq(cid));
        }

        let paginator = select
            .order_by_desc(invoice_entity::Column::CreatedAt)
            .paginate(&self.db, page_size);

        let total = paginator.num_items().await? as i64;
        let models = paginator.fetch_page(page_num - 1).await?;
        Ok((models, total))
    }

    pub async fn get_invoice_by_id(&self, id: i64) -> Result<Option<InvoiceModel>, AppError> {
        let model = invoice_entity::Entity::find_by_id(id)
            .one(&self.db)
            .await
            ?;
        Ok(model)
    }

    pub async fn create_invoice(
        &self,
        invoice_number: &str,
        customer_id: i64,
        branch_id: i64,
        subscription_id: i64,
        period_start: chrono::NaiveDate,
        period_end: chrono::NaiveDate,
        subtotal: rust_decimal::Decimal,
        discount: rust_decimal::Decimal,
        tax: rust_decimal::Decimal,
        cgst: rust_decimal::Decimal,
        sgst: rust_decimal::Decimal,
        igst: rust_decimal::Decimal,
        total: rust_decimal::Decimal,
        due_date: chrono::NaiveDate,
        notes: Option<&str>,
    ) -> Result<InvoiceModel, AppError> {
        let active = invoice_entity::ActiveModel {
            invoice_number: Set(invoice_number.to_string()),
            customer_id: Set(customer_id),
            branch_id: Set(branch_id),
            subscription_id: Set(subscription_id),
            billing_period_start: Set(period_start),
            billing_period_end: Set(period_end),
            subtotal: Set(subtotal),
            discount_amount: Set(discount),
            tax_amount: Set(tax),
            cgst_amount: Set(cgst),
            sgst_amount: Set(sgst),
            igst_amount: Set(igst),
            total_amount: Set(total),
            due_date: Set(due_date),
            notes: Set(notes.map(|s| s.to_string())),
            ..Default::default()
        };
        let model = active.insert(&self.db).await?;
        Ok(model)
    }

    pub async fn update_invoice_status(&self, id: i64, status: &str) -> Result<InvoiceModel, AppError> {
        let model = invoice_entity::Entity::find_by_id(id)
            .one(&self.db)
            .await
            ?
            .ok_or_else(|| AppError::NotFound("Invoice not found".into()))?;

        let mut active: invoice_entity::ActiveModel = model.into();
        active.status = Set(status.to_string());
        active.updated_at = Set(chrono::Utc::now().into());
        if status == "paid" {
            active.paid_at = Set(Some(chrono::Utc::now().into()));
        }
        let updated = active.update(&self.db).await?;
        Ok(updated)
    }

    pub async fn generate_invoice_number(&self) -> Result<String, AppError> {
        let now = chrono::Utc::now();
        let prefix = format!("INV-{}-{:02}", now.format("%Y"), now.format("%m"));
        let count = invoice_entity::Entity::find()
            .filter(invoice_entity::Column::InvoiceNumber.like(format!("{}%", prefix)))
            .count(&self.db)
            .await
            ? as i64;
        Ok(format!("{}-{:04}", prefix, count + 1))
    }

    pub async fn review_invoice(
        &self,
        id: i64,
        review_status: &str,
        review_notes: Option<&str>,
        reviewed_by: Option<i64>,
    ) -> Result<InvoiceModel, AppError> {
        let model = invoice_entity::Entity::find_by_id(id)
            .one(&self.db)
            .await
            ?
            .ok_or_else(|| AppError::NotFound("Invoice not found".into()))?;

        let mut active: invoice_entity::ActiveModel = model.into();
        active.review_status = Set(Some(review_status.to_string()));
        active.review_notes = Set(review_notes.map(|s| s.to_string()));
        active.reviewed_by = Set(reviewed_by);
        active.reviewed_at = Set(Some(chrono::Utc::now().into()));
        active.updated_at = Set(chrono::Utc::now().into());
        let updated = active.update(&self.db).await?;
        Ok(updated)
    }

    pub async fn approve_invoice(&self, id: i64, approved_by: i64) -> Result<InvoiceModel, AppError> {
        let model = invoice_entity::Entity::find_by_id(id)
            .one(&self.db)
            .await
            ?
            .ok_or_else(|| AppError::NotFound("Invoice not found".into()))?;

        let mut active: invoice_entity::ActiveModel = model.into();
        active.review_status = Set(Some("approved".to_string()));
        active.approved_by = Set(Some(approved_by));
        active.approved_at = Set(Some(chrono::Utc::now().into()));
        active.updated_at = Set(chrono::Utc::now().into());
        let updated = active.update(&self.db).await?;
        Ok(updated)
    }

    // ──── Line Items ────

    pub async fn create_line_item(
        &self,
        invoice_id: i64,
        description: &str,
        quantity: rust_decimal::Decimal,
        unit_price: rust_decimal::Decimal,
        amount: rust_decimal::Decimal,
        tax_rate: rust_decimal::Decimal,
        tax_amount: rust_decimal::Decimal,
    ) -> Result<LineItemModel, AppError> {
        let active = invoice_line_item_entity::ActiveModel {
            invoice_id: Set(invoice_id),
            description: Set(description.to_string()),
            quantity: Set(quantity),
            unit_price: Set(unit_price),
            amount: Set(amount),
            tax_rate: Set(tax_rate),
            tax_amount: Set(tax_amount),
            ..Default::default()
        };
        let model = active.insert(&self.db).await?;
        Ok(model)
    }

    pub async fn get_line_items(&self, invoice_id: i64) -> Result<Vec<LineItemModel>, AppError> {
        let models = invoice_line_item_entity::Entity::find()
            .filter(invoice_line_item_entity::Column::InvoiceId.eq(invoice_id))
            .order_by_asc(invoice_line_item_entity::Column::Id)
            .all(&self.db)
            .await
            ?;
        Ok(models)
    }

    // ──── Payments ────

    pub async fn list_payments(
        &self,
        customer_id: Option<i64>,
        status: Option<&str>,
        page: i64,
        per_page: i64,
    ) -> Result<(Vec<PaymentModel>, i64), AppError> {
        let page_size = per_page.max(1) as u64;
        let page_num = page.max(1) as u64;

        let mut select = payment_entity::Entity::find();
        if let Some(cid) = customer_id {
            select = select.filter(payment_entity::Column::CustomerId.eq(cid));
        }
        if let Some(s) = status {
            select = select.filter(payment_entity::Column::Status.eq(s));
        }

        let paginator = select
            .order_by_desc(payment_entity::Column::CreatedAt)
            .paginate(&self.db, page_size);

        let total = paginator.num_items().await? as i64;
        let models = paginator.fetch_page(page_num - 1).await?;
        Ok((models, total))
    }

    pub async fn create_payment(
        &self,
        payment_number: &str,
        invoice_id: i64,
        customer_id: i64,
        branch_id: i64,
        amount: rust_decimal::Decimal,
        method: &str,
        gateway: Option<&str>,
        gateway_tx_id: Option<&str>,
    ) -> Result<PaymentModel, AppError> {
        let active = payment_entity::ActiveModel {
            payment_number: Set(payment_number.to_string()),
            invoice_id: Set(invoice_id),
            customer_id: Set(customer_id),
            branch_id: Set(branch_id),
            amount: Set(amount),
            payment_method: Set(method.to_string()),
            payment_gateway: Set(gateway.map(|s| s.to_string())),
            gateway_transaction_id: Set(gateway_tx_id.map(|s| s.to_string())),
            status: Set("completed".to_string()),
            ..Default::default()
        };
        let model = active.insert(&self.db).await?;
        Ok(model)
    }

    pub async fn generate_payment_number(&self) -> Result<String, AppError> {
        let now = chrono::Utc::now();
        let prefix = format!("PAY-{}-{:02}", now.format("%Y"), now.format("%m"));
        let count = payment_entity::Entity::find()
            .filter(payment_entity::Column::PaymentNumber.like(format!("{}%", prefix)))
            .count(&self.db)
            .await
            ? as i64;
        Ok(format!("{}-{:04}", prefix, count + 1))
    }

    // ──── Refunds ────

    pub async fn list_refunds(
        &self,
        status: Option<&str>,
        page: i64,
        per_page: i64,
    ) -> Result<(Vec<RefundModel>, i64), AppError> {
        let page_size = per_page.max(1) as u64;
        let page_num = page.max(1) as u64;

        let mut select = refund_entity::Entity::find();
        if let Some(s) = status {
            select = select.filter(refund_entity::Column::Status.eq(s));
        }

        let paginator = select
            .order_by_desc(refund_entity::Column::CreatedAt)
            .paginate(&self.db, page_size);

        let total = paginator.num_items().await? as i64;
        let models = paginator.fetch_page(page_num - 1).await?;
        Ok((models, total))
    }

    pub async fn create_refund(
        &self,
        refund_number: &str,
        payment_id: i64,
        invoice_id: i64,
        customer_id: i64,
        amount: rust_decimal::Decimal,
        reason: &str,
        requested_by: Option<i64>,
    ) -> Result<RefundModel, AppError> {
        let active = refund_entity::ActiveModel {
            refund_number: Set(refund_number.to_string()),
            payment_id: Set(payment_id),
            invoice_id: Set(invoice_id),
            customer_id: Set(customer_id),
            amount: Set(amount),
            reason: Set(reason.to_string()),
            requested_by: Set(requested_by),
            ..Default::default()
        };
        let model = active.insert(&self.db).await?;
        Ok(model)
    }

    pub async fn approve_refund(&self, id: i64, approved_by: i64) -> Result<RefundModel, AppError> {
        let model = refund_entity::Entity::find_by_id(id)
            .one(&self.db)
            .await
            ?
            .ok_or_else(|| AppError::NotFound("Refund not found".into()))?;

        let mut active: refund_entity::ActiveModel = model.into();
        active.status = Set("approved".to_string());
        active.approved_by = Set(Some(approved_by));
        let updated = active.update(&self.db).await?;
        Ok(updated)
    }

    // ──── Discounts ────

    pub async fn list_discounts(&self, page: i64, per_page: i64) -> Result<(Vec<DiscountModel>, i64), AppError> {
        let page_size = per_page.max(1) as u64;
        let page_num = page.max(1) as u64;

        let paginator = discount_entity::Entity::find()
            .order_by_desc(discount_entity::Column::CreatedAt)
            .paginate(&self.db, page_size);

        let total = paginator.num_items().await? as i64;
        let models = paginator.fetch_page(page_num - 1).await?;
        Ok((models, total))
    }

    pub async fn create_discount(
        &self,
        name: &str,
        code: Option<&str>,
        discount_type: &str,
        value: rust_decimal::Decimal,
        max_uses: Option<i32>,
        valid_from: chrono::NaiveDate,
        valid_until: chrono::NaiveDate,
    ) -> Result<DiscountModel, AppError> {
        let active = discount_entity::ActiveModel {
            name: Set(name.to_string()),
            code: Set(code.map(|s| s.to_string())),
            discount_type: Set(discount_type.to_string()),
            value: Set(value),
            max_uses: Set(max_uses),
            valid_from: Set(valid_from),
            valid_until: Set(valid_until),
            ..Default::default()
        };
        let model = active.insert(&self.db).await?;
        Ok(model)
    }

    // ──── Branch & Customer State Helpers ────

    pub async fn get_branch_state(&self, branch_id: i64) -> Result<Option<String>, AppError> {
        use crate::modules::branch::model::branch_entity;
        let branch = branch_entity::Entity::find_by_id(branch_id)
            .one(&self.db).await?;
        Ok(branch.and_then(|b| b.state))
    }

    pub async fn get_branch_gstin(&self, branch_id: i64) -> Option<String> {
        use crate::modules::branch::model::branch_entity;
        branch_entity::Entity::find_by_id(branch_id)
            .one(&self.db).await
            .ok()
            .flatten()
            .and_then(|b| b.gstin)
    }

    pub async fn get_customer_state(&self, customer_id: i64) -> Result<Option<String>, AppError> {
        use crate::modules::customer::model::customer_address_entity;
        use crate::modules::customer::model::customer_profile_entity;
        use crate::modules::billing::utils::gst::extract_state_from_gstin;

        // 1. Try primary address first (most reliable)
        let addr = customer_address_entity::Entity::find()
            .filter(customer_address_entity::Column::CustomerId.eq(customer_id))
            .filter(customer_address_entity::Column::IsPrimary.eq(true))
            .one(&self.db).await?;
        if let Some(a) = addr {
            return Ok(Some(a.state));
        }

        // 2. Fallback: extract state from customer's GSTIN
        let profile = customer_profile_entity::Entity::find()
            .filter(customer_profile_entity::Column::CustomerId.eq(customer_id))
            .one(&self.db).await?;
        if let Some(p) = profile {
            if let Some(gstin) = &p.gstin {
                if let Some(state_code) = extract_state_from_gstin(gstin) {
                    return Ok(Some(state_code));
                }
            }
        }

        Ok(None)
    }

    // ──── Billing Config (Dunning & Tax) ────

    pub async fn get_config(&self, config_key: &str) -> Result<Option<BillingConfigModel>, AppError> {
        let model = billing_config_entity::Entity::find()
            .filter(billing_config_entity::Column::ConfigKey.eq(config_key))
            .one(&self.db).await?;
        Ok(model)
    }

    pub async fn upsert_config(&self, config_key: &str, config_value: serde_json::Value, updated_by: Option<i64>) -> Result<BillingConfigModel, AppError> {
        let now = chrono::Utc::now();
        let existing = self.get_config(config_key).await?;
        if let Some(model) = existing {
            let mut active: billing_config_entity::ActiveModel = model.into();
            active.config_value = Set(config_value);
            active.updated_by = Set(updated_by);
            active.updated_at = Set(now.into());
            let updated = active.update(&self.db).await?;
            Ok(updated)
        } else {
            let active = billing_config_entity::ActiveModel {
                config_key: Set(config_key.to_string()),
                config_value: Set(config_value),
                updated_by: Set(updated_by),
                ..Default::default()
            };
            let model = active.insert(&self.db).await?;
            Ok(model)
        }
    }
}
