use crate::modules::subscription::domain::entities::{Subscription, SubscriptionColumn};
use crate::shared::errors::AppError;
use sea_orm::{ColumnTrait, DatabaseConnection, EntityTrait, QueryFilter};

pub struct SubscriptionRepository<'a> {
    db: &'a DatabaseConnection,
}

impl<'a> SubscriptionRepository<'a> {
    pub fn new(db: &'a DatabaseConnection) -> Self {
        Self { db }
    }

    pub async fn find_by_id(
        &self,
        id: i64,
    ) -> Result<Option<<Subscription as sea_orm::EntityTrait>::Model>, AppError> {
        Ok(Subscription::find_by_id(id).one(self.db).await?)
    }

    pub async fn find_by_customer(
        &self,
        customer_id: i64,
    ) -> Result<Vec<<Subscription as sea_orm::EntityTrait>::Model>, AppError> {
        Ok(Subscription::find()
            .filter(SubscriptionColumn::CustomerId.eq(customer_id))
            .all(self.db)
            .await?)
    }
}
