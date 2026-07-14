use sea_orm::{DatabaseConnection, EntityTrait, ColumnTrait, QueryFilter, QueryOrder, PaginatorTrait, Set, ActiveModelTrait};
use crate::common::errors::app_error::AppError;
use crate::modules::scheduler::model::task_execution_entity::{self, Model as ExecutionModel};

pub struct TaskExecutionRepository<'a> { db: &'a DatabaseConnection }
impl<'a> TaskExecutionRepository<'a> {
    pub fn new(db: &'a DatabaseConnection) -> Self { Self { db } }
    pub async fn record(&self, task_id: i64, status: &str, result: Option<serde_json::Value>, error_message: Option<&str>) -> Result<ExecutionModel, AppError> {
        let active = task_execution_entity::ActiveModel {
            task_id: Set(task_id), status: Set(status.to_owned()),
            result: Set(result), error_message: Set(error_message.map(|s| s.to_owned())),
            started_at: Set(chrono::Utc::now().into()), ..Default::default()
        };
        Ok(active.insert(self.db).await?)
    }
    pub async fn list(&self, task_id: Option<i64>, page: i64, per_page: i64) -> Result<(Vec<ExecutionModel>, i64), AppError> {
        let page_size = per_page as u64; let page_num = if per_page > 0 { ((page - 1).max(0) as u64 * page_size) / page_size } else { 0 };
        let mut select = task_execution_entity::Entity::find();
        if let Some(tid) = task_id { select = select.filter(task_execution_entity::Column::TaskId.eq(tid)); }
        let total = select.clone().count(self.db).await? as i64;
        let items = select.order_by_desc(task_execution_entity::Column::StartedAt).paginate(self.db, page_size).fetch_page(page_num).await?;
        Ok((items, total))
    }
}
