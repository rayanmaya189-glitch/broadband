use sea_orm::{DatabaseConnection, EntityTrait};
use crate::shared::errors::AppError;
use crate::modules::billing::domain::entities::{Invoice, Payment};

pub struct BillingRepository<'a> {
    db: &'a DatabaseConnection,
}

impl<'a> BillingRepository<'a> {
    pub fn new(db: &'a DatabaseConnection) -> Self { Self { db } }

    pub async fn find_invoice_by_id(&self, id: i64) -> Result<Option<<Invoice as sea_orm::EntityTrait>::Model>, AppError> {
        Ok(Invoice::find_by_id(id).one(self.db).await?)
    }

    pub async fn find_payment_by_id(&self, id: i64) -> Result<Option<<Payment as sea_orm::EntityTrait>::Model>, AppError> {
        Ok(Payment::find_by_id(id).one(self.db).await?)
    }
}
