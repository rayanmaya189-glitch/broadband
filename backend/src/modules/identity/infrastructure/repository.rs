use crate::modules::identity::domain::entities::user;
use crate::shared::errors::AppError;
use sea_orm::{ColumnTrait, DatabaseConnection, EntityTrait, QueryFilter};

/// Repository for user data access.
pub struct UserRepository<'a> {
    db: &'a DatabaseConnection,
}

impl<'a> UserRepository<'a> {
    pub fn new(db: &'a DatabaseConnection) -> Self {
        Self { db }
    }

    pub async fn find_by_id(&self, id: i64) -> Result<Option<user::Model>, AppError> {
        Ok(user::Entity::find_by_id(id).one(self.db).await?)
    }

    pub async fn find_by_email(&self, email: &str) -> Result<Option<user::Model>, AppError> {
        Ok(user::Entity::find()
            .filter(user::Column::Email.eq(email))
            .one(self.db)
            .await?)
    }

    pub async fn find_by_phone(&self, phone: &str) -> Result<Option<user::Model>, AppError> {
        Ok(user::Entity::find()
            .filter(user::Column::Phone.eq(phone))
            .one(self.db)
            .await?)
    }

    pub async fn find_all_active(&self) -> Result<Vec<user::Model>, AppError> {
        Ok(user::Entity::find()
            .filter(user::Column::DeletedAt.is_null())
            .all(self.db)
            .await?)
    }
}
