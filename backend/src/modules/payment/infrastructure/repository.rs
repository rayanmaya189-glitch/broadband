use crate::modules::payment::domain::entities::{gateway_config, payment_link};
use crate::shared::errors::AppError;
use sea_orm::{ColumnTrait, DatabaseConnection, EntityTrait, QueryFilter};

pub struct PaymentRepository<'a> {
    db: &'a DatabaseConnection,
}

impl<'a> PaymentRepository<'a> {
    pub fn new(db: &'a DatabaseConnection) -> Self {
        Self { db }
    }

    pub async fn find_link_by_id(&self, id: i64) -> Result<Option<payment_link::Model>, AppError> {
        Ok(payment_link::Entity::find_by_id(id).one(self.db).await?)
    }

    pub async fn find_link_by_link_id(
        &self,
        link_id: &str,
    ) -> Result<Option<payment_link::Model>, AppError> {
        Ok(payment_link::Entity::find()
            .filter(payment_link::Column::LinkId.eq(link_id.to_owned()))
            .one(self.db)
            .await?)
    }

    pub async fn find_links_by_invoice(
        &self,
        invoice_id: i64,
    ) -> Result<Vec<payment_link::Model>, AppError> {
        Ok(payment_link::Entity::find()
            .filter(payment_link::Column::InvoiceId.eq(invoice_id))
            .all(self.db)
            .await?)
    }

    pub async fn find_links_by_customer(
        &self,
        customer_id: i64,
    ) -> Result<Vec<payment_link::Model>, AppError> {
        Ok(payment_link::Entity::find()
            .filter(payment_link::Column::CustomerId.eq(customer_id))
            .all(self.db)
            .await?)
    }

    pub async fn find_gateway_config(
        &self,
        gateway_id: &str,
    ) -> Result<Option<gateway_config::Model>, AppError> {
        Ok(gateway_config::Entity::find()
            .filter(gateway_config::Column::GatewayId.eq(gateway_id.to_owned()))
            .one(self.db)
            .await?)
    }
}
