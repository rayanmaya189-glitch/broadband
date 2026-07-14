use sea_orm::{DatabaseConnection, EntityTrait, ColumnTrait, QueryFilter, QueryOrder, PaginatorTrait, Set, ActiveModelTrait, IntoActiveModel};
use crate::common::errors::app_error::AppError;
use crate::modules::monitoring::model::alert_entity::{self, Model as AlertModel};
pub struct AlertRepository<'a> { db: &'a DatabaseConnection }
impl<'a> AlertRepository<'a> {
    pub fn new(db: &'a DatabaseConnection) -> Self { Self { db } }
    pub async fn list(&self, status: Option<&str>, severity: Option<&str>, page: i64, per_page: i64) -> Result<(Vec<AlertModel>, i64), AppError> {
        let page_size = per_page as u64; let page_num = if per_page > 0 { ((page - 1).max(0) as u64 * page_size) / page_size } else { 0 };
        let mut select = alert_entity::Entity::find();
        if let Some(s) = status { select = select.filter(alert_entity::Column::Status.eq(s)); }
        if let Some(sev) = severity { select = select.filter(alert_entity::Column::Severity.eq(sev)); }
        let total = select.clone().count(self.db).await? as i64;
        let items = select.order_by_desc(alert_entity::Column::CreatedAt).paginate(self.db, page_size).fetch_page(page_num).await?;
        Ok((items, total))
    }
    pub async fn create(&self, service_name: &str, severity: &str, message: &str, rule_id: Option<i64>) -> Result<AlertModel, AppError> {
        let active = alert_entity::ActiveModel {
            rule_id: Set(rule_id), service_name: Set(service_name.to_owned()),
            severity: Set(severity.to_owned()), message: Set(message.to_owned()),
            status: Set("open".to_owned()), created_at: Set(chrono::Utc::now().into()),
            ..Default::default()
        };
        Ok(active.insert(self.db).await?)
    }
    pub async fn acknowledge(&self, id: i64, user_id: i64) -> Result<AlertModel, AppError> {
        let existing = alert_entity::Entity::find_by_id(id).one(self.db).await?.ok_or_else(|| AppError::NotFound("Alert not found".into()))?;
        let mut active = existing.into_active_model();
        active.status = Set("acknowledged".to_owned());
        active.acknowledged_by = Set(Some(user_id));
        active.acknowledged_at = Set(Some(chrono::Utc::now().into()));
        Ok(active.update(self.db).await?)
    }
    pub async fn resolve(&self, id: i64) -> Result<AlertModel, AppError> {
        let existing = alert_entity::Entity::find_by_id(id).one(self.db).await?.ok_or_else(|| AppError::NotFound("Alert not found".into()))?;
        let mut active = existing.into_active_model();
        active.status = Set("resolved".to_owned());
        active.resolved_at = Set(Some(chrono::Utc::now().into()));
        Ok(active.update(self.db).await?)
    }
}
