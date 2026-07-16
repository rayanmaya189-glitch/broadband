use async_trait::async_trait;
use sea_orm::{DatabaseConnection};
use crate::shared::errors::AppError;

pub type InvoiceModel = crate::modules::billing::domain::entities::invoice::Model;
pub type PaymentModel = crate::modules::billing::domain::entities::payment::Model;

#[async_trait]
pub trait BillingServiceTrait: Send + Sync {
    async fn list_invoices(
        &self,
        db: &DatabaseConnection,
        branch_id: Option<i64>,
        page: u64,
        limit: u64,
    ) -> Result<(Vec<InvoiceModel>, u64), AppError>;

    async fn get_invoice(
        &self,
        db: &DatabaseConnection,
        id: i64,
    ) -> Result<InvoiceModel, AppError>;

    async fn create_invoice(
        &self,
        db: &DatabaseConnection,
        customer_id: i64,
        branch_id: i64,
        subscription_id: i64,
        billing_period_start: chrono::NaiveDate,
        billing_period_end: chrono::NaiveDate,
        total_amount: sea_orm::prelude::Decimal,
    ) -> Result<InvoiceModel, AppError>;

    async fn record_payment(
        &self,
        db: &DatabaseConnection,
        invoice_id: i64,
        customer_id: i64,
        branch_id: i64,
        amount: sea_orm::prelude::Decimal,
        payment_method: String,
    ) -> Result<PaymentModel, AppError>;

    async fn list_payments(
        &self,
        db: &DatabaseConnection,
        branch_id: Option<i64>,
        page: u64,
        limit: u64,
    ) -> Result<(Vec<PaymentModel>, u64), AppError>;
}
