use crate::modules::billing::domain::entities::{
    Discount, DiscountActiveModel, DiscountColumn, Invoice, InvoiceActiveModel, InvoiceColumn,
    Payment, PaymentActiveModel, PaymentColumn, Refund, RefundActiveModel,
};
use crate::shared::errors::AppError;
use sea_orm::{
    ActiveModelTrait, ColumnTrait, DatabaseConnection, EntityTrait, PaginatorTrait, QueryFilter,
    QueryOrder, Set,
};

pub struct BillingService;

impl BillingService {
    pub async fn list_invoices(
        db: &DatabaseConnection,
        branch_id: Option<i64>,
        _page: u64,
        _limit: u64,
    ) -> Result<
        (
            Vec<crate::modules::billing::domain::entities::invoice::Model>,
            u64,
        ),
        AppError,
    > {
        let mut query = Invoice::find();
        if let Some(bid) = branch_id {
            query = query.filter(InvoiceColumn::BranchId.eq(bid));
        }
        let total = query.clone().count(db).await?;
        let items = query.all(db).await?;
        Ok((items, total))
    }

    pub async fn get_invoice(
        db: &DatabaseConnection,
        id: i64,
    ) -> Result<crate::modules::billing::domain::entities::invoice::Model, AppError> {
        Invoice::find_by_id(id)
            .one(db)
            .await?
            .ok_or_else(|| AppError::NotFound(format!("Invoice {} not found", id)))
    }

    pub async fn create_invoice(
        db: &DatabaseConnection,
        customer_id: i64,
        branch_id: i64,
        subscription_id: i64,
        billing_period_start: chrono::NaiveDate,
        billing_period_end: chrono::NaiveDate,
        total_amount: sea_orm::prelude::Decimal,
    ) -> Result<crate::modules::billing::domain::entities::invoice::Model, AppError> {
        let now = chrono::Utc::now();
        let invoice_number = format!(
            "INV-{}-{:04}",
            now.format("%Y%m"),
            now.timestamp_millis() % 10000
        );
        let new_inv = InvoiceActiveModel {
            invoice_number: Set(invoice_number),
            customer_id: Set(customer_id),
            branch_id: Set(branch_id),
            subscription_id: Set(subscription_id),
            billing_period_start: Set(billing_period_start),
            billing_period_end: Set(billing_period_end),
            subtotal: Set(total_amount),
            discount_amount: Set(sea_orm::prelude::Decimal::ZERO),
            tax_amount: Set(sea_orm::prelude::Decimal::ZERO),
            total_amount: Set(total_amount),
            currency: Set("INR".to_string()),
            status: Set("pending".to_string()),
            due_date: Set(billing_period_end + chrono::Duration::days(15)),
            review_status: Set(Some("pending".to_string())),
            created_at: Set(now),
            updated_at: Set(now),
            ..Default::default()
        };
        Ok(new_inv.insert(db).await?)
    }

    pub async fn record_payment(
        db: &DatabaseConnection,
        invoice_id: i64,
        customer_id: i64,
        branch_id: i64,
        amount: sea_orm::prelude::Decimal,
        payment_method: String,
    ) -> Result<crate::modules::billing::domain::entities::payment::Model, AppError> {
        let now = chrono::Utc::now();
        let payment_number = format!(
            "PAY-{}-{:04}",
            now.format("%Y%m"),
            now.timestamp_millis() % 10000
        );
        let new_pay = PaymentActiveModel {
            payment_number: Set(payment_number),
            invoice_id: Set(invoice_id),
            customer_id: Set(customer_id),
            branch_id: Set(branch_id),
            amount: Set(amount),
            currency: Set("INR".to_string()),
            payment_method: Set(payment_method),
            status: Set("completed".to_string()),
            processed_at: Set(Some(now)),
            created_at: Set(now),
            ..Default::default()
        };
        let payment = new_pay.insert(db).await?;
        let inv = Invoice::find_by_id(invoice_id).one(db).await?;
        if let Some(i) = inv {
            let mut active: InvoiceActiveModel = i.into();
            active.status = Set("paid".to_string());
            active.paid_at = Set(Some(now));
            active.updated_at = Set(now);
            active.update(db).await?;
        }
        Ok(payment)
    }

    pub async fn list_payments(
        db: &DatabaseConnection,
        branch_id: Option<i64>,
        _page: u64,
        _limit: u64,
    ) -> Result<
        (
            Vec<crate::modules::billing::domain::entities::payment::Model>,
            u64,
        ),
        AppError,
    > {
        let mut query = Payment::find();
        if let Some(bid) = branch_id {
            query = query.filter(PaymentColumn::BranchId.eq(bid));
        }
        let total = query.clone().count(db).await?;
        let items = query.all(db).await?;
        Ok((items, total))
    }

    /// List invoices that are overdue (due_date < today and status is pending)
    pub async fn list_overdue_invoices(
        db: &DatabaseConnection,
        branch_id: Option<i64>,
    ) -> Result<Vec<crate::modules::billing::domain::entities::invoice::Model>, AppError> {
        let today = chrono::Utc::now().date_naive();
        let mut query = Invoice::find()
            .filter(InvoiceColumn::Status.eq("pending"))
            .filter(InvoiceColumn::DueDate.lt(today));

        if let Some(bid) = branch_id {
            query = query.filter(InvoiceColumn::BranchId.eq(bid));
        }

        let items = query.order_by_asc(InvoiceColumn::DueDate).all(db).await?;
        Ok(items)
    }

    /// Auto-generate invoices for subscriptions due for billing
    /// Returns the number of invoices generated
    pub async fn auto_generate_invoices(db: &DatabaseConnection) -> Result<u64, AppError> {
        use crate::modules::subscription::domain::entities::{Subscription, SubscriptionColumn};

        let today = chrono::Utc::now().date_naive();
        let due_subscriptions = Subscription::find()
            .filter(SubscriptionColumn::Status.eq("active"))
            .filter(SubscriptionColumn::NextBillingDate.is_not_null())
            .filter(SubscriptionColumn::NextBillingDate.lte(today))
            .all(db)
            .await?;

        let mut count = 0u64;
        for sub in due_subscriptions {
            // Check if an invoice already exists for this subscription and billing period
            let existing = Invoice::find()
                .filter(InvoiceColumn::SubscriptionId.eq(sub.id))
                .filter(InvoiceColumn::BillingPeriodEnd.eq(today))
                .one(db)
                .await?;

            if existing.is_some() {
                continue; // Skip if already invoiced
            }

            // Fetch plan price from plans module
            use crate::modules::plans::domain::entities::{PlanPricing, PlanPricingColumn};
            let pricing = PlanPricing::find()
                .filter(PlanPricingColumn::PlanId.eq(sub.plan_id))
                .filter(PlanPricingColumn::BillingPeriodMonths.eq(sub.billing_period_months))
                .filter(PlanPricingColumn::IsActive.eq(true))
                .one(db)
                .await?;

            let plan_price = pricing
                .map(|p| p.price)
                .unwrap_or(sea_orm::prelude::Decimal::ZERO);

            let period_start = sub.next_billing_date.unwrap_or(today);
            let period_end =
                period_start + chrono::Duration::days(30 * sub.billing_period_months as i64);

            let now = chrono::Utc::now();
            let invoice_number = format!(
                "INV-{}-{:04}",
                now.format("%Y%m"),
                now.timestamp_millis() % 10000
            );

            let new_inv = InvoiceActiveModel {
                invoice_number: Set(invoice_number),
                customer_id: Set(sub.customer_id),
                branch_id: Set(sub.branch_id),
                subscription_id: Set(sub.id),
                billing_period_start: Set(period_start),
                billing_period_end: Set(period_end),
                subtotal: Set(plan_price),
                discount_amount: Set(sea_orm::prelude::Decimal::ZERO),
                tax_amount: Set(sea_orm::prelude::Decimal::ZERO),
                total_amount: Set(plan_price),
                currency: Set("INR".to_string()),
                status: Set("pending".to_string()),
                due_date: Set(period_end + chrono::Duration::days(15)),
                review_status: Set(Some("pending".to_string())),
                created_at: Set(now),
                updated_at: Set(now),
                ..Default::default()
            };

            if let Ok(invoice) = new_inv.insert(db).await {
                // Update subscription's next_billing_date
                let mut sub_active: crate::modules::subscription::domain::entities::SubscriptionActiveModel = sub.into();
                sub_active.next_billing_date = Set(Some(period_end));
                sub_active.updated_at = Set(now);
                let _ = sub_active.update(db).await;

                count += 1;
                tracing::info!(
                    invoice_id = invoice.id,
                    subscription_id = invoice.subscription_id,
                    "Auto-generated invoice"
                );
            }
        }

        Ok(count)
    }

    // ─── Invoice Send ────────────────────────────────────────────────────────

    pub async fn send_invoice(
        db: &DatabaseConnection,
        id: i64,
    ) -> Result<crate::modules::billing::domain::entities::invoice::Model, AppError> {
        let inv = Self::get_invoice(db, id).await?;
        let mut active: InvoiceActiveModel = inv.into();
        active.status = Set("sent".to_string());
        active.updated_at = Set(chrono::Utc::now());
        Ok(active.update(db).await?)
    }

    // ─── Invoice Void ────────────────────────────────────────────────────────

    pub async fn void_invoice(
        db: &DatabaseConnection,
        id: i64,
        _reason: &str,
    ) -> Result<crate::modules::billing::domain::entities::invoice::Model, AppError> {
        let inv = Self::get_invoice(db, id).await?;
        if inv.status == "paid" {
            return Err(AppError::Validation(
                "Cannot void a paid invoice".to_string(),
            ));
        }
        let mut active: InvoiceActiveModel = inv.into();
        active.status = Set("void".to_string());
        active.updated_at = Set(chrono::Utc::now());
        Ok(active.update(db).await?)
    }

    // ─── Refunds ─────────────────────────────────────────────────────────────

    pub async fn request_refund(
        db: &DatabaseConnection,
        payment_id: i64,
        invoice_id: i64,
        customer_id: i64,
        amount: sea_orm::prelude::Decimal,
        reason: String,
        requested_by: i64,
    ) -> Result<crate::modules::billing::domain::entities::refund::Model, AppError> {
        let now = chrono::Utc::now();
        let refund_number = format!(
            "REF-{}-{:04}",
            now.format("%Y%m"),
            now.timestamp_millis() % 10000
        );
        let new_refund = RefundActiveModel {
            refund_number: Set(refund_number),
            payment_id: Set(payment_id),
            invoice_id: Set(invoice_id),
            customer_id: Set(customer_id),
            amount: Set(amount),
            reason: Set(reason),
            requested_by: Set(Some(requested_by)),
            status: Set("pending".to_string()),
            created_at: Set(now),
            ..Default::default()
        };
        Ok(new_refund.insert(db).await?)
    }

    pub async fn approve_refund(
        db: &DatabaseConnection,
        id: i64,
        approved_by: i64,
    ) -> Result<crate::modules::billing::domain::entities::refund::Model, AppError> {
        let refund = Refund::find_by_id(id)
            .one(db)
            .await?
            .ok_or_else(|| AppError::NotFound(format!("Refund {} not found", id)))?;
        if refund.status != "pending" {
            return Err(AppError::Validation(
                "Refund is not in pending status".to_string(),
            ));
        }
        let now = chrono::Utc::now();
        let mut active: RefundActiveModel = refund.into();
        active.status = Set("approved".to_string());
        active.approved_by = Set(Some(approved_by));
        active.approved_at = Set(Some(now));
        Ok(active.update(db).await?)
    }

    pub async fn reject_refund(
        db: &DatabaseConnection,
        id: i64,
        approved_by: i64,
        review_notes: &str,
    ) -> Result<crate::modules::billing::domain::entities::refund::Model, AppError> {
        let refund = Refund::find_by_id(id)
            .one(db)
            .await?
            .ok_or_else(|| AppError::NotFound(format!("Refund {} not found", id)))?;
        if refund.status != "pending" {
            return Err(AppError::Validation(
                "Refund is not in pending status".to_string(),
            ));
        }
        let now = chrono::Utc::now();
        let mut active: RefundActiveModel = refund.into();
        active.status = Set("rejected".to_string());
        active.approved_by = Set(Some(approved_by));
        active.approved_at = Set(Some(now));
        active.review_notes = Set(Some(review_notes.to_string()));
        Ok(active.update(db).await?)
    }

    // ─── Discounts ───────────────────────────────────────────────────────────

    pub async fn list_discounts(
        db: &DatabaseConnection,
    ) -> Result<Vec<crate::modules::billing::domain::entities::discount::Model>, AppError> {
        let items = Discount::find()
            .filter(DiscountColumn::IsActive.eq(true))
            .all(db)
            .await?;
        Ok(items)
    }

    pub async fn create_discount(
        db: &DatabaseConnection,
        name: String,
        code: Option<String>,
        discount_type: String,
        value: sea_orm::prelude::Decimal,
        valid_from: chrono::NaiveDate,
        valid_until: chrono::NaiveDate,
        created_by: i64,
    ) -> Result<crate::modules::billing::domain::entities::discount::Model, AppError> {
        let now = chrono::Utc::now();
        let new_discount = DiscountActiveModel {
            name: Set(name),
            code: Set(code),
            discount_type: Set(discount_type),
            value: Set(value),
            valid_from: Set(valid_from),
            valid_until: Set(valid_until),
            is_active: Set(true),
            current_uses: Set(Some(0)),
            created_by: Set(Some(created_by)),
            review_status: Set(Some("pending".to_string())),
            created_at: Set(now),
            ..Default::default()
        };
        Ok(new_discount.insert(db).await?)
    }

    // ─── Dunning Config ──────────────────────────────────────────────────────

    pub async fn get_dunning_config(
        _db: &DatabaseConnection,
    ) -> Result<crate::modules::billing::api::http::DunningConfigResponse, AppError> {
        Ok(crate::modules::billing::api::http::DunningConfigResponse {
            reminder_days: vec![3, 7],
            suspension_day: 10,
            termination_day: 30,
            late_fee_percent: "2.0".to_string(),
            late_fee_cap_percent: "10.0".to_string(),
            channels: vec![
                "sms".to_string(),
                "email".to_string(),
                "whatsapp".to_string(),
            ],
        })
    }

    // ─── Tax Config ──────────────────────────────────────────────────────────

    pub async fn get_tax_config(
        _db: &DatabaseConnection,
    ) -> Result<crate::modules::billing::api::http::TaxConfigResponse, AppError> {
        Ok(crate::modules::billing::api::http::TaxConfigResponse {
            cgst_rate: "9.0".to_string(),
            sgst_rate: "9.0".to_string(),
            igst_rate: "18.0".to_string(),
            applicable_state: "Maharashtra".to_string(),
            hsn_code: "998421".to_string(),
            sac_code: "998421".to_string(),
            tax_name: "GST on Internet Services".to_string(),
        })
    }
}
