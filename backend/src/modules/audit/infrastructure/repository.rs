use sea_orm::{
    ActiveModelTrait, ColumnTrait, DatabaseConnection, EntityTrait, PaginatorTrait, QueryFilter,
    QueryOrder, QuerySelect, Set,
};

use crate::modules::audit::domain::entities::audit_log;
use crate::shared::errors::AppError;

pub struct AuditRepository<'a> {
    db: &'a DatabaseConnection,
}

impl<'a> AuditRepository<'a> {
    pub fn new(db: &'a DatabaseConnection) -> Self {
        Self { db }
    }

    pub async fn find_by_id(&self, id: i64) -> Result<Option<audit_log::Model>, AppError> {
        Ok(audit_log::Entity::find_by_id(id).one(self.db).await?)
    }

    pub async fn search_logs(
        &self,
        user_id: Option<i64>,
        action: Option<&str>,
        resource_type: Option<&str>,
        result: Option<&str>,
        limit: i64,
        offset: i64,
    ) -> Result<Vec<audit_log::Model>, AppError> {
        let mut query = audit_log::Entity::find();
        if let Some(uid) = user_id {
            query = query.filter(audit_log::Column::UserId.eq(uid));
        }
        if let Some(a) = action {
            query = query.filter(audit_log::Column::Action.contains(a));
        }
        if let Some(rt) = resource_type {
            query = query.filter(audit_log::Column::ResourceType.eq(rt));
        }
        if let Some(r) = result {
            query = query.filter(audit_log::Column::Result.eq(r));
        }
        Ok(query
            .order_by_desc(audit_log::Column::CreatedAt)
            .limit(limit as u64)
            .offset(offset as u64)
            .all(self.db)
            .await?)
    }

    pub async fn count_logs(
        &self,
        user_id: Option<i64>,
        action: Option<&str>,
    ) -> Result<u64, AppError> {
        let mut query = audit_log::Entity::find();
        if let Some(uid) = user_id {
            query = query.filter(audit_log::Column::UserId.eq(uid));
        }
        if let Some(a) = action {
            query = query.filter(audit_log::Column::Action.contains(a));
        }
        Ok(query.count(self.db).await?)
    }

    pub async fn insert_log(
        &self,
        user_id: Option<i64>,
        user_email: Option<String>,
        user_role: Option<String>,
        action: String,
        resource_type: Option<String>,
        resource_id: Option<String>,
        ip_address: Option<String>,
        user_agent: Option<String>,
        result: String,
        old_data: Option<serde_json::Value>,
        new_data: Option<serde_json::Value>,
        metadata: Option<serde_json::Value>,
    ) -> Result<audit_log::Model, AppError> {
        let model = audit_log::ActiveModel {
            user_id: Set(user_id),
            user_email: Set(user_email),
            user_role: Set(user_role),
            action: Set(action),
            resource_type: Set(resource_type),
            resource_id: Set(resource_id),
            ip_address: Set(ip_address),
            user_agent: Set(user_agent),
            result: Set(result),
            old_data: Set(old_data),
            new_data: Set(new_data),
            metadata: Set(metadata),
            created_at: Set(chrono::Utc::now()),
            ..Default::default()
        };
        Ok(model.insert(self.db).await?)
    }

    pub async fn get_user_activity(
        &self,
        user_id: i64,
        limit: i64,
    ) -> Result<Vec<audit_log::Model>, AppError> {
        Ok(audit_log::Entity::find()
            .filter(audit_log::Column::UserId.eq(user_id))
            .order_by_desc(audit_log::Column::CreatedAt)
            .limit(limit as u64)
            .all(self.db)
            .await?)
    }
}
