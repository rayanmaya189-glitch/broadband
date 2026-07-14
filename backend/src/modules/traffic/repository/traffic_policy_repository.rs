use sea_orm::{DatabaseConnection, EntityTrait, ColumnTrait, QueryFilter, Set, ActiveModelTrait};
use crate::common::errors::app_error::AppError;
use crate::modules::traffic::model::traffic_policy_entity::{self, Model as PolicyModel};
pub struct TrafficPolicyRepository<'a> { db: &'a DatabaseConnection }
impl<'a> TrafficPolicyRepository<'a> {
    pub fn new(db: &'a DatabaseConnection) -> Self { Self { db } }
    pub async fn list(&self, branch_id: Option<i64>) -> Result<Vec<PolicyModel>, AppError> {
        let mut select = traffic_policy_entity::Entity::find();
        if let Some(bid) = branch_id { select = select.filter(traffic_policy_entity::Column::BranchId.eq(bid)); }
        Ok(select.all(self.db).await?)
    }
    pub async fn create(&self, branch_id: Option<i64>, name: &str, priority: i32, criteria: serde_json::Value, action: serde_json::Value) -> Result<PolicyModel, AppError> {
        let now = chrono::Utc::now();
        let active = traffic_policy_entity::ActiveModel {
            branch_id: Set(branch_id), name: Set(name.to_owned()), priority: Set(priority),
            criteria: Set(criteria), action: Set(action), is_active: Set(true),
            created_at: Set(now.into()), updated_at: Set(now.into()), ..Default::default()
        };
        Ok(active.insert(self.db).await?)
    }
    pub async fn delete(&self, id: i64) -> Result<bool, AppError> {
        let result = traffic_policy_entity::Entity::delete_by_id(id).exec(self.db).await?;
        Ok(result.rows_affected > 0)
    }
}
