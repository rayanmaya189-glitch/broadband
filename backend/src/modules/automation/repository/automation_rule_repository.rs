use sea_orm::{DatabaseConnection, EntityTrait, ColumnTrait, QueryFilter, Set, ActiveModelTrait};
use crate::common::errors::app_error::AppError;
use crate::modules::automation::model::automation_rule_entity::{self, Model as RuleModel};
use crate::modules::automation::model::automation_trigger_entity;
use crate::modules::automation::model::automation_action_entity;

pub struct AutomationRuleRepository<'a> { db: &'a DatabaseConnection }
impl<'a> AutomationRuleRepository<'a> {
    pub fn new(db: &'a DatabaseConnection) -> Self { Self { db } }
    pub async fn list(&self, branch_id: Option<i64>) -> Result<Vec<RuleModel>, AppError> {
        let mut select = automation_rule_entity::Entity::find();
        if let Some(bid) = branch_id { select = select.filter(automation_rule_entity::Column::BranchId.eq(bid)); }
        Ok(select.all(self.db).await?)
    }
    pub async fn get_by_id(&self, id: i64) -> Result<Option<RuleModel>, AppError> {
        Ok(automation_rule_entity::Entity::find_by_id(id).one(self.db).await?)
    }
    pub async fn create(&self, branch_id: Option<i64>, name: &str, description: Option<&str>, priority: i32) -> Result<RuleModel, AppError> {
        let now = chrono::Utc::now();
        let active = automation_rule_entity::ActiveModel {
            branch_id: Set(branch_id), name: Set(name.to_owned()),
            description: Set(description.map(|s| s.to_owned())),
            priority: Set(priority), is_active: Set(true),
            created_at: Set(now.into()), updated_at: Set(now.into()), ..Default::default()
        };
        Ok(active.insert(self.db).await?)
    }
    pub async fn add_trigger(&self, rule_id: i64, trigger_type: &str, config: serde_json::Value) -> Result<(), AppError> {
        let active = automation_trigger_entity::ActiveModel {
            rule_id: Set(rule_id), trigger_type: Set(trigger_type.to_owned()),
            config: Set(config), is_active: Set(true),
            created_at: Set(chrono::Utc::now().into()), ..Default::default()
        };
        active.insert(self.db).await?;
        Ok(())
    }
    pub async fn add_action(&self, rule_id: i64, action_type: &str, config: serde_json::Value, order_index: i32) -> Result<(), AppError> {
        let active = automation_action_entity::ActiveModel {
            rule_id: Set(rule_id), action_type: Set(action_type.to_owned()),
            config: Set(config), order_index: Set(order_index), is_active: Set(true),
            created_at: Set(chrono::Utc::now().into()), ..Default::default()
        };
        active.insert(self.db).await?;
        Ok(())
    }
    pub async fn delete(&self, id: i64) -> Result<bool, AppError> {
        let result = automation_rule_entity::Entity::delete_by_id(id).exec(self.db).await?;
        Ok(result.rows_affected > 0)
    }
}
