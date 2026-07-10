use sea_orm::{*, sea_query::Expr};

use crate::common::errors::app_error::AppError;
use crate::common::utils::helpers::{total_pages, PaginatedResponse};
use crate::modules::user::model::user_entity::{self, Model as UserModel};
use crate::modules::user::model::refresh_token_entity::{self, Model as RefreshTokenModel};
use crate::modules::user::response::user_response::UserResponse;

pub struct UserRepositorySeaorm {
    db: DatabaseConnection,
}

impl UserRepositorySeaorm {
    pub fn new(db: &DatabaseConnection) -> Self {
        Self { db: db.clone() }
    }

    pub async fn find_by_id(&self, user_id: i64) -> Result<Option<UserModel>, AppError> {
        let model = user_entity::Entity::find_by_id(user_id)
            .one(&self.db)
            .await
            .map_err(AppError::DatabaseSeaorm)?;
        Ok(model)
    }

    pub async fn find_by_email(&self, email: &str) -> Result<Option<UserModel>, AppError> {
        let model = user_entity::Entity::find()
            .filter(user_entity::Column::Email.eq(email))
            .one(&self.db)
            .await
            .map_err(AppError::DatabaseSeaorm)?;
        Ok(model)
    }

    pub async fn find_by_phone(&self, phone: &str) -> Result<Option<UserModel>, AppError> {
        let model = user_entity::Entity::find()
            .filter(user_entity::Column::Phone.eq(phone))
            .one(&self.db)
            .await
            .map_err(AppError::DatabaseSeaorm)?;
        Ok(model)
    }

    pub async fn create(
        &self,
        email: &str,
        password_hash: &str,
        name: &str,
        phone: &str,
        role_id: i64,
        branch_id: Option<i64>,
        is_company_wide: bool,
    ) -> Result<UserModel, AppError> {
        let active = user_entity::ActiveModel {
            email: Set(email.to_string()),
            password_hash: Set(password_hash.to_string()),
            name: Set(name.to_string()),
            phone: Set(Some(phone.to_string())),
            role_id: Set(role_id),
            branch_id: Set(branch_id),
            is_company_wide: Set(is_company_wide),
            ..Default::default()
        };
        let model = active
            .insert(&self.db)
            .await
            .map_err(AppError::DatabaseSeaorm)?;
        Ok(model)
    }

    pub async fn update(
        &self,
        user_id: i64,
        name: Option<&str>,
        phone: Option<&str>,
        branch_id: Option<i64>,
        avatar_url: Option<&str>,
    ) -> Result<UserModel, AppError> {
        let model = user_entity::Entity::find_by_id(user_id)
            .one(&self.db)
            .await
            .map_err(AppError::DatabaseSeaorm)?
            .ok_or_else(|| AppError::NotFound("User not found".into()))?;

        let mut active: user_entity::ActiveModel = model.into();
        if let Some(v) = name { active.name = Set(v.to_string()); }
        if let Some(v) = phone { active.phone = Set(Some(v.to_string())); }
        if let Some(v) = avatar_url { active.avatar_url = Set(Some(v.to_string())); }
        if branch_id.is_some() { active.branch_id = Set(branch_id); }
        active.updated_at = Set(chrono::Utc::now().into());

        let updated = active
            .update(&self.db)
            .await
            .map_err(AppError::DatabaseSeaorm)?;
        Ok(updated)
    }

    pub async fn soft_delete(&self, user_id: i64) -> Result<(), AppError> {
        let model = user_entity::Entity::find_by_id(user_id)
            .one(&self.db)
            .await
            .map_err(AppError::DatabaseSeaorm)?
            .ok_or_else(|| AppError::NotFound("User not found".into()))?;

        let mut active: user_entity::ActiveModel = model.into();
        active.is_active = Set(false);
        active.updated_at = Set(chrono::Utc::now().into());
        active.update(&self.db).await.map_err(AppError::DatabaseSeaorm)?;
        Ok(())
    }

    pub async fn update_status(&self, user_id: i64, is_active: bool) -> Result<UserModel, AppError> {
        let model = user_entity::Entity::find_by_id(user_id)
            .one(&self.db)
            .await
            .map_err(AppError::DatabaseSeaorm)?
            .ok_or_else(|| AppError::NotFound("User not found".into()))?;

        let mut active: user_entity::ActiveModel = model.into();
        active.is_active = Set(is_active);
        active.updated_at = Set(chrono::Utc::now().into());

        let updated = active
            .update(&self.db)
            .await
            .map_err(AppError::DatabaseSeaorm)?;
        Ok(updated)
    }

    pub async fn update_last_login(&self, user_id: i64) -> Result<(), AppError> {
        let model = user_entity::Entity::find_by_id(user_id)
            .one(&self.db)
            .await
            .map_err(AppError::DatabaseSeaorm)?
            .ok_or_else(|| AppError::NotFound("User not found".into()))?;

        let mut active: user_entity::ActiveModel = model.into();
        active.last_login_at = Set(Some(chrono::Utc::now().into()));
        active.failed_attempts = Set(0);
        active.updated_at = Set(chrono::Utc::now().into());
        active.update(&self.db).await.map_err(AppError::DatabaseSeaorm)?;
        Ok(())
    }

    pub async fn increment_failed_attempts(&self, user_id: i64) -> Result<i32, AppError> {
        let model = user_entity::Entity::find_by_id(user_id)
            .one(&self.db)
            .await
            .map_err(AppError::DatabaseSeaorm)?
            .ok_or_else(|| AppError::NotFound("User not found".into()))?;

        let mut active: user_entity::ActiveModel = model.into();
        let new_count = active.failed_attempts.clone().unwrap() + 1;
        active.failed_attempts = Set(new_count);
        active.updated_at = Set(chrono::Utc::now().into());

        if new_count >= 5 {
            active.is_locked = Set(true);
            active.locked_until = Set(Some((chrono::Utc::now() + chrono::Duration::minutes(30)).into()));
        }

        active.update(&self.db).await.map_err(AppError::DatabaseSeaorm)?;
        Ok(new_count)
    }

    pub async fn update_password(&self, user_id: i64, new_hash: &str) -> Result<(), AppError> {
        let model = user_entity::Entity::find_by_id(user_id)
            .one(&self.db)
            .await
            .map_err(AppError::DatabaseSeaorm)?
            .ok_or_else(|| AppError::NotFound("User not found".into()))?;

        let mut active: user_entity::ActiveModel = model.into();
        active.password_hash = Set(new_hash.to_string());
        active.updated_at = Set(chrono::Utc::now().into());
        active.update(&self.db).await.map_err(AppError::DatabaseSeaorm)?;
        Ok(())
    }

    pub async fn list(
        &self,
        offset: u32,
        limit: u32,
        role_id: Option<i64>,
        branch_id: Option<i64>,
        is_active: Option<bool>,
        search: Option<&str>,
    ) -> Result<PaginatedResponse<UserResponse>, AppError> {
        let page_size = limit.max(1) as u64;
        let page_num = ((offset / limit) + 1).max(1) as u64;

        let mut select = user_entity::Entity::find();
        if let Some(rid) = role_id {
            select = select.filter(user_entity::Column::RoleId.eq(rid));
        }
        if let Some(bid) = branch_id {
            select = select.filter(user_entity::Column::BranchId.eq(bid));
        }
        if let Some(active) = is_active {
            select = select.filter(user_entity::Column::IsActive.eq(active));
        }
        if let Some(s) = search {
            let pattern = format!("%{s}%");
            select = select.filter(
                Condition::any()
                    .add(user_entity::Column::Name.contains(&pattern))
                    .add(user_entity::Column::Email.contains(&pattern))
                    .add(user_entity::Column::Phone.contains(&pattern)),
            );
        }

        let paginator = select
            .order_by_desc(user_entity::Column::CreatedAt)
            .paginate(&self.db, page_size);

        let total = paginator
            .num_items()
            .await
            .map_err(AppError::DatabaseSeaorm)? as i64;

        let models = paginator
            .fetch_page(page_num - 1)
            .await
            .map_err(AppError::DatabaseSeaorm)?;

        let data = models.into_iter().map(|m| UserResponse::from_model(m, None)).collect();
        let tp = total_pages(total, limit);
        Ok(PaginatedResponse {
            data,
            total,
            page: page_num as u32,
            limit,
            total_pages: tp,
        })
    }

    pub async fn email_exists(&self, email: &str, exclude: Option<i64>) -> Result<bool, AppError> {
        let mut select = user_entity::Entity::find()
            .filter(user_entity::Column::Email.eq(email));
        if let Some(id) = exclude {
            select = select.filter(user_entity::Column::Id.ne(id));
        }
        let count = select.count(&self.db).await.map_err(AppError::DatabaseSeaorm)?;
        Ok(count > 0)
    }

    pub async fn phone_exists(&self, phone: &str, exclude: Option<i64>) -> Result<bool, AppError> {
        let mut select = user_entity::Entity::find()
            .filter(user_entity::Column::Phone.eq(phone));
        if let Some(id) = exclude {
            select = select.filter(user_entity::Column::Id.ne(id));
        }
        let count = select.count(&self.db).await.map_err(AppError::DatabaseSeaorm)?;
        Ok(count > 0)
    }

    // ── Refresh Token queries ────────────────────────────────

    pub async fn create_refresh_token(
        &self,
        user_id: i64,
        token_hash: &str,
        device_info: Option<&str>,
        ip_address: Option<&str>,
        expires_at: chrono::DateTime<chrono::Utc>,
    ) -> Result<RefreshTokenModel, AppError> {
        let active = refresh_token_entity::ActiveModel {
            user_id: Set(user_id),
            token_hash: Set(token_hash.to_string()),
            device_info: Set(device_info.map(|s| s.to_string())),
            ip_address: Set(ip_address.map(|s| s.to_string())),
            expires_at: Set(expires_at.into()),
            ..Default::default()
        };
        let model = active
            .insert(&self.db)
            .await
            .map_err(AppError::DatabaseSeaorm)?;
        Ok(model)
    }

    pub async fn find_valid_refresh_token(&self, token_hash: &str) -> Result<Option<RefreshTokenModel>, AppError> {
        let now = chrono::Utc::now();
        let model = refresh_token_entity::Entity::find()
            .filter(refresh_token_entity::Column::TokenHash.eq(token_hash))
            .filter(refresh_token_entity::Column::RevokedAt.is_null())
            .filter(refresh_token_entity::Column::ExpiresAt.gt(now))
            .one(&self.db)
            .await
            .map_err(AppError::DatabaseSeaorm)?;
        Ok(model)
    }

    pub async fn revoke_refresh_token(&self, token_hash: &str) -> Result<(), AppError> {
        let model = refresh_token_entity::Entity::find()
            .filter(refresh_token_entity::Column::TokenHash.eq(token_hash))
            .filter(refresh_token_entity::Column::RevokedAt.is_null())
            .one(&self.db)
            .await
            .map_err(AppError::DatabaseSeaorm)?;

        if let Some(model) = model {
            let mut active: refresh_token_entity::ActiveModel = model.into();
            active.revoked_at = Set(Some(chrono::Utc::now().into()));
            active.update(&self.db).await.map_err(AppError::DatabaseSeaorm)?;
        }
        Ok(())
    }

    pub async fn revoke_all_user_tokens(&self, user_id: i64) -> Result<u64, AppError> {
        let result = refresh_token_entity::Entity::update_many()
            .filter(refresh_token_entity::Column::UserId.eq(user_id))
            .filter(refresh_token_entity::Column::RevokedAt.is_null())
            .col_expr(
                refresh_token_entity::Column::RevokedAt,
                Expr::val(chrono::Utc::now()).into(),
            )
            .exec(&self.db)
            .await
            .map_err(AppError::DatabaseSeaorm)?;
        Ok(result.rows_affected)
    }

    pub async fn count_active_tokens(&self, user_id: i64) -> Result<i64, AppError> {
        let now = chrono::Utc::now();
        let count = refresh_token_entity::Entity::find()
            .filter(refresh_token_entity::Column::UserId.eq(user_id))
            .filter(refresh_token_entity::Column::RevokedAt.is_null())
            .filter(refresh_token_entity::Column::ExpiresAt.gt(now))
            .count(&self.db)
            .await
            .map_err(AppError::DatabaseSeaorm)? as i64;
        Ok(count)
    }

    pub async fn revoke_oldest_token(&self, user_id: i64) -> Result<(), AppError> {
        let model = refresh_token_entity::Entity::find()
            .filter(refresh_token_entity::Column::UserId.eq(user_id))
            .filter(refresh_token_entity::Column::RevokedAt.is_null())
            .order_by_asc(refresh_token_entity::Column::CreatedAt)
            .one(&self.db)
            .await
            .map_err(AppError::DatabaseSeaorm)?;

        if let Some(model) = model {
            let mut active: refresh_token_entity::ActiveModel = model.into();
            active.revoked_at = Set(Some(chrono::Utc::now().into()));
            active.update(&self.db).await.map_err(AppError::DatabaseSeaorm)?;
        }
        Ok(())
    }

    pub async fn list_active_sessions(&self, user_id: i64) -> Result<Vec<RefreshTokenModel>, AppError> {
        let now = chrono::Utc::now();
        let models = refresh_token_entity::Entity::find()
            .filter(refresh_token_entity::Column::UserId.eq(user_id))
            .filter(refresh_token_entity::Column::RevokedAt.is_null())
            .filter(refresh_token_entity::Column::ExpiresAt.gt(now))
            .order_by_desc(refresh_token_entity::Column::CreatedAt)
            .all(&self.db)
            .await
            .map_err(AppError::DatabaseSeaorm)?;
        Ok(models)
    }
}
