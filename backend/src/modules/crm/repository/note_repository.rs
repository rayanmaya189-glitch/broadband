use sea_orm::{DatabaseConnection, EntityTrait, ColumnTrait, QueryFilter, QueryOrder, PaginatorTrait, Set, ActiveModelTrait, IntoActiveModel};

use crate::common::errors::app_error::AppError;
use crate::modules::crm::model::note_entity::{self, Model as NoteModel};

pub struct NoteRepository<'a> {
    db: &'a DatabaseConnection,
}

impl<'a> NoteRepository<'a> {
    pub fn new(db: &'a DatabaseConnection) -> Self { Self { db } }

    pub async fn list(&self, customer_id: i64, page: i64, per_page: i64) -> Result<(Vec<NoteModel>, i64), AppError> {
        let page_size = per_page as u64;
        let page_num = if per_page > 0 { ((page - 1).max(0) as u64 * page_size) / page_size } else { 0 };
        let select = note_entity::Entity::find().filter(note_entity::Column::CustomerId.eq(customer_id));
        let total = select.clone().count(self.db).await? as i64;
        let items = select.order_by_desc(note_entity::Column::CreatedAt).paginate(self.db, page_size).fetch_page(page_num).await?;
        Ok((items, total))
    }

    pub async fn get_by_id(&self, id: i64) -> Result<Option<NoteModel>, AppError> {
        Ok(note_entity::Entity::find_by_id(id).one(self.db).await?)
    }

    pub async fn create(&self, customer_id: i64, branch_id: i64, user_id: i64, title: &str, content: &str, priority: &str, is_pinned: bool, expires_at: Option<chrono::DateTime<chrono::Utc>>) -> Result<NoteModel, AppError> {
        let now = chrono::Utc::now();
        let active = note_entity::ActiveModel {
            customer_id: Set(customer_id),
            branch_id: Set(branch_id),
            user_id: Set(user_id),
            title: Set(title.to_owned()),
            content: Set(content.to_owned()),
            priority: Set(priority.to_owned()),
            is_pinned: Set(is_pinned),
            expires_at: Set(expires_at.map(|v| v.into())),
            created_at: Set(now.into()),
            updated_at: Set(now.into()),
            ..Default::default()
        };
        Ok(active.insert(self.db).await?)
    }

    pub async fn update(&self, id: i64, title: Option<&str>, content: Option<&str>, priority: Option<&str>, is_pinned: Option<bool>) -> Result<NoteModel, AppError> {
        let existing = note_entity::Entity::find_by_id(id).one(self.db).await?.ok_or_else(|| AppError::NotFound("Note not found".into()))?;
        let mut active = existing.into_active_model();
        if let Some(v) = title { active.title = Set(v.to_owned()); }
        if let Some(v) = content { active.content = Set(v.to_owned()); }
        if let Some(v) = priority { active.priority = Set(v.to_owned()); }
        if let Some(v) = is_pinned { active.is_pinned = Set(v); }
        active.updated_at = Set(chrono::Utc::now().into());
        Ok(active.update(self.db).await?)
    }

    pub async fn delete(&self, id: i64) -> Result<bool, AppError> {
        let result = note_entity::Entity::delete_by_id(id).exec(self.db).await?;
        Ok(result.rows_affected > 0)
    }
}
