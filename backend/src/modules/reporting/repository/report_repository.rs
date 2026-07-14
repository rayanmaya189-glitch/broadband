use sea_orm::{DatabaseConnection, EntityTrait, ColumnTrait, QueryFilter, QueryOrder, PaginatorTrait, Set, ActiveModelTrait, IntoActiveModel};
use crate::common::errors::app_error::AppError;
use crate::modules::reporting::model::report_entity::{self, Model as ReportModel};

pub struct ReportRepository<'a> { db: &'a DatabaseConnection }
impl<'a> ReportRepository<'a> {
    pub fn new(db: &'a DatabaseConnection) -> Self { Self { db } }
    pub async fn list(&self, branch_id: Option<i64>, report_type: Option<&str>, page: i64, per_page: i64) -> Result<(Vec<ReportModel>, i64), AppError> {
        let page_size = per_page as u64;
        let page_num = if per_page > 0 { ((page - 1).max(0) as u64 * page_size) / page_size } else { 0 };
        let mut select = report_entity::Entity::find();
        if let Some(bid) = branch_id { select = select.filter(report_entity::Column::BranchId.eq(bid)); }
        if let Some(rt) = report_type { select = select.filter(report_entity::Column::ReportType.eq(rt)); }
        let total = select.clone().count(self.db).await? as i64;
        let items = select.order_by_desc(report_entity::Column::CreatedAt).paginate(self.db, page_size).fetch_page(page_num).await?;
        Ok((items, total))
    }
    pub async fn get_by_id(&self, id: i64) -> Result<Option<ReportModel>, AppError> {
        Ok(report_entity::Entity::find_by_id(id).one(self.db).await?)
    }
    pub async fn create(&self, branch_id: Option<i64>, user_id: i64, report_type: &str, name: &str, parameters: Option<serde_json::Value>) -> Result<ReportModel, AppError> {
        let active = report_entity::ActiveModel {
            branch_id: Set(branch_id), user_id: Set(user_id),
            report_type: Set(report_type.to_owned()), name: Set(name.to_owned()),
            parameters: Set(parameters), status: Set("pending".to_owned()),
            created_at: Set(chrono::Utc::now().into()), ..Default::default()
        };
        Ok(active.insert(self.db).await?)
    }
    pub async fn update_status(&self, id: i64, status: &str, result: Option<serde_json::Value>, file_url: Option<&str>) -> Result<ReportModel, AppError> {
        let existing = report_entity::Entity::find_by_id(id).one(self.db).await?.ok_or_else(|| AppError::NotFound("Report not found".into()))?;
        let mut active = existing.into_active_model();
        active.status = Set(status.to_owned());
        active.result = Set(result);
        active.file_url = Set(file_url.map(|s| s.to_owned()));
        if status == "completed" { active.completed_at = Set(Some(chrono::Utc::now().into())); }
        Ok(active.update(self.db).await?)
    }
}
