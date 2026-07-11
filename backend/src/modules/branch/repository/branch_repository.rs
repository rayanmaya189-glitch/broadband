use sea_orm::*;

use crate::common::errors::app_error::AppError;
use crate::common::utils::helpers::{total_pages, PaginatedResponse};
use crate::modules::branch::model::branch_entity::{self, Model as BranchModel};
use crate::modules::branch::model::branch_working_hour_entity::{self, Model as WorkingHourModel};
use crate::modules::branch::model::user_branch_entity::{self, Model as UserBranchModel};
use crate::modules::branch::response::branch_response::*;

pub struct BranchRepository {
    db: DatabaseConnection,
}

impl BranchRepository {
    pub fn new(db: &DatabaseConnection) -> Self {
        Self { db: db.clone() }
    }

    pub async fn find_by_id(&self, id: i64) -> Result<Option<BranchModel>, AppError> {
        let model = branch_entity::Entity::find_by_id(id)
            .one(&self.db)
            .await
            ?;
        Ok(model)
    }

    pub async fn list(
        &self,
        offset: u32,
        limit: u32,
        is_active: Option<bool>,
        city: Option<&str>,
        search: Option<&str>,
    ) -> Result<PaginatedResponse<BranchResponse>, AppError> {
        let page_size = limit.max(1) as u64;
        let page_num = ((offset / limit) + 1).max(1) as u64;

        let mut select = branch_entity::Entity::find();
        if let Some(active) = is_active {
            select = select.filter(branch_entity::Column::IsActive.eq(active));
        }
        if let Some(c) = city {
            select = select.filter(branch_entity::Column::City.eq(c));
        }
        if let Some(s) = search {
            let pattern = format!("%{s}%");
            select = select.filter(
                Condition::any()
                    .add(branch_entity::Column::Name.contains(&pattern))
                    .add(branch_entity::Column::Code.contains(&pattern))
                    .add(branch_entity::Column::City.contains(&pattern)),
            );
        }

        let paginator = select
            .order_by_asc(branch_entity::Column::Name)
            .paginate(&self.db, page_size);

        let total = paginator
            .num_items()
            .await
            ? as i64;

        let models = paginator
            .fetch_page(page_num - 1)
            .await
            ?;

        let data = models.into_iter().map(BranchResponse::from_model).collect();
        let tp = total_pages(total, limit);
        Ok(PaginatedResponse {
            data,
            total,
            page: page_num as u32,
            limit,
            total_pages: tp,
        })
    }

    pub async fn create(
        &self,
        name: &str,
        code: &str,
        address: Option<&str>,
        city: Option<&str>,
        state: Option<&str>,
        pincode: Option<&str>,
        phone: Option<&str>,
        email: Option<&str>,
        timezone: &str,
    ) -> Result<BranchModel, AppError> {
        let active = branch_entity::ActiveModel {
            name: Set(name.to_string()),
            code: Set(code.to_string()),
            address: Set(address.map(|s| s.to_string())),
            city: Set(city.map(|s| s.to_string())),
            state: Set(state.map(|s| s.to_string())),
            pincode: Set(pincode.map(|s| s.to_string())),
            phone: Set(phone.map(|s| s.to_string())),
            email: Set(email.map(|s| s.to_string())),
            timezone: Set(timezone.to_string()),
            ..Default::default()
        };
        let model = active
            .insert(&self.db)
            .await
            ?;
        Ok(model)
    }

    pub async fn update(
        &self,
        id: i64,
        name: Option<&str>,
        address: Option<&str>,
        city: Option<&str>,
        state: Option<&str>,
        pincode: Option<&str>,
        phone: Option<&str>,
        email: Option<&str>,
        timezone: Option<&str>,
    ) -> Result<BranchModel, AppError> {
        let model = branch_entity::Entity::find_by_id(id)
            .one(&self.db)
            .await
            ?
            .ok_or_else(|| AppError::NotFound("Branch not found".into()))?;

        let mut active: branch_entity::ActiveModel = model.into();
        if let Some(v) = name { active.name = Set(v.to_string()); }
        if let Some(v) = address { active.address = Set(Some(v.to_string())); }
        if let Some(v) = city { active.city = Set(Some(v.to_string())); }
        if let Some(v) = state { active.state = Set(Some(v.to_string())); }
        if let Some(v) = pincode { active.pincode = Set(Some(v.to_string())); }
        if let Some(v) = phone { active.phone = Set(Some(v.to_string())); }
        if let Some(v) = email { active.email = Set(Some(v.to_string())); }
        if let Some(v) = timezone { active.timezone = Set(v.to_string()); }
        active.updated_at = Set(chrono::Utc::now().into());

        let updated = active
            .update(&self.db)
            .await
            ?;
        Ok(updated)
    }

    pub async fn deactivate(&self, id: i64) -> Result<(), AppError> {
        let model = branch_entity::Entity::find_by_id(id)
            .one(&self.db)
            .await
            ?
            .ok_or_else(|| AppError::NotFound("Branch not found".into()))?;

        let mut active: branch_entity::ActiveModel = model.into();
        active.is_active = Set(false);
        active.updated_at = Set(chrono::Utc::now().into());
        active.update(&self.db).await?;
        Ok(())
    }

    pub async fn code_exists(&self, code: &str, exclude: Option<i64>) -> Result<bool, AppError> {
        let mut select = branch_entity::Entity::find()
            .filter(branch_entity::Column::Code.eq(code));
        if let Some(id) = exclude {
            select = select.filter(branch_entity::Column::Id.ne(id));
        }
        let count = select
            .count(&self.db)
            .await
            ?;
        Ok(count > 0)
    }

    // ── Working Hours ──────────────────────────────────────

    pub async fn get_working_hours(&self, branch_id: i64) -> Result<Vec<WorkingHourModel>, AppError> {
        let models = branch_working_hour_entity::Entity::find()
            .filter(branch_working_hour_entity::Column::BranchId.eq(branch_id))
            .order_by_asc(branch_working_hour_entity::Column::DayOfWeek)
            .all(&self.db)
            .await
            ?;
        Ok(models)
    }

    pub async fn upsert_working_hours(
        &self,
        branch_id: i64,
        day_of_week: i32,
        open_time: Option<chrono::NaiveTime>,
        close_time: Option<chrono::NaiveTime>,
        is_closed: bool,
    ) -> Result<WorkingHourModel, AppError> {
        let existing = branch_working_hour_entity::Entity::find()
            .filter(branch_working_hour_entity::Column::BranchId.eq(branch_id))
            .filter(branch_working_hour_entity::Column::DayOfWeek.eq(day_of_week))
            .one(&self.db)
            .await
            ?;

        if let Some(model) = existing {
            let mut active: branch_working_hour_entity::ActiveModel = model.into();
            active.open_time = Set(open_time);
            active.close_time = Set(close_time);
            active.is_closed = Set(is_closed);
            let updated = active
                .update(&self.db)
                .await
                ?;
            Ok(updated)
        } else {
            let active = branch_working_hour_entity::ActiveModel {
                branch_id: Set(branch_id),
                day_of_week: Set(day_of_week),
                open_time: Set(open_time),
                close_time: Set(close_time),
                is_closed: Set(is_closed),
                ..Default::default()
            };
            let inserted = active
                .insert(&self.db)
                .await
                ?;
            Ok(inserted)
        }
    }

    // ── User-Branch Assignment ─────────────────────────────

    pub async fn assign_user(
        &self,
        branch_id: i64,
        user_id: i64,
        is_primary: bool,
    ) -> Result<(), AppError> {
        let existing = user_branch_entity::Entity::find()
            .filter(user_branch_entity::Column::UserId.eq(user_id))
            .filter(user_branch_entity::Column::BranchId.eq(branch_id))
            .one(&self.db)
            .await
            ?;

        if let Some(model) = existing {
            let mut active: user_branch_entity::ActiveModel = model.into();
            active.is_primary = Set(is_primary);
            active.update(&self.db).await?;
        } else {
            let active = user_branch_entity::ActiveModel {
                user_id: Set(user_id),
                branch_id: Set(branch_id),
                is_primary: Set(is_primary),
                ..Default::default()
            };
            active.insert(&self.db).await?;
        }
        Ok(())
    }

    pub async fn remove_user(&self, branch_id: i64, user_id: i64) -> Result<(), AppError> {
        user_branch_entity::Entity::delete_many()
            .filter(user_branch_entity::Column::BranchId.eq(branch_id))
            .filter(user_branch_entity::Column::UserId.eq(user_id))
            .exec(&self.db)
            .await
            ?;
        Ok(())
    }

    pub async fn list_branch_users(&self, branch_id: i64) -> Result<Vec<UserBranchModel>, AppError> {
        let models = user_branch_entity::Entity::find()
            .filter(user_branch_entity::Column::BranchId.eq(branch_id))
            .order_by_desc(user_branch_entity::Column::IsPrimary)
            .order_by_asc(user_branch_entity::Column::CreatedAt)
            .all(&self.db)
            .await
            ?;
        Ok(models)
    }

    // ── Branch Statistics ─────────────────────────────────

    pub async fn get_branch_stats(&self, branch_id: i64) -> Result<BranchStatsResponse, AppError> {
        use crate::modules::customer::model::customer_entity;
        use crate::modules::subscription::model::subscription_entity;

        let total_customers = customer_entity::Entity::find()
            .filter(customer_entity::Column::BranchId.eq(branch_id))
            .count(&self.db).await? as i64;
        let active_customers = customer_entity::Entity::find()
            .filter(customer_entity::Column::BranchId.eq(branch_id))
            .filter(customer_entity::Column::Status.eq("active"))
            .count(&self.db).await? as i64;
        let total_subscriptions = subscription_entity::Entity::find()
            .filter(subscription_entity::Column::BranchId.eq(branch_id))
            .count(&self.db).await? as i64;
        let active_subscriptions = subscription_entity::Entity::find()
            .filter(subscription_entity::Column::BranchId.eq(branch_id))
            .filter(subscription_entity::Column::Status.eq("active"))
            .count(&self.db).await? as i64;

        Ok(BranchStatsResponse {
            branch_id, total_customers, active_customers,
            total_subscriptions, active_subscriptions,
        })
    }
}
