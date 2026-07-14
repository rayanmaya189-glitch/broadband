use sea_orm::{DatabaseConnection, EntityTrait, ColumnTrait, QueryFilter, QueryOrder, Set, ActiveModelTrait, IntoActiveModel};
use crate::common::errors::app_error::AppError;
use crate::modules::scheduler::model::scheduled_task_entity::{self, Model as TaskModel};

pub struct ScheduledTaskRepository<'a> { db: &'a DatabaseConnection }
impl<'a> ScheduledTaskRepository<'a> {
    pub fn new(db: &'a DatabaseConnection) -> Self { Self { db } }
    pub async fn list(&self, branch_id: Option<i64>) -> Result<Vec<TaskModel>, AppError> {
        let mut select = scheduled_task_entity::Entity::find();
        if let Some(bid) = branch_id { select = select.filter(scheduled_task_entity::Column::BranchId.eq(bid)); }
        Ok(select.order_by_asc(scheduled_task_entity::Column::NextRunAt).all(self.db).await?)
    }
    pub async fn get_by_id(&self, id: i64) -> Result<Option<TaskModel>, AppError> {
        Ok(scheduled_task_entity::Entity::find_by_id(id).one(self.db).await?)
    }
    pub async fn create(&self, branch_id: Option<i64>, name: &str, task_type: &str, config: Option<serde_json::Value>, schedule_type: &str, schedule_value: &str) -> Result<TaskModel, AppError> {
        let now = chrono::Utc::now();
        let active = scheduled_task_entity::ActiveModel {
            branch_id: Set(branch_id), name: Set(name.to_owned()), task_type: Set(task_type.to_owned()),
            config: Set(config), schedule_type: Set(schedule_type.to_owned()), schedule_value: Set(schedule_value.to_owned()),
            is_active: Set(true), created_at: Set(now.into()), updated_at: Set(now.into()), ..Default::default()
        };
        Ok(active.insert(self.db).await?)
    }
    pub async fn toggle_active(&self, id: i64) -> Result<TaskModel, AppError> {
        let existing = scheduled_task_entity::Entity::find_by_id(id).one(self.db).await?.ok_or_else(|| AppError::NotFound("Task not found".into()))?;
        let mut active = existing.into_active_model();
        active.is_active = Set(!active.is_active.clone().unwrap());
        active.updated_at = Set(chrono::Utc::now().into());
        Ok(active.update(self.db).await?)
    }
    pub async fn delete(&self, id: i64) -> Result<bool, AppError> {
        let result = scheduled_task_entity::Entity::delete_by_id(id).exec(self.db).await?;
        Ok(result.rows_affected > 0)
    }
}
