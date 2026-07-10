//! SeaORM-based repository for the Permission domain.
//! Zero plain SQL — all queries use EntityTrait, ActiveModelTrait, and Select.

use sea_orm::{
    ActiveModelTrait, ColumnTrait, DatabaseConnection, EntityTrait, IntoActiveModel,
    PaginatorTrait, QueryFilter, QueryOrder, Set,
};

use crate::common::errors::app_error::AppError;
use crate::common::utils::helpers::{total_pages, PaginatedResponse};
use crate::modules::permission::model::permission_entity::{self, Model as PermissionModel};
use crate::modules::permission::response::permission_response::PermissionResponse;

pub struct PermissionRepository<'a> {
    db: &'a DatabaseConnection,
}

impl<'a> PermissionRepository<'a> {
    pub fn new(db: &'a DatabaseConnection) -> Self { Self { db } }

    pub async fn find_by_id(&self, id: i64) -> Result<Option<PermissionModel>, AppError> {
        let model = permission_entity::Entity::find_by_id(id).one(self.db).await?;
        Ok(model)
    }

    pub async fn find_by_name_method_url(&self, name: &str, method: &str, api_url: &str) -> Result<Option<PermissionModel>, AppError> {
        let model = permission_entity::Entity::find()
            .filter(permission_entity::Column::Name.eq(name))
            .filter(permission_entity::Column::Method.eq(method))
            .filter(permission_entity::Column::ApiUrl.eq(api_url))
            .one(self.db).await?;
        Ok(model)
    }

    pub async fn list(&self, offset: u32, limit: u32, module: Option<&str>) -> Result<PaginatedResponse<PermissionResponse>, AppError> {
        let page_size = (limit.min(100)) as u64;
        let mut select = permission_entity::Entity::find();
        if let Some(m) = module {
            select = select.filter(permission_entity::Column::Module.eq(m));
        }
        let total = select.clone().count(self.db).await?;
        let total_i64 = total as i64;
        let page_num = if limit > 0 { (offset / limit) as u64 } else { 0 };
        let models = select
            .order_by_asc(permission_entity::Column::Name)
            .paginate(self.db, page_size)
            .fetch_page(page_num).await?;
        let perms: Vec<PermissionResponse> = models.into_iter().map(PermissionResponse::from_model).collect();
        let tp = total_pages(total_i64, limit);
        Ok(PaginatedResponse { data: perms, total: total_i64, page: page_num as u32 + 1, limit, total_pages: tp })
    }

    pub async fn create(&self, name: &str, method: &str, api_url: &str, guard: &str, module: &str) -> Result<PermissionModel, AppError> {
        // Upsert: insert if not exists, then fetch
        let existing = self.find_by_name_method_url(name, method, api_url).await?;
        if let Some(p) = existing { return Ok(p); }

        let now = chrono::Utc::now();
        let active_model = permission_entity::ActiveModel {
            name: Set(name.to_owned()),
            method: Set(method.to_owned()),
            api_url: Set(api_url.to_owned()),
            guard: Set(guard.to_owned()),
            module: Set(module.to_owned()),
            created_at: Set(now.into()),
            ..Default::default()
        };
        let model = active_model.insert(self.db).await?;
        Ok(model)
    }

    pub async fn delete(&self, id: i64) -> Result<(), AppError> {
        let existing = permission_entity::Entity::find_by_id(id).one(self.db).await?
            .ok_or_else(|| AppError::NotFound("Permission not found".into()))?;
        existing.into_active_model().delete(self.db).await?;
        Ok(())
    }
}
