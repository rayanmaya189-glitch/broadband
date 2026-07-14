use sea_orm::{DatabaseConnection, EntityTrait, ColumnTrait, QueryFilter, Set, ActiveModelTrait};

use crate::common::errors::app_error::AppError;
use crate::modules::crm::model::tag_entity::{self, Model as TagModel};
use crate::modules::crm::model::customer_tag_entity;

pub struct TagRepository<'a> {
    db: &'a DatabaseConnection,
}

impl<'a> TagRepository<'a> {
    pub fn new(db: &'a DatabaseConnection) -> Self { Self { db } }

    pub async fn list(&self, branch_id: Option<i64>, category: Option<&str>) -> Result<Vec<TagModel>, AppError> {
        let mut select = tag_entity::Entity::find();
        if let Some(bid) = branch_id { select = select.filter(tag_entity::Column::BranchId.eq(bid)); }
        if let Some(c) = category { select = select.filter(tag_entity::Column::Category.eq(c)); }
        Ok(select.all(self.db).await?)
    }

    pub async fn create(&self, branch_id: Option<i64>, name: &str, color: Option<&str>, category: &str) -> Result<TagModel, AppError> {
        let active = tag_entity::ActiveModel {
            branch_id: Set(branch_id),
            name: Set(name.to_owned()),
            color: Set(color.map(|s| s.to_owned())),
            category: Set(category.to_owned()),
            usage_count: Set(0),
            created_at: Set(chrono::Utc::now().into()),
            ..Default::default()
        };
        Ok(active.insert(self.db).await?)
    }

    pub async fn assign_to_customer(&self, customer_id: i64, tag_id: i64) -> Result<(), AppError> {
        let active = customer_tag_entity::ActiveModel {
            customer_id: Set(customer_id),
            tag_id: Set(tag_id),
            created_at: Set(chrono::Utc::now().into()),
            ..Default::default()
        };
        active.insert(self.db).await?;
        Ok(())
    }

    pub async fn remove_from_customer(&self, customer_id: i64, tag_id: i64) -> Result<bool, AppError> {
        let result = customer_tag_entity::Entity::delete_many()
            .filter(customer_tag_entity::Column::CustomerId.eq(customer_id))
            .filter(customer_tag_entity::Column::TagId.eq(tag_id))
            .exec(self.db).await?;
        Ok(result.rows_affected > 0)
    }

    pub async fn get_customer_tags(&self, customer_id: i64) -> Result<Vec<TagModel>, AppError> {
        let links = customer_tag_entity::Entity::find()
            .filter(customer_tag_entity::Column::CustomerId.eq(customer_id))
            .all(self.db).await?;
        let tag_ids: Vec<i64> = links.into_iter().map(|l| l.tag_id).collect();
        if tag_ids.is_empty() { return Ok(vec![]); }
        Ok(tag_entity::Entity::find().filter(tag_entity::Column::Id.is_in(tag_ids)).all(self.db).await?)
    }

    pub async fn delete(&self, id: i64) -> Result<bool, AppError> {
        let result = tag_entity::Entity::delete_by_id(id).exec(self.db).await?;
        Ok(result.rows_affected > 0)
    }
}
