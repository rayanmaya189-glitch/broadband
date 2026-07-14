use sea_orm::{DatabaseConnection, EntityTrait, ColumnTrait, QueryFilter, Set, ActiveModelTrait};

use crate::common::errors::app_error::AppError;
use crate::modules::crm::model::segment_entity::{self, Model as SegmentModel};
use crate::modules::crm::model::customer_segment_entity;

pub struct SegmentRepository<'a> {
    db: &'a DatabaseConnection,
}

impl<'a> SegmentRepository<'a> {
    pub fn new(db: &'a DatabaseConnection) -> Self { Self { db } }

    pub async fn list(&self, branch_id: Option<i64>) -> Result<Vec<SegmentModel>, AppError> {
        let mut select = segment_entity::Entity::find();
        if let Some(bid) = branch_id { select = select.filter(segment_entity::Column::BranchId.eq(bid)); }
        Ok(select.all(self.db).await?)
    }

    pub async fn get_by_id(&self, id: i64) -> Result<Option<SegmentModel>, AppError> {
        Ok(segment_entity::Entity::find_by_id(id).one(self.db).await?)
    }

    pub async fn create(&self, branch_id: Option<i64>, name: &str, description: Option<&str>, criteria: serde_json::Value, is_dynamic: bool) -> Result<SegmentModel, AppError> {
        let now = chrono::Utc::now();
        let active = segment_entity::ActiveModel {
            branch_id: Set(branch_id),
            name: Set(name.to_owned()),
            description: Set(description.map(|s| s.to_owned())),
            criteria: Set(criteria),
            customer_count: Set(0),
            is_dynamic: Set(is_dynamic),
            created_at: Set(now.into()),
            updated_at: Set(now.into()),
            ..Default::default()
        };
        Ok(active.insert(self.db).await?)
    }

    pub async fn add_customer(&self, customer_id: i64, segment_id: i64) -> Result<(), AppError> {
        let active = customer_segment_entity::ActiveModel {
            customer_id: Set(customer_id),
            segment_id: Set(segment_id),
            added_at: Set(chrono::Utc::now().into()),
            ..Default::default()
        };
        active.insert(self.db).await?;
        Ok(())
    }

    pub async fn remove_customer(&self, customer_id: i64, segment_id: i64) -> Result<bool, AppError> {
        let result = customer_segment_entity::Entity::delete_many()
            .filter(customer_segment_entity::Column::CustomerId.eq(customer_id))
            .filter(customer_segment_entity::Column::SegmentId.eq(segment_id))
            .exec(self.db).await?;
        Ok(result.rows_affected > 0)
    }

    pub async fn delete(&self, id: i64) -> Result<bool, AppError> {
        let result = segment_entity::Entity::delete_by_id(id).exec(self.db).await?;
        Ok(result.rows_affected > 0)
    }
}
