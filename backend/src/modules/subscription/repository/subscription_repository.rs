use sea_orm::*;

use crate::common::errors::app_error::AppError;
use crate::common::utils::helpers::{total_pages, PaginatedResponse};
use crate::modules::subscription::model::subscription_entity::{self, Model as SubscriptionModel};
use crate::modules::subscription::model::subscription_history_entity::{self, Model as HistoryModel};
use crate::modules::subscription::response::subscription_response::SubscriptionResponse;

pub struct SubscriptionRepository {
    db: DatabaseConnection,
}

impl SubscriptionRepository {
    pub fn new(db: &DatabaseConnection) -> Self {
        Self { db: db.clone() }
    }

    pub async fn find_by_id(&self, id: i64) -> Result<Option<SubscriptionModel>, AppError> {
        let model = subscription_entity::Entity::find_by_id(id)
            .one(&self.db)
            .await
            ?;
        Ok(model)
    }

    pub async fn create(
        &self,
        customer_id: i64,
        branch_id: i64,
        plan_id: i64,
        billing_period_months: i32,
        start_date: chrono::NaiveDate,
        auto_renew: bool,
    ) -> Result<SubscriptionModel, AppError> {
        let active = subscription_entity::ActiveModel {
            customer_id: Set(customer_id),
            branch_id: Set(branch_id),
            plan_id: Set(plan_id),
            status: Set("active".to_string()),
            billing_period_months: Set(billing_period_months),
            start_date: Set(start_date),
            auto_renew: Set(auto_renew),
            ..Default::default()
        };
        let model = active
            .insert(&self.db)
            .await
            ?;
        Ok(model)
    }

    pub async fn update_status(&self, id: i64, status: &str) -> Result<SubscriptionModel, AppError> {
        let model = subscription_entity::Entity::find_by_id(id)
            .one(&self.db)
            .await
            ?
            .ok_or_else(|| AppError::NotFound("Subscription not found".into()))?;

        let mut active: subscription_entity::ActiveModel = model.into();
        active.status = Set(status.to_string());
        active.updated_at = Set(chrono::Utc::now().into());

        let updated = active
            .update(&self.db)
            .await
            ?;
        Ok(updated)
    }

    pub async fn cancel(&self, id: i64) -> Result<SubscriptionModel, AppError> {
        let model = subscription_entity::Entity::find_by_id(id)
            .one(&self.db)
            .await
            ?
            .ok_or_else(|| AppError::NotFound("Subscription not found".into()))?;

        let mut active: subscription_entity::ActiveModel = model.into();
        active.status = Set("cancelled".to_string());
        active.auto_renew = Set(false);
        active.end_date = Set(Some(chrono::Utc::now().date_naive()));
        active.updated_at = Set(chrono::Utc::now().into());

        let updated = active
            .update(&self.db)
            .await
            ?;
        Ok(updated)
    }

    pub async fn list(
        &self,
        page: u32,
        per_page: u32,
        status: Option<&str>,
        customer_id: Option<i64>,
        branch_id: Option<i64>,
    ) -> Result<PaginatedResponse<SubscriptionResponse>, AppError> {
        let page_size = per_page.max(1) as u64;
        let page_num = page.max(1) as u64;

        let mut select = subscription_entity::Entity::find();
        if let Some(s) = status {
            select = select.filter(subscription_entity::Column::Status.eq(s));
        }
        if let Some(cid) = customer_id {
            select = select.filter(subscription_entity::Column::CustomerId.eq(cid));
        }
        if let Some(bid) = branch_id {
            select = select.filter(subscription_entity::Column::BranchId.eq(bid));
        }

        let paginator = select
            .order_by_desc(subscription_entity::Column::CreatedAt)
            .paginate(&self.db, page_size);

        let total = paginator
            .num_items()
            .await
            ? as i64;

        let models = paginator
            .fetch_page(page_num - 1)
            .await
            ?;

        let data = models
            .into_iter()
            .map(SubscriptionResponse::from_model)
            .collect();

        let tp = total_pages(total, per_page);
        Ok(PaginatedResponse {
            data,
            total,
            page,
            limit: per_page,
            total_pages: tp,
        })
    }

    pub async fn change_plan(
        &self,
        id: i64,
        new_plan_id: i64,
    ) -> Result<SubscriptionModel, AppError> {
        let model = subscription_entity::Entity::find_by_id(id)
            .one(&self.db)
            .await
            ?
            .ok_or_else(|| AppError::NotFound("Subscription not found".into()))?;

        let mut active: subscription_entity::ActiveModel = model.into();
        active.plan_id = Set(new_plan_id);
        active.updated_at = Set(chrono::Utc::now().into());

        let updated = active
            .update(&self.db)
            .await
            ?;
        Ok(updated)
    }

    // ── Subscription History ───────────────────────────────

    pub async fn add_history(&self, subscription_id: i64, change_type: &str, old_value: Option<&str>, new_value: Option<&str>, changed_by: Option<i64>, notes: Option<&str>) -> Result<HistoryModel, AppError> {
        let active = subscription_history_entity::ActiveModel {
            subscription_id: Set(subscription_id),
            change_type: Set(change_type.to_string()),
            old_value: Set(old_value.map(|s| s.to_string())),
            new_value: Set(new_value.map(|s| s.to_string())),
            changed_by: Set(changed_by),
            notes: Set(notes.map(|s| s.to_string())),
            ..Default::default()
        };
        let model = active.insert(&self.db).await?;
        Ok(model)
    }

    pub async fn get_history(&self, subscription_id: i64) -> Result<Vec<HistoryModel>, AppError> {
        let models = subscription_history_entity::Entity::find()
            .filter(subscription_history_entity::Column::SubscriptionId.eq(subscription_id))
            .order_by_desc(subscription_history_entity::Column::CreatedAt)
            .all(&self.db).await?;
        Ok(models)
    }

    // ── Plan Price & Remaining Days ─────────────────────────

    pub async fn get_plan_price(&self, plan_id: i64) -> Result<rust_decimal::Decimal, AppError> {
        let result = crate::modules::plan::model::plan_entity::Entity::find_by_id(plan_id)
            .one(&self.db).await?;
        Ok(result.map(|p| p.price_monthly).unwrap_or(rust_decimal::Decimal::ZERO))
    }

    pub async fn get_remaining_days(&self, subscription_id: i64) -> Result<i32, AppError> {
        let model = subscription_entity::Entity::find_by_id(subscription_id)
            .one(&self.db).await?
            .ok_or_else(|| AppError::NotFound("Subscription not found".into()))?;
        let now = chrono::Utc::now().date_naive();
        let end = model.end_date.unwrap_or_else(|| {
            model.start_date + chrono::Duration::days((model.billing_period_months * 30) as i64)
        });
        let remaining = (end - now).num_days().max(0) as i32;
        Ok(remaining)
    }
}
