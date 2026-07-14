//! SeaORM implementation of the subscription repository.

use async_trait::async_trait;
use sea_orm::DatabaseConnection;

use crate::common::errors::app_error::AppError;
use crate::modules::subscription::domain::aggregates::subscription::subscription::Subscription;
use crate::modules::subscription::infrastructure::repository::subscription_repository_trait::SubscriptionRepositoryTrait;

/// SeaORM implementation of the subscription repository.
#[allow(dead_code)]
pub struct SeaOrmSubscriptionRepository {
    db: DatabaseConnection,
}

impl SeaOrmSubscriptionRepository {
    pub fn new(db: &DatabaseConnection) -> Self {
        Self { db: db.clone() }
    }
}

#[async_trait]
impl SubscriptionRepositoryTrait for SeaOrmSubscriptionRepository {
    async fn find_by_id(&self, _id: i64) -> Result<Option<Subscription>, AppError> {
        // TODO: Implement with actual SeaORM entity
        Ok(None)
    }

    async fn save(&self, _subscription: &mut Subscription) -> Result<(), AppError> {
        // TODO: Implement with actual SeaORM entity
        Ok(())
    }

    async fn update(&self, _subscription: &Subscription) -> Result<(), AppError> {
        // TODO: Implement with actual SeaORM entity
        Ok(())
    }

    async fn list_by_customer(&self, _customer_id: i64) -> Result<Vec<Subscription>, AppError> {
        // TODO: Implement with actual SeaORM entity
        Ok(vec![])
    }
}
