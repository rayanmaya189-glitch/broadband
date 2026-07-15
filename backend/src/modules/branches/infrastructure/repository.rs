use sea_orm::{DatabaseConnection, EntityTrait, QueryFilter, ColumnTrait};
use crate::shared::errors::AppError;
use crate::modules::branches::domain::entities::branch;

/// Repository for branch data access.
pub struct BranchRepository<'a> {
    db: &'a DatabaseConnection,
}

impl<'a> BranchRepository<'a> {
    pub fn new(db: &'a DatabaseConnection) -> Self {
        Self { db }
    }

    pub async fn find_all_active(&self) -> Result<Vec<branch::Model>, AppError> {
        let branches = branch::Entity::find()
            .filter(branch::Column::IsActive.eq(true))
            .all(self.db)
            .await?;
        Ok(branches)
    }

    pub async fn find_by_id(&self, id: i64) -> Result<Option<branch::Model>, AppError> {
        let branch = branch::Entity::find_by_id(id)
            .one(self.db)
            .await?;
        Ok(branch)
    }

    pub async fn find_by_slug(&self, slug: &str) -> Result<Option<branch::Model>, AppError> {
        let branch = branch::Entity::find()
            .filter(branch::Column::Slug.eq(slug))
            .one(self.db)
            .await?;
        Ok(branch)
    }

    pub async fn find_by_code(&self, code: &str) -> Result<Option<branch::Model>, AppError> {
        let branch = branch::Entity::find()
            .filter(branch::Column::Code.eq(code))
            .one(self.db)
            .await?;
        Ok(branch)
    }
}
