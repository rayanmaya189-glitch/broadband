use crate::modules::plans::domain::entities::{Plan, PlanColumn};
use crate::shared::errors::AppError;
use sea_orm::{ColumnTrait, DatabaseConnection, EntityTrait, QueryFilter};

pub struct PlanRepository<'a> {
    db: &'a DatabaseConnection,
}

impl<'a> PlanRepository<'a> {
    pub fn new(db: &'a DatabaseConnection) -> Self {
        Self { db }
    }
    pub async fn find_by_id(
        &self,
        id: i64,
    ) -> Result<Option<<Plan as sea_orm::EntityTrait>::Model>, AppError> {
        Ok(Plan::find_by_id(id).one(self.db).await?)
    }
    pub async fn find_by_slug(
        &self,
        slug: &str,
    ) -> Result<Option<<Plan as sea_orm::EntityTrait>::Model>, AppError> {
        Ok(Plan::find()
            .filter(PlanColumn::Slug.eq(slug))
            .one(self.db)
            .await?)
    }
}
