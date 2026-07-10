//! SeaORM-based repository for the EntityHistory domain.

use sea_orm::{
    ActiveModelTrait, ColumnTrait, DatabaseConnection, EntityTrait, IntoActiveModel,
    PaginatorTrait, QueryFilter, QueryOrder, Set,
};

use crate::common::errors::app_error::AppError;
use crate::modules::audit::model::entity_history_entity::{self, Model as EntityHistoryModel};

pub struct EntityHistoryRepositorySeaorm<'a> {
    db: &'a DatabaseConnection,
}

impl<'a> EntityHistoryRepositorySeaorm<'a> {
    pub fn new(db: &'a DatabaseConnection) -> Self { Self { db } }

    pub async fn search(
        &self, entity_type: Option<&str>, entity_id: Option<i64>, action: Option<&str>,
        user_id: Option<i64>, from: Option<&str>, to: Option<&str>,
        page: i64, per_page: i64,
    ) -> Result<(Vec<EntityHistoryModel>, i64), AppError> {
        let page_size = per_page as u64;
        let page_num = if per_page > 0 { ((page - 1).max(0) as u64 * page_size as u64) / page_size } else { 0 };
        let mut select = entity_history_entity::Entity::find();
        if let Some(et) = entity_type {
            select = select.filter(entity_history_entity::Column::EntityType.eq(et));
        }
        if let Some(eid) = entity_id {
            select = select.filter(entity_history_entity::Column::EntityId.eq(eid));
        }
        if let Some(a) = action {
            select = select.filter(entity_history_entity::Column::Action.eq(a));
        }
        if let Some(uid) = user_id {
            select = select.filter(entity_history_entity::Column::UserId.eq(uid));
        }
        let total = select.clone().count(self.db).await?;
        let entries = select
            .order_by_desc(entity_history_entity::Column::CreatedAt)
            .paginate(self.db, page_size)
            .fetch_page(page_num).await?;
        Ok((entries, total as i64))
    }

    pub async fn get_by_id(&self, id: i64) -> Result<Option<EntityHistoryModel>, AppError> {
        Ok(entity_history_entity::Entity::find_by_id(id).one(self.db).await?)
    }

    pub async fn get_entity_history(&self, entity_type: &str, entity_id: i64) -> Result<Vec<EntityHistoryModel>, AppError> {
        let entries = entity_history_entity::Entity::find()
            .filter(entity_history_entity::Column::EntityType.eq(entity_type))
            .filter(entity_history_entity::Column::EntityId.eq(entity_id))
            .order_by_desc(entity_history_entity::Column::CreatedAt)
            .all(self.db).await?;
        Ok(entries)
    }

    pub async fn record_change(
        &self, entity_type: &str, entity_id: i64, action: &str,
        old_data: Option<serde_json::Value>, new_data: Option<serde_json::Value>,
        changed_fields: Option<Vec<String>>, user_id: Option<i64>,
        branch_id: Option<i64>, reason: Option<&str>,
    ) -> Result<EntityHistoryModel, AppError> {
        let now = chrono::Utc::now();
        let active = entity_history_entity::ActiveModel {
            entity_type: Set(entity_type.to_owned()),
            entity_id: Set(entity_id),
            action: Set(action.to_owned()),
            old_data: Set(old_data),
            new_data: Set(new_data),
            changed_fields: Set(changed_fields),
            user_id: Set(user_id),
            branch_id: Set(branch_id),
            reason: Set(reason.map(|s| s.to_owned())),
            created_at: Set(now.into()),
            ..Default::default()
        };
        let model = active.insert(self.db).await?;
        Ok(model)
    }

    pub async fn rollback(&self, history_id: i64, user_id: i64, reason: &str) -> Result<EntityHistoryModel, AppError> {
        let entry = entity_history_entity::Entity::find_by_id(history_id)
            .one(self.db).await?
            .ok_or_else(|| AppError::NotFound("History entry not found".into()))?;
        let old_data = entry.old_data.clone()
            .ok_or_else(|| AppError::Validation("Cannot rollback: no previous state available".into()))?;

        // Perform the actual rollback using raw SQL for dynamic table updates
        if let Some(obj) = old_data.as_object() {
            let table_name = entry.entity_type.replace('-', "_");
            let mut set_parts = Vec::new();
            for key in obj.keys() {
                if key == "id" || key == "created_at" || key == "updated_at" { continue; }
                set_parts.push(format!("{} = $3->>'{}'", key, key));
            }
            if !set_parts.is_empty() {
                let update_sql = format!(
                    "UPDATE {} SET {}, updated_at = NOW() WHERE id = $2",
                    table_name, set_parts.join(", ")
                );
                let stmt = sea_orm::Statement::from_string(
                    sea_orm::DatabaseBackend::Postgres, update_sql
                );
                sea_orm::Entity::find().from_raw_sql(stmt).all(self.db).await?;
            }
        }

        // Record rollback history
        let now = chrono::Utc::now();
        let active = entity_history_entity::ActiveModel {
            entity_type: Set(entry.entity_type.clone()),
            entity_id: Set(entry.entity_id),
            action: Set("rollback".to_owned()),
            old_data: Set(entry.new_data.clone()),
            new_data: Set(Some(old_data)),
            changed_fields: Set(entry.changed_fields.clone()),
            user_id: Set(Some(user_id)),
            reason: Set(Some(reason.to_owned())),
            rollback_reference: Set(Some(history_id)),
            created_at: Set(now.into()),
            ..Default::default()
        };
        let rollback_entry = active.insert(self.db).await?;
        Ok(rollback_entry)
    }
}
