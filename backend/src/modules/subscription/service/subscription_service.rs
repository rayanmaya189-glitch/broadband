use sea_orm::DatabaseConnection;

use crate::common::errors::app_error::AppError;
use crate::common::utils::helpers::PaginatedResponse;
use crate::modules::subscription::repository::subscription_repository::SubscriptionRepository;
use crate::modules::subscription::request::subscription_request::*;
use crate::modules::subscription::response::subscription_response::*;

pub struct SubscriptionService {
    repo: SubscriptionRepository,
}

impl SubscriptionService {
    pub fn new(db: &DatabaseConnection) -> Self {
        Self {
            repo: SubscriptionRepository::new(db),
        }
    }

    pub async fn list_subscriptions(
        &self,
        query: &ListSubscriptionsQuery,
    ) -> Result<PaginatedResponse<SubscriptionResponse>, AppError> {
        let page = query.pagination.page.max(1);
        let per_page = query.pagination.limit.max(1);
        self.repo
            .list(
                page,
                per_page,
                query.status.as_deref(),
                query.customer_id,
                query.branch_id,
            )
            .await
    }

    pub async fn get_subscription(&self, id: i64) -> Result<SubscriptionResponse, AppError> {
        let model = self
            .repo
            .find_by_id(id)
            .await?
            .ok_or_else(|| AppError::NotFound("Subscription not found".into()))?;
        Ok(SubscriptionResponse::from_model(model))
    }

    pub async fn create_subscription(
        &self,
        req: &CreateSubscriptionRequest,
    ) -> Result<SubscriptionResponse, AppError> {
        let months = req.billing_period_months.unwrap_or(1);
        if !(1..=12).contains(&months) {
            return Err(AppError::Validation(
                "Billing period must be 1-12 months".into(),
            ));
        }
        let model = self
            .repo
            .create(
                req.customer_id,
                req.branch_id,
                req.plan_id,
                months,
                req.start_date,
                req.auto_renew.unwrap_or(true),
            )
            .await?;
        Ok(SubscriptionResponse::from_model(model))
    }

    pub async fn suspend_subscription(
        &self,
        id: i64,
        _reason: Option<&str>,
    ) -> Result<SubscriptionResponse, AppError> {
        let model = self
            .repo
            .find_by_id(id)
            .await?
            .ok_or_else(|| AppError::NotFound("Subscription not found".into()))?;

        if model.status != "active" {
            return Err(AppError::Validation(
                "Only active subscriptions can be suspended".into(),
            ));
        }

        let model = self.repo.update_status(id, "suspended").await?;
        Ok(SubscriptionResponse::from_model(model))
    }

    pub async fn reactivate_subscription(
        &self,
        id: i64,
    ) -> Result<SubscriptionResponse, AppError> {
        let model = self
            .repo
            .find_by_id(id)
            .await?
            .ok_or_else(|| AppError::NotFound("Subscription not found".into()))?;

        if model.status != "suspended" {
            return Err(AppError::Validation(
                "Only suspended subscriptions can be reactivated".into(),
            ));
        }

        let model = self.repo.update_status(id, "active").await?;
        Ok(SubscriptionResponse::from_model(model))
    }

    pub async fn cancel_subscription(
        &self,
        id: i64,
        _reason: Option<&str>,
    ) -> Result<MessageResponse, AppError> {
        let model = self
            .repo
            .find_by_id(id)
            .await?
            .ok_or_else(|| AppError::NotFound("Subscription not found".into()))?;

        if model.status == "cancelled" || model.status == "expired" {
            return Err(AppError::Validation(
                "Subscription is already cancelled or expired".into(),
            ));
        }

        self.repo.cancel(id).await?;
        Ok(MessageResponse {
            message: "Subscription cancelled successfully".into(),
        })
    }

    pub async fn upgrade_subscription(
        &self,
        id: i64,
        req: &UpgradeDowngradeRequest,
    ) -> Result<UpgradeDowngradeResponse, AppError> {
        let model = self
            .repo
            .find_by_id(id)
            .await?
            .ok_or_else(|| AppError::NotFound("Subscription not found".into()))?;

        if model.status != "active" {
            return Err(AppError::Validation(
                "Only active subscriptions can be upgraded".into(),
            ));
        }

        let new_model = self.repo.change_plan(id, req.new_plan_id).await?;
        let resp = SubscriptionResponse::from_model(new_model);

        Ok(UpgradeDowngradeResponse {
            subscription: resp,
            pro_rata: ProRataAdjustment {
                old_plan_id: model.plan_id,
                new_plan_id: req.new_plan_id,
                old_plan_price: rust_decimal::Decimal::ZERO,
                new_plan_price: rust_decimal::Decimal::ZERO,
                old_plan_credit: rust_decimal::Decimal::ZERO,
                new_plan_charge: rust_decimal::Decimal::ZERO,
                adjustment: rust_decimal::Decimal::ZERO,
                remaining_days: 0,
                billing_period_days: model.billing_period_months * 30,
            },
            message: "Upgrade complete".into(),
        })
    }

    pub async fn downgrade_subscription(
        &self,
        id: i64,
        req: &UpgradeDowngradeRequest,
    ) -> Result<UpgradeDowngradeResponse, AppError> {
        let model = self
            .repo
            .find_by_id(id)
            .await?
            .ok_or_else(|| AppError::NotFound("Subscription not found".into()))?;

        if model.status != "active" {
            return Err(AppError::Validation(
                "Only active subscriptions can be downgraded".into(),
            ));
        }

        let new_model = self.repo.change_plan(id, req.new_plan_id).await?;
        let resp = SubscriptionResponse::from_model(new_model);

        Ok(UpgradeDowngradeResponse {
            subscription: resp,
            pro_rata: ProRataAdjustment {
                old_plan_id: model.plan_id,
                new_plan_id: req.new_plan_id,
                old_plan_price: rust_decimal::Decimal::ZERO,
                new_plan_price: rust_decimal::Decimal::ZERO,
                old_plan_credit: rust_decimal::Decimal::ZERO,
                new_plan_charge: rust_decimal::Decimal::ZERO,
                adjustment: rust_decimal::Decimal::ZERO,
                remaining_days: 0,
                billing_period_days: model.billing_period_months * 30,
            },
            message: "Downgrade applied".into(),
        })
    }

    pub async fn get_history(
        &self,
        id: i64,
        _query: &SubscriptionHistoryQuery,
    ) -> Result<Vec<SubscriptionHistoryEntry>, AppError> {
        self.repo
            .find_by_id(id)
            .await?
            .ok_or_else(|| AppError::NotFound("Subscription not found".into()))?;
        // History queries still use sqlx via background jobs
        // For now return empty - will be converted when subscription_history table is added
        Ok(vec![])
    }
}
