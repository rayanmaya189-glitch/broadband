use crate::modules::billing::domain::entities::{
    Discount, DiscountActiveModel, DiscountColumn, Invoice, InvoiceActiveModel, InvoiceColumn,
    InvoiceLineItem, InvoiceLineItemActiveModel, InvoiceLineItemColumn, Payment, PaymentActiveModel,
    PaymentColumn, Refund, RefundActiveModel,
};
use crate::modules::billing::domain::rules::tax_service;
use crate::shared::errors::AppError;
use rust_decimal_macros::dec;
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
            "INV-{}-{}",
            now.format("%Y%m"),
            ulid::Ulid::new()
        );

        // Determine place of supply from customer state (default: Maharashtra for intra-state)
        let place_of_supply = Self::get_customer_state(db, customer_id).await
            .unwrap_or_else(|| "Maharashtra".to_string());
        let supplier_state = std::env::var("SUPPLIER_STATE")
            .unwrap_or_else(|_| "Maharashtra".to_string());
        let is_intra_state = tax_service::is_intra_state(&supplier_state, &place_of_supply);
        let gst = tax_service::calculate_gst_breakdown(total_amount, is_intra_state);

        let supplier_gstin = std::env::var("SUPPLIER_GSTIN")
            .unwrap_or_else(|_| "27AABCA1234H1Z5".to_string());

        let new_inv = InvoiceActiveModel {
            invoice_number: Set(invoice_number),
            customer_id: Set(customer_id),
            branch_id: Set(branch_id),
            subscription_id: Set(subscription_id),
            billing_period_start: Set(billing_period_start),
            billing_period_end: Set(billing_period_end),
            subtotal: Set(total_amount),
            discount_amount: Set(sea_orm::prelude::Decimal::ZERO),
            tax_amount: Set(gst.total_tax),
            total_amount: Set(total_amount + gst.total_tax),
            currency: Set("INR".to_string()),
            status: Set("pending".to_string()),
            due_date: Set(billing_period_end + chrono::Duration::days(15)),
            review_status: Set(Some("pending".to_string())),
            created_at: Set(now),
            updated_at: Set(now),
            cgst_amount: Set(gst.cgst_amount),
            sgst_amount: Set(gst.sgst_amount),
            igst_amount: Set(gst.igst_amount),
            place_of_supply_state: Set(place_of_supply),
            supplier_gstin: Set(Some(supplier_gstin)),
            reverse_charge: Set(false),
            late_fee_subtotal: Set(sea_orm::prelude::Decimal::ZERO),
            late_fee_gst: Set(sea_orm::prelude::Decimal::ZERO),
            ..Default::default()
        };
        Ok(new_inv.insert(db).await?)
    }

    /// Get customer's state for place-of-supply determination
    async fn get_customer_state(db: &DatabaseConnection, customer_id: i64) -> Option<String> {
        use crate::modules::customer::domain::entities::address;
        let addr = address::Entity::find()
            .filter(address::Column::CustomerId.eq(customer_id))
            .one(db)
            .await
            .ok()
            .flatten()?;
        Some(addr.state)
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
            "PAY-{}-{}",
            now.format("%Y%m"),
            ulid::Ulid::new()
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

        // Check if invoice is fully paid by summing all completed payments
        let inv = Invoice::find_by_id(invoice_id).one(db).await?;
        if let Some(i) = inv {
            let total_amount = i.total_amount;
            let total_paid = crate::modules::billing::domain::entities::payment::Entity::find()
                .filter(crate::modules::billing::domain::entities::payment::Column::InvoiceId.eq(invoice_id))
                .filter(crate::modules::billing::domain::entities::payment::Column::Status.eq("completed"))
                .all(db)
                .await?
                .iter()
                .fold(sea_orm::prelude::Decimal::ZERO, |acc, p| acc + p.amount);

            let mut active: InvoiceActiveModel = i.into();
            if total_paid >= total_amount {
                active.status = Set("paid".to_string());
                active.paid_at = Set(Some(now));
            } else {
                active.status = Set("partial".to_string());
            }
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
                "INV-{}-{}",
                now.format("%Y%m"),
                ulid::Ulid::new()
            );

            // Calculate GST for auto-generated invoice
            let place_of_supply = Self::get_customer_state(db, sub.customer_id).await
                .unwrap_or_else(|| "Maharashtra".to_string());
            let supplier_state = std::env::var("SUPPLIER_STATE")
                .unwrap_or_else(|_| "Maharashtra".to_string());
            let is_intra_state = tax_service::is_intra_state(&supplier_state, &place_of_supply);
            let gst = tax_service::calculate_gst_breakdown(plan_price, is_intra_state);
            let supplier_gstin = std::env::var("SUPPLIER_GSTIN")
                .unwrap_or_else(|_| "27AABCA1234H1Z5".to_string());

            let new_inv = InvoiceActiveModel {
                invoice_number: Set(invoice_number),
                customer_id: Set(sub.customer_id),
                branch_id: Set(sub.branch_id),
                subscription_id: Set(sub.id),
                billing_period_start: Set(period_start),
                billing_period_end: Set(period_end),
                subtotal: Set(plan_price),
                discount_amount: Set(sea_orm::prelude::Decimal::ZERO),
                tax_amount: Set(gst.total_tax),
                total_amount: Set(plan_price + gst.total_tax),
                currency: Set("INR".to_string()),
                status: Set("pending".to_string()),
                due_date: Set(period_end + chrono::Duration::days(15)),
                review_status: Set(Some("pending".to_string())),
                created_at: Set(now),
                updated_at: Set(now),
                cgst_amount: Set(gst.cgst_amount),
                sgst_amount: Set(gst.sgst_amount),
                igst_amount: Set(gst.igst_amount),
                place_of_supply_state: Set(place_of_supply),
                supplier_gstin: Set(Some(supplier_gstin)),
                reverse_charge: Set(false),
                late_fee_subtotal: Set(sea_orm::prelude::Decimal::ZERO),
                late_fee_gst: Set(sea_orm::prelude::Decimal::ZERO),
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
            "REF-{}-{}",
            now.format("%Y%m"),
            ulid::Ulid::new()
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

    // ─── Line Items ───────────────────────────────────────────────────────

    pub async fn list_line_items(
        db: &DatabaseConnection,
        invoice_id: i64,
    ) -> Result<Vec<crate::modules::billing::domain::entities::invoice_line_item::Model>, AppError> {
        let items = InvoiceLineItem::find()
            .filter(InvoiceLineItemColumn::InvoiceId.eq(invoice_id))
            .all(db)
            .await?;
        Ok(items)
    }

    pub async fn add_line_item(
        db: &DatabaseConnection,
        invoice_id: i64,
        description: String,
        quantity: sea_orm::prelude::Decimal,
        unit_price: sea_orm::prelude::Decimal,
        hsn_sac_code: Option<String>,
    ) -> Result<crate::modules::billing::domain::entities::invoice_line_item::Model, AppError> {
        let amount = quantity * unit_price;

        // Get invoice's place-of-supply to determine GST type
        let inv = Self::get_invoice(db, invoice_id).await?;
        let is_intra_state = inv.place_of_supply_state.to_lowercase() == "maharashtra";
        let gst = tax_service::calculate_gst_breakdown(amount, is_intra_state);
        let tax_type = if is_intra_state { "CGST_SGST" } else { "IGST" };
        let default_hsn = hsn_sac_code.unwrap_or_else(|| tax_service::SAC_INTERNET_ACCESS.to_string());

        let now = chrono::Utc::now();
        let item = InvoiceLineItemActiveModel {
            invoice_id: Set(invoice_id),
            description: Set(description),
            quantity: Set(quantity),
            unit_price: Set(unit_price),
            amount: Set(amount),
            tax_rate: Set(gst.total_tax / amount * dec!(100)),
            tax_amount: Set(gst.total_tax),
            created_at: Set(now),
            hsn_sac_code: Set(Some(default_hsn)),
            tax_type: Set(tax_type.to_string()),
            cgst_rate: Set(gst.cgst_rate),
            sgst_rate: Set(gst.sgst_rate),
            igst_rate: Set(gst.igst_rate),
            cgst_amount: Set(gst.cgst_amount),
            sgst_amount: Set(gst.sgst_amount),
            igst_amount: Set(gst.igst_amount),
            ..Default::default()
        };
        let saved = item.insert(db).await?;
        Self::recalculate_invoice_totals(db, invoice_id).await?;
        Ok(saved)
    }

    pub async fn remove_line_item(
        db: &DatabaseConnection,
        invoice_id: i64,
        item_id: i64,
    ) -> Result<(), AppError> {
        let item = InvoiceLineItem::find_by_id(item_id)
            .one(db)
            .await?
            .ok_or_else(|| AppError::NotFound(format!("Line item {} not found", item_id)))?;
        if item.invoice_id != invoice_id {
            return Err(AppError::Validation("Line item does not belong to this invoice".into()));
        }
        InvoiceLineItem::delete_by_id(item_id).exec(db).await?;
        Self::recalculate_invoice_totals(db, invoice_id).await?;
        Ok(())
    }

    async fn recalculate_invoice_totals(
        db: &DatabaseConnection,
        invoice_id: i64,
    ) -> Result<(), AppError> {
        let items = InvoiceLineItem::find()
            .filter(InvoiceLineItemColumn::InvoiceId.eq(invoice_id))
            .all(db)
            .await?;
        let subtotal: sea_orm::prelude::Decimal = items.iter().map(|i| i.amount).sum();
        let tax_total: sea_orm::prelude::Decimal = items.iter().map(|i| i.tax_amount).sum();
        let total = subtotal + tax_total;
        if let Some(inv) = Invoice::find_by_id(invoice_id).one(db).await? {
            let mut active: InvoiceActiveModel = inv.into();
            active.subtotal = Set(subtotal);
            active.tax_amount = Set(tax_total);
            active.total_amount = Set(total);
            active.updated_at = Set(chrono::Utc::now());
            active.update(db).await?;
        }
        Ok(())
    }

    // ─── Credit/Debit Notes ──────────────────────────────────────────────

    pub async fn create_credit_note(
        db: &DatabaseConnection,
        original_invoice_id: i64,
        customer_id: i64,
        branch_id: i64,
        reason: String,
        amount: sea_orm::prelude::Decimal,
        created_by: i64,
    ) -> Result<crate::modules::billing::domain::entities::credit_debit_note::Model, AppError> {
        let now = chrono::Utc::now();
        let note_number = format!("CN-{}-{}", now.format("%Y%m"), ulid::Ulid::new());

        // Get original invoice for GST context
        let inv = Self::get_invoice(db, original_invoice_id).await?;
        let is_intra_state = inv.place_of_supply_state.to_lowercase() == "maharashtra";
        let gst = tax_service::calculate_gst_breakdown(amount, is_intra_state);

        use crate::modules::billing::domain::entities::credit_debit_note;
        let note = credit_debit_note::ActiveModel {
            note_number: Set(note_number),
            note_type: Set("credit".to_string()),
            original_invoice_id: Set(original_invoice_id),
            customer_id: Set(customer_id),
            branch_id: Set(branch_id),
            reason: Set(reason),
            subtotal: Set(amount),
            cgst_amount: Set(gst.cgst_amount),
            sgst_amount: Set(gst.sgst_amount),
            igst_amount: Set(gst.igst_amount),
            total_amount: Set(amount + gst.total_tax),
            status: Set("pending".to_string()),
            created_by: Set(created_by),
            created_at: Set(now),
            updated_at: Set(now),
            ..Default::default()
        };
        Ok(note.insert(db).await?)
    }

    pub async fn approve_credit_note(
        db: &DatabaseConnection,
        note_id: i64,
        approved_by: i64,
    ) -> Result<crate::modules::billing::domain::entities::credit_debit_note::Model, AppError> {
        use crate::modules::billing::domain::entities::credit_debit_note;
        let note = credit_debit_note::Entity::find_by_id(note_id)
            .one(db)
            .await?
            .ok_or_else(|| AppError::NotFound(format!("Credit/debit note {} not found", note_id)))?;
        if note.status != "pending" {
            return Err(AppError::Validation("Note is not in pending status".into()));
        }
        let now = chrono::Utc::now();
        let mut active: credit_debit_note::ActiveModel = note.into();
        active.status = Set("approved".to_string());
        active.approved_by = Set(Some(approved_by));
        active.approved_at = Set(Some(now));
        active.updated_at = Set(now);
        Ok(active.update(db).await?)
    }

    // ─── Security Deposits ───────────────────────────────────────────────

    pub async fn collect_security_deposit(
        db: &DatabaseConnection,
        customer_id: i64,
        branch_id: i64,
        amount: sea_orm::prelude::Decimal,
        deposit_type: String,
    ) -> Result<crate::modules::billing::domain::entities::security_deposit::Model, AppError> {
        use crate::modules::billing::domain::entities::security_deposit;
        let now = chrono::Utc::now();
        let deposit = security_deposit::ActiveModel {
            customer_id: Set(customer_id),
            branch_id: Set(branch_id),
            amount: Set(amount),
            deposit_type: Set(deposit_type),
            status: Set("held".to_string()),
            collected_at: Set(now),
            created_at: Set(now),
            updated_at: Set(now),
            ..Default::default()
        };
        Ok(deposit.insert(db).await?)
    }

    pub async fn refund_security_deposit(
        db: &DatabaseConnection,
        deposit_id: i64,
        refund_amount: sea_orm::prelude::Decimal,
        reason: String,
    ) -> Result<crate::modules::billing::domain::entities::security_deposit::Model, AppError> {
        use crate::modules::billing::domain::entities::security_deposit;
        let deposit = security_deposit::Entity::find_by_id(deposit_id)
            .one(db)
            .await?
            .ok_or_else(|| AppError::NotFound(format!("Deposit {} not found", deposit_id)))?;
        if deposit.status != "held" {
            return Err(AppError::Validation("Deposit is not in held status".into()));
        }
        if refund_amount > deposit.amount {
            return Err(AppError::Validation("Refund exceeds deposit amount".into()));
        }
        let now = chrono::Utc::now();
        let mut active: security_deposit::ActiveModel = deposit.into();
        active.status = Set("refunded".to_string());
        active.refund_amount = Set(Some(refund_amount));
        active.refund_reason = Set(Some(reason));
        active.refunded_at = Set(Some(now));
        active.updated_at = Set(now);
        Ok(active.update(db).await?)
    }
}
