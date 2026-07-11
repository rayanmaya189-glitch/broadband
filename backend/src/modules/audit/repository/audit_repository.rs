//! SeaORM-based repository for the Audit domain.
//! Zero plain SQL — all queries use EntityTrait, ActiveModelTrait, and Select.

use sea_orm::{
    ActiveModelTrait, ColumnTrait, DatabaseConnection, EntityTrait, IntoActiveModel,
    PaginatorTrait, QueryFilter, QueryOrder, Set,
};
use sea_orm::sea_query::Expr;

use crate::common::errors::app_error::AppError;
use crate::modules::audit::model::audit_log_entity::{self, Model as AuditLogModel};

pub struct AuditRepository<'a> {
    db: &'a DatabaseConnection,
}

impl<'a> AuditRepository<'a> {
    pub fn new(db: &'a DatabaseConnection) -> Self { Self { db } }

    pub async fn list(
        &self, user_id: Option<i64>, action: Option<&str>, resource_type: Option<&str>,
        result: Option<&str>, from: Option<&str>, to: Option<&str>,
        page: i64, per_page: i64,
    ) -> Result<(Vec<AuditLogModel>, i64), AppError> {
        let page_size = per_page as u64;
        let page_num = if per_page > 0 { ((page - 1).max(0) as u64 * page_size as u64) / page_size } else { 0 };
        let mut select = audit_log_entity::Entity::find();
        if let Some(uid) = user_id {
            select = select.filter(audit_log_entity::Column::UserId.eq(uid));
        }
        if let Some(a) = action {
            select = select.filter(audit_log_entity::Column::Action.contains(a));
        }
        if let Some(rt) = resource_type {
            select = select.filter(audit_log_entity::Column::ResourceType.eq(rt));
        }
        if let Some(r) = result {
            select = select.filter(audit_log_entity::Column::Result.eq(r));
        }
        let total = select.clone().count(self.db).await?;
        let logs = select
            .order_by_desc(audit_log_entity::Column::CreatedAt)
            .paginate(self.db, page_size)
            .fetch_page(page_num).await?;
        Ok((logs, total as i64))
    }

    pub async fn get_by_id(&self, id: i64) -> Result<Option<AuditLogModel>, AppError> {
        Ok(audit_log_entity::Entity::find_by_id(id).one(self.db).await?)
    }

    pub async fn get_by_resource(&self, resource_type: &str, resource_id: &str) -> Result<Vec<AuditLogModel>, AppError> {
        let logs = audit_log_entity::Entity::find()
            .filter(audit_log_entity::Column::ResourceType.eq(resource_type))
            .filter(audit_log_entity::Column::ResourceId.eq(resource_id))
            .order_by_desc(audit_log_entity::Column::CreatedAt)
            .all(self.db).await?;
        Ok(logs)
    }

    pub async fn insert(
        &self, user_id: Option<i64>, email: Option<&str>, role: Option<&str>,
        action: &str, resource_type: Option<&str>, result: &str,
        old_data: Option<serde_json::Value>, new_data: Option<serde_json::Value>,
        metadata: Option<serde_json::Value>,
    ) -> Result<(), AppError> {
        let now = chrono::Utc::now();
        let active = audit_log_entity::ActiveModel {
            user_id: Set(user_id),
            user_email: Set(email.map(|s| s.to_owned())),
            user_role: Set(role.map(|s| s.to_owned())),
            action: Set(action.to_owned()),
            resource_type: Set(resource_type.map(|s| s.to_owned())),
            result: Set(result.to_owned()),
            old_data: Set(old_data),
            new_data: Set(new_data),
            metadata: Set(metadata),
            created_at: Set(now.into()),
            ..Default::default()
        };
        active.insert(self.db).await?;
        Ok(())
    }
}
