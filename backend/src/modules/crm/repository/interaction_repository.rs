use sea_orm::{DatabaseConnection, EntityTrait, ColumnTrait, QueryFilter, QueryOrder, PaginatorTrait, Set, ActiveModelTrait};

use crate::common::errors::app_error::AppError;
use crate::modules::crm::model::interaction_entity::{self, Model as InteractionModel};

pub struct CrmInteractionRepository<'a> {
    db: &'a DatabaseConnection,
}

impl<'a> CrmInteractionRepository<'a> {
    pub fn new(db: &'a DatabaseConnection) -> Self { Self { db } }

    pub async fn list(&self, customer_id: Option<i64>, branch_id: Option<i64>, interaction_type: Option<&str>, page: i64, per_page: i64) -> Result<(Vec<InteractionModel>, i64), AppError> {
        let page_size = per_page as u64;
        let page_num = if per_page > 0 { ((page - 1).max(0) as u64 * page_size) / page_size } else { 0 };
        let mut select = interaction_entity::Entity::find();
        if let Some(cid) = customer_id { select = select.filter(interaction_entity::Column::CustomerId.eq(cid)); }
        if let Some(bid) = branch_id { select = select.filter(interaction_entity::Column::BranchId.eq(bid)); }
        if let Some(t) = interaction_type { select = select.filter(interaction_entity::Column::InteractionType.eq(t)); }
        let total = select.clone().count(self.db).await? as i64;
        let items = select.order_by_desc(interaction_entity::Column::CreatedAt).paginate(self.db, page_size).fetch_page(page_num).await?;
        Ok((items, total))
    }

    pub async fn get_by_id(&self, id: i64) -> Result<Option<InteractionModel>, AppError> {
        Ok(interaction_entity::Entity::find_by_id(id).one(self.db).await?)
    }

    pub async fn create(&self, customer_id: i64, branch_id: i64, user_id: i64, interaction_type: &str, subject: &str, body: Option<&str>, channel: &str, duration_seconds: Option<i32>, sentiment: Option<&str>, follow_up_date: Option<chrono::NaiveDate>) -> Result<InteractionModel, AppError> {
        let now = chrono::Utc::now();
        let active = interaction_entity::ActiveModel {
            customer_id: Set(customer_id),
            branch_id: Set(branch_id),
            user_id: Set(user_id),
            interaction_type: Set(interaction_type.to_owned()),
            subject: Set(subject.to_owned()),
            body: Set(body.map(|s| s.to_owned())),
            channel: Set(channel.to_owned()),
            duration_seconds: Set(duration_seconds),
            sentiment: Set(sentiment.map(|s| s.to_owned())),
            follow_up_date: Set(follow_up_date),
            follow_up_done: Set(false),
            created_at: Set(now.into()),
            updated_at: Set(now.into()),
            ..Default::default()
        };
        Ok(active.insert(self.db).await?)
    }

    pub async fn delete(&self, id: i64) -> Result<bool, AppError> {
        let result = interaction_entity::Entity::delete_by_id(id).exec(self.db).await?;
        Ok(result.rows_affected > 0)
    }
}
