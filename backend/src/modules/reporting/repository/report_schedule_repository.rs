use sea_orm::{DatabaseConnection, EntityTrait, ColumnTrait, QueryFilter, Set, ActiveModelTrait};
use crate::common::errors::app_error::AppError;
use crate::modules::reporting::model::report_schedule_entity::{self, Model as ScheduleModel};

pub struct ReportScheduleRepository<'a> { db: &'a DatabaseConnection }
impl<'a> ReportScheduleRepository<'a> {
    pub fn new(db: &'a DatabaseConnection) -> Self { Self { db } }
    pub async fn list(&self, branch_id: Option<i64>) -> Result<Vec<ScheduleModel>, AppError> {
        let mut select = report_schedule_entity::Entity::find();
        if let Some(bid) = branch_id { select = select.filter(report_schedule_entity::Column::BranchId.eq(bid)); }
        Ok(select.all(self.db).await?)
    }
    pub async fn create(&self, branch_id: Option<i64>, user_id: i64, report_type: &str, name: &str, parameters: Option<serde_json::Value>, frequency: &str) -> Result<ScheduleModel, AppError> {
        let active = report_schedule_entity::ActiveModel {
            branch_id: Set(branch_id), user_id: Set(user_id),
            report_type: Set(report_type.to_owned()), name: Set(name.to_owned()),
            parameters: Set(parameters), frequency: Set(frequency.to_owned()),
            is_active: Set(true), created_at: Set(chrono::Utc::now().into()), ..Default::default()
        };
        Ok(active.insert(self.db).await?)
    }
    pub async fn delete(&self, id: i64) -> Result<bool, AppError> {
        let result = report_schedule_entity::Entity::delete_by_id(id).exec(self.db).await?;
        Ok(result.rows_affected > 0)
    }
}
