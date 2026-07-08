use sqlx::PgPool;

use crate::common::errors::app_error::AppError;
use crate::common::utils::helpers::PaginatedResponse;
use crate::modules::subscription::mapper::subscription_mapper::subscription_to_response;
use crate::modules::subscription::repository::subscription_repository::SubscriptionRepository;
use crate::modules::subscription::request::subscription_request::*;
use crate::modules::subscription::response::subscription_response::*;

pub struct SubscriptionService<'a> {
    repo: SubscriptionRepository<'a>,
}

impl<'a> SubscriptionService<'a> {
    pub fn new(pool: &'a PgPool) -> Self { Self { repo: SubscriptionRepository::new(pool) } }

    pub async fn list_subscriptions(&self, query: &ListSubscriptionsQuery) -> Result<PaginatedResponse<SubscriptionResponse>, AppError> {
        let offset = query.pagination.offset();
        let limit = query.pagination.limit_i64() as u32;
        self.repo.list(offset, limit, query.status.as_deref(), query.customer_id, query.branch_id).await
    }

    pub async fn get_subscription(&self, id: i64) -> Result<SubscriptionDetailResponse, AppError> {
        let s = self.repo.find_by_id(id).await?.ok_or_else(|| AppError::NotFound("Subscription not found".into()))?;
        Ok(subscription_to_response(&s))
    }

    pub async fn create_subscription(&self, req: &CreateSubscriptionRequest) -> Result<SubscriptionDetailResponse, AppError> {
        let months = req.billing_period_months.unwrap_or(1);
        if !(1..=12).contains(&months) {
            return Err(AppError::Validation("Billing period must be 1-12 months".into()));
        }
        let s = self.repo.create(req.customer_id, req.branch_id, req.plan_id, months, req.start_date, req.auto_renew.unwrap_or(true)).await?;
        Ok(subscription_to_response(&s))
    }

    pub async fn suspend_subscription(&self, id: i64) -> Result<SubscriptionDetailResponse, AppError> {
        let s = self.repo.find_by_id(id).await?.ok_or_else(|| AppError::NotFound("Subscription not found".into()))?;
        if s.status != "active" { return Err(AppError::Validation("Only active subscriptions can be suspended".into())); }
        let s = self.repo.update_status(id, "suspended").await?;
        Ok(subscription_to_response(&s))
    }

    pub async fn reactivate_subscription(&self, id: i64) -> Result<SubscriptionDetailResponse, AppError> {
        let s = self.repo.find_by_id(id).await?.ok_or_else(|| AppError::NotFound("Subscription not found".into()))?;
        if s.status != "suspended" { return Err(AppError::Validation("Only suspended subscriptions can be reactivated".into())); }
        let s = self.repo.update_status(id, "active").await?;
        Ok(subscription_to_response(&s))
    }

    pub async fn cancel_subscription(&self, id: i64) -> Result<MessageResponse, AppError> {
        let s = self.repo.find_by_id(id).await?.ok_or_else(|| AppError::NotFound("Subscription not found".into()))?;
        if s.status == "cancelled" || s.status == "expired" {
            return Err(AppError::Validation("Subscription is already cancelled or expired".into()));
        }
        self.repo.cancel(id).await?;
        Ok(MessageResponse { message: "Subscription cancelled successfully".into() })
    }
}
