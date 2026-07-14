//! SeaORM implementation of the billing repository.

use async_trait::async_trait;
use sea_orm::DatabaseConnection;

use crate::common::errors::app_error::AppError;
use crate::modules::billing::domain::aggregates::invoice::invoice::Invoice;
use crate::modules::billing::domain::aggregates::payment::payment::Payment;
use crate::modules::billing::infrastructure::repository::billing_repository_trait::BillingRepositoryTrait;

/// SeaORM implementation of the billing repository.
#[allow(dead_code)]
pub struct SeaOrmBillingRepository {
    db: DatabaseConnection,
}

impl SeaOrmBillingRepository {
    pub fn new(db: &DatabaseConnection) -> Self {
        Self { db: db.clone() }
    }
}

#[async_trait]
impl BillingRepositoryTrait for SeaOrmBillingRepository {
    async fn find_invoice_by_id(&self, _id: i64) -> Result<Option<Invoice>, AppError> {
        // TODO: Implement with actual SeaORM entity
        Ok(None)
    }

    async fn save_invoice(&self, _invoice: &mut Invoice) -> Result<(), AppError> {
        // TODO: Implement with actual SeaORM entity
        Ok(())
    }

    async fn update_invoice(&self, _invoice: &Invoice) -> Result<(), AppError> {
        // TODO: Implement with actual SeaORM entity
        Ok(())
    }

    async fn find_payment_by_id(&self, _id: i64) -> Result<Option<Payment>, AppError> {
        // TODO: Implement with actual SeaORM entity
        Ok(None)
    }

    async fn save_payment(&self, _payment: &mut Payment) -> Result<(), AppError> {
        // TODO: Implement with actual SeaORM entity
        Ok(())
    }

    async fn list_invoices(
        &self,
        _customer_id: Option<i64>,
        _status: Option<&str>,
        _offset: u32,
        _limit: u32,
    ) -> Result<Vec<Invoice>, AppError> {
        // TODO: Implement with actual SeaORM entity
        Ok(vec![])
    }
}
