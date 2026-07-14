use sea_orm::{DatabaseConnection, EntityTrait, Set, ActiveModelTrait};
use crate::common::errors::app_error::AppError;
use crate::modules::monitoring::model::alert_rule_entity::{self, Model as AlertRuleModel};
pub struct AlertRuleRepository<'a> { db: &'a DatabaseConnection }
impl<'a> AlertRuleRepository<'a> {
    pub fn new(db: &'a DatabaseConnection) -> Self { Self { db } }
    pub async fn list(&self) -> Result<Vec<AlertRuleModel>, AppError> {
        Ok(alert_rule_entity::Entity::find().all(self.db).await?)
    }
    pub async fn create(&self, name: &str, service_name: &str, metric_name: &str, operator: &str, threshold: f64, severity: &str) -> Result<AlertRuleModel, AppError> {
        let active = alert_rule_entity::ActiveModel {
            name: Set(name.to_owned()), service_name: Set(service_name.to_owned()),
            metric_name: Set(metric_name.to_owned()), operator: Set(operator.to_owned()),
            threshold: Set(threshold), severity: Set(severity.to_owned()),
            is_active: Set(true), created_at: Set(chrono::Utc::now().into()),
            ..Default::default()
        };
        Ok(active.insert(self.db).await?)
    }
    pub async fn delete(&self, id: i64) -> Result<bool, AppError> {
        let result = alert_rule_entity::Entity::delete_by_id(id).exec(self.db).await?;
        Ok(result.rows_affected > 0)
    }
}
