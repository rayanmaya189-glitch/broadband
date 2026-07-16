use crate::modules::billing::domain::entities::{
    Invoice, InvoiceActiveModel, InvoiceColumn, Payment, PaymentActiveModel, PaymentColumn,
};
use crate::shared::errors::AppError;
use sea_orm::{
    ActiveModelTrait, ColumnTrait, DatabaseConnection, EntityTrait, PaginatorTrait, QueryFilter,
    Set,
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
}

