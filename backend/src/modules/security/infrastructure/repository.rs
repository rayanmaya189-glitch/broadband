use crate::modules::security::domain::entities::{Permission, PermissionColumn, Role, RoleColumn};
use crate::shared::errors::AppError;
use sea_orm::{ColumnTrait, DatabaseConnection, EntityTrait, QueryFilter};

pub struct SecurityRepository<'a> {
    db: &'a DatabaseConnection,
}

impl<'a> SecurityRepository<'a> {
    pub fn new(db: &'a DatabaseConnection) -> Self {
        Self { db }
    }

    pub async fn find_role_by_slug(
        &self,
        slug: &str,
    ) -> Result<Option<<Role as sea_orm::EntityTrait>::Model>, AppError> {
        Ok(Role::find()
            .filter(RoleColumn::Slug.eq(slug))
            .one(self.db)
            .await?)
    }

    pub async fn find_permission_by_name(
        &self,
        name: &str,
    ) -> Result<Option<<Permission as sea_orm::EntityTrait>::Model>, AppError> {
        Ok(Permission::find()
            .filter(PermissionColumn::Name.eq(name))
            .one(self.db)
            .await?)
    }
}
