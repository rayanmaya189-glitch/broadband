//! SeaORM-based repository for the Role domain.
//! Zero plain SQL — all queries use EntityTrait, ActiveModelTrait, and Select.

use sea_orm::{
    ActiveModelTrait, ColumnTrait, DatabaseConnection, EntityTrait, IntoActiveModel,
    PaginatorTrait, QueryFilter, QueryOrder, Set,
};

use crate::common::errors::app_error::AppError;
use crate::common::utils::helpers::{total_pages, PaginatedResponse};
use crate::modules::role::model::role_entity::{self, Model as RoleModel};
use crate::modules::role::model::role_permission_entity;
use crate::modules::role::model::user_role_entity;
use crate::modules::role::response::role_response::RoleResponse;

pub struct RoleRepositorySeaorm<'a> {
    db: &'a DatabaseConnection,
}

impl<'a> RoleRepositorySeaorm<'a> {
    pub fn new(db: &'a DatabaseConnection) -> Self { Self { db } }

    pub async fn find_by_id(&self, role_id: i64) -> Result<Option<RoleModel>, AppError> {
        let model = role_entity::Entity::find_by_id(role_id).one(self.db).await?;
        Ok(model)
    }

    pub async fn find_by_name(&self, name: &str) -> Result<Option<RoleModel>, AppError> {
        let model = role_entity::Entity::find()
            .filter(role_entity::Column::Name.eq(name))
            .one(self.db).await?;
        Ok(model)
    }

    pub async fn list(&self, offset: u32, limit: u32, is_active: Option<bool>) -> Result<PaginatedResponse<RoleResponse>, AppError> {
        let page_size = (limit.min(100)) as u64;
        let mut select = role_entity::Entity::find();
        if let Some(v) = is_active {
            select = select.filter(role_entity::Column::IsActive.eq(v));
        }
        let total = select.clone().count(self.db).await?;
        let total_i64 = total as i64;
        let page_num = if limit > 0 { (offset / limit) as u64 } else { 0 };
        let models = select
            .order_by_asc(role_entity::Column::Name)
            .paginate(self.db, page_size)
            .fetch_page(page_num).await?;
        let roles: Vec<RoleResponse> = models.into_iter().map(RoleResponse::from_model).collect();
        let tp = total_pages(total_i64, limit);
        Ok(PaginatedResponse { data: roles, total: total_i64, page: page_num as u32 + 1, limit, total_pages: tp })
    }

    pub async fn create(&self, name: &str, display_name: &str, description: Option<&str>) -> Result<RoleModel, AppError> {
        let now = chrono::Utc::now();
        let active_model = role_entity::ActiveModel {
            name: Set(name.to_owned()),
            display_name: Set(display_name.to_owned()),
            description: Set(description.map(|s| s.to_owned())),
            is_system: Set(false),
            is_active: Set(true),
            created_at: Set(now.into()),
            updated_at: Set(now.into()),
            ..Default::default()
        };
        let model = active_model.insert(self.db).await?;
        Ok(model)
    }

    pub async fn update(&self, role_id: i64, name: Option<&str>, display_name: Option<&str>, description: Option<&str>) -> Result<RoleModel, AppError> {
        let existing = role_entity::Entity::find_by_id(role_id).one(self.db).await?
            .ok_or_else(|| AppError::NotFound("Role not found".into()))?;
        let mut active = existing.into_active_model();
        if let Some(v) = name { active.name = Set(v.to_owned()); }
        if let Some(v) = display_name { active.display_name = Set(v.to_owned()); }
        if let Some(v) = description { active.description = Set(Some(v.to_owned())); }
        active.updated_at = Set(chrono::Utc::now().into());
        let model = active.update(self.db).await?;
        Ok(model)
    }

    pub async fn deactivate(&self, role_id: i64) -> Result<(), AppError> {
        let existing = role_entity::Entity::find_by_id(role_id).one(self.db).await?
            .ok_or_else(|| AppError::NotFound("Role not found".into()))?;
        if existing.is_system {
            return Err(AppError::Forbidden("Cannot deactivate system role".into()));
        }
        let mut active = existing.into_active_model();
        active.is_active = Set(false);
        active.updated_at = Set(chrono::Utc::now().into());
        active.update(self.db).await?;
        Ok(())
    }

    pub async fn name_exists(&self, name: &str, exclude: Option<i64>) -> Result<bool, AppError> {
        let mut select = role_entity::Entity::find().filter(role_entity::Column::Name.eq(name));
        if let Some(id) = exclude {
            select = select.filter(role_entity::Column::Id.ne(id));
        }
        let count = select.count(self.db).await?;
        Ok(count > 0)
    }

    // ── Permission Assignment ──────────────────────────────

    pub async fn assign_permissions(&self, role_id: i64, permission_ids: &[i64]) -> Result<(), AppError> {
        // Batch delete existing permissions for this role
        role_permission_entity::Entity::delete_many()
            .filter(role_permission_entity::Column::RoleId.eq(role_id))
            .exec(self.db).await?;

        // Batch insert new permissions
        let active_models: Vec<role_permission_entity::ActiveModel> = permission_ids
            .iter()
            .map(|pid| role_permission_entity::ActiveModel {
                role_id: Set(role_id),
                permission_id: Set(*pid),
            })
            .collect();
        for active in active_models {
            active.insert(self.db).await?;
        }
        Ok(())
    }

    pub async fn remove_permission(&self, role_id: i64, permission_id: i64) -> Result<(), AppError> {
        role_permission_entity::Entity::delete_many()
            .filter(role_permission_entity::Column::RoleId.eq(role_id))
            .filter(role_permission_entity::Column::PermissionId.eq(permission_id))
            .exec(self.db).await?;
        Ok(())
    }

    // ── User-Role Management ───────────────────────────────

    pub async fn list_user_roles(&self, user_id: i64) -> Result<Vec<RoleResponse>, AppError> {
        let ur_models = user_role_entity::Entity::find()
            .filter(user_role_entity::Column::UserId.eq(user_id))
            .filter(user_role_entity::Column::IsActive.eq(true))
            .all(self.db).await?;
        let role_ids: Vec<i64> = ur_models.iter().map(|ur| ur.role_id).collect();
        if role_ids.is_empty() {
            return Ok(Vec::new());
        }
        let roles = role_entity::Entity::find()
            .filter(role_entity::Column::Id.is_in(role_ids))
            .order_by_asc(role_entity::Column::Name)
            .all(self.db).await?;
        Ok(roles.into_iter().map(RoleResponse::from_model).collect())
    }

    pub async fn assign_role_to_user(&self, user_id: i64, role_id: i64) -> Result<(), AppError> {
        let existing = user_role_entity::Entity::find()
            .filter(user_role_entity::Column::UserId.eq(user_id))
            .filter(user_role_entity::Column::RoleId.eq(role_id))
            .one(self.db).await?;
        if let Some(ur) = existing {
            let mut active = ur.into_active_model();
            active.is_active = Set(true);
            active.update(self.db).await?;
        } else {
            let now = chrono::Utc::now();
            let active = user_role_entity::ActiveModel {
                user_id: Set(user_id),
                role_id: Set(role_id),
                is_active: Set(true),
                expires_at: Set(None),
                created_at: Set(now.into()),
            };
            active.insert(self.db).await?;
        }
        Ok(())
    }

    pub async fn revoke_role_from_user(&self, user_id: i64, role_id: i64) -> Result<(), AppError> {
        let existing = user_role_entity::Entity::find()
            .filter(user_role_entity::Column::UserId.eq(user_id))
            .filter(user_role_entity::Column::RoleId.eq(role_id))
            .one(self.db).await?;
        if let Some(ur) = existing {
            let mut active = ur.into_active_model();
            active.is_active = Set(false);
            active.update(self.db).await?;
        }
        Ok(())
    }
}
