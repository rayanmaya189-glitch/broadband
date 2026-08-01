use crate::infrastructure::messaging::outbox;
use crate::modules::subscription::domain::entities::{
    Subscription, SubscriptionActiveModel, SubscriptionColumn,
};
use crate::shared::errors::AppError;
use sea_orm::{
    ActiveModelTrait, ColumnTrait, DatabaseConnection, EntityTrait, PaginatorTrait, QueryFilter,
    Set,
};
use tracing::info;

pub struct SubscriptionService;

impl SubscriptionService {
    pub async fn list_subscriptions(
        db: &DatabaseConnection,
        branch_id: Option<i64>,
        _page: u64,
        _limit: u64,
    ) -> Result<
        (
            Vec<crate::modules::subscription::domain::entities::subscription::Model>,
            u64,
        ),
        AppError,
    > {
        let mut query = Subscription::find();
        if let Some(bid) = branch_id {
            query = query.filter(SubscriptionColumn::BranchId.eq(bid));
        }
        let total = query.clone().count(db).await?;
        let items = query.all(db).await?;
        Ok((items, total))
    }

    pub async fn get_subscription(
        db: &DatabaseConnection,
        id: i64,
    ) -> Result<crate::modules::subscription::domain::entities::subscription::Model, AppError> {
        Subscription::find_by_id(id)
            .one(db)
            .await?
            .ok_or_else(|| AppError::NotFound(format!("Subscription {} not found", id)))
    }

    pub async fn create_subscription(
        db: &DatabaseConnection,
        customer_id: i64,
        branch_id: i64,
        plan_id: i64,
        billing_period_months: i32,
    ) -> Result<crate::modules::subscription::domain::entities::subscription::Model, AppError> {
        let now = chrono::Utc::now();
        let start = now.date_naive();
        let next_billing = start + chrono::Duration::days((billing_period_months as i64) * 30);
        let new_sub = SubscriptionActiveModel {
            customer_id: Set(customer_id),
            branch_id: Set(branch_id),
            plan_id: Set(plan_id),
            status: Set("active".to_string()),
            billing_period_months: Set(billing_period_months),
            start_date: Set(start),
            next_billing_date: Set(Some(next_billing)),
            auto_renew: Set(true),
            review_status: Set(Some("pending".to_string())),
            created_at: Set(now),
            updated_at: Set(now),
            ..Default::default()
        };
        let new_sub_model = new_sub.insert(db).await?;
        Ok(new_sub_model)
    }

    /// Reactivate a suspended or cancelled subscription
    pub async fn reactivate_subscription(
        db: &DatabaseConnection,
        id: i64,
    ) -> Result<crate::modules::subscription::domain::entities::subscription::Model, AppError> {
        let sub = Self::get_subscription(db, id).await?;

        // Only suspended or cancelled subscriptions can be reactivated
        if sub.status != "suspended" && sub.status != "cancelled" {
            return Err(AppError::Validation(format!(
                "Cannot reactivate subscription in '{}' status; must be 'suspended' or 'cancelled'",
                sub.status
            )));
        }

        let now = chrono::Utc::now();
        let next_billing =
            now.date_naive() + chrono::Duration::days((sub.billing_period_months as i64) * 30);

        let mut active: SubscriptionActiveModel = sub.into();
        active.status = Set("active".to_string());
        active.next_billing_date = Set(Some(next_billing));
        active.updated_at = Set(now);
        Ok(active.update(db).await?)
    }

    /// Upgrade a subscription to a new plan with mid-cycle proration
    pub async fn upgrade_subscription(
        db: &DatabaseConnection,
        id: i64,
        new_plan_id: i64,
        new_billing_period_months: Option<i32>,
    ) -> Result<(
        crate::modules::subscription::domain::entities::subscription::Model,
        Option<crate::shared::primitives::ProRataAdjustment>,
    ), AppError> {
        let sub = Self::get_subscription(db, id).await?;

        if sub.status != "active" {
            return Err(AppError::Validation(format!(
                "Cannot upgrade subscription in '{}' status; must be 'active'",
                sub.status
            )));
        }

        if sub.plan_id == new_plan_id {
            return Err(AppError::Validation(
                "New plan must be different from current plan".into(),
            ));
        }

        // Fetch old and new plan prices for proration
        use crate::modules::plans::domain::entities::{PlanPricing, PlanPricingColumn};
        let old_pricing = PlanPricing::find()
            .filter(PlanPricingColumn::PlanId.eq(sub.plan_id))
            .filter(PlanPricingColumn::BillingPeriodMonths.eq(sub.billing_period_months))
            .filter(PlanPricingColumn::IsActive.eq(true))
            .one(db)
            .await?;
        let new_pricing = PlanPricing::find()
            .filter(PlanPricingColumn::PlanId.eq(new_plan_id))
            .filter(PlanPricingColumn::BillingPeriodMonths.eq(
                new_billing_period_months.unwrap_or(sub.billing_period_months)
            ))
            .filter(PlanPricingColumn::IsActive.eq(true))
            .one(db)
            .await?;

        let proration = match (old_pricing, new_pricing) {
            (Some(old_p), Some(new_p)) => {
                let today = chrono::Utc::now().date_naive();
                let billing_period_days = (sub.billing_period_months as i64 * 30) as i32;
                let days_used = (today - sub.start_date).num_days() as i32;
                let days_used = days_used.max(0).min(billing_period_days);

                let adjustment = crate::shared::primitives::calculate_pro_rata(
                    old_p.price,
                    new_p.price,
                    billing_period_days,
                    days_used,
                );
                Some(adjustment)
            }
            _ => None,
        };

        let now = chrono::Utc::now();
        let mut active: SubscriptionActiveModel = sub.into();
        active.plan_id = Set(new_plan_id);
        if let Some(period) = new_billing_period_months {
            active.billing_period_months = Set(period);
        }
        active.updated_at = Set(now);
        let updated = active.update(db).await?;
        Ok((updated, proration))
    }

    /// Downgrade a subscription to a lower plan
    /// Takes effect at next billing cycle (soft downgrade)
    pub async fn downgrade_subscription(
        db: &DatabaseConnection,
        id: i64,
        new_plan_id: i64,
        new_billing_period_months: Option<i32>,
    ) -> Result<crate::modules::subscription::domain::entities::subscription::Model, AppError> {
        let sub = Self::get_subscription(db, id).await?;

        if sub.status != "active" {
            return Err(AppError::Validation(format!(
                "Cannot downgrade subscription in '{}' status; must be 'active'",
                sub.status
            )));
        }

        if sub.plan_id == new_plan_id {
            return Err(AppError::Validation(
                "New plan must be different from current plan".into(),
            ));
        }

        let now = chrono::Utc::now();
        let mut active: SubscriptionActiveModel = sub.into();
        active.plan_id = Set(new_plan_id);
        active.review_status = Set(Some("pending_downgrade".to_string()));
        if let Some(period) = new_billing_period_months {
            active.billing_period_months = Set(period);
        }
        active.updated_at = Set(now);
        Ok(active.update(db).await?)
    }

    pub async fn cancel_subscription(
        db: &DatabaseConnection,
        id: i64,
        _reason: &str,
    ) -> Result<crate::modules::subscription::domain::entities::subscription::Model, AppError> {
        let sub = Self::get_subscription(db, id).await?;
        let customer_id = sub.customer_id;
        let sub_id = sub.id;
        let mut active: SubscriptionActiveModel = sub.into();
        active.status = Set("cancelled".to_string());
        active.updated_at = Set(chrono::Utc::now());
        let updated = active.update(db).await?;

        // Cleanup: release MAC bindings associated with this subscription
        (async {
            use crate::modules::network::domain::entities::{
                MacBinding, MacBindingActiveModel, MacBindingColumn,
            };
            let bindings = MacBinding::find()
                .filter(MacBindingColumn::SubscriptionId.eq(sub_id))
                .all(db)
                .await?;
            for b in bindings {
                let mut m: MacBindingActiveModel = b.into();
                m.is_active = Set(false);
                m.updated_at = Set(chrono::Utc::now());
                m.update(db).await?;
            }
            Ok::<_, sea_orm::DbErr>(())
        })
        .await
        .ok();

        outbox::insert_outbox_event(
            db, "subscription.cancelled", "subscription", sub_id,
            serde_json::json!({"customer_id": customer_id, "status": "cancelled"}),
            None, None, None,
        ).await.ok();
        Ok(updated)
    }

    pub async fn suspend_subscription(
        db: &DatabaseConnection,
        id: i64,
        _reason: &str,
    ) -> Result<crate::modules::subscription::domain::entities::subscription::Model, AppError> {
        let sub = Self::get_subscription(db, id).await?;
        let customer_id = sub.customer_id;
        let sub_id = sub.id;
        let mut active: SubscriptionActiveModel = sub.into();
        active.status = Set("suspended".to_string());
        active.updated_at = Set(chrono::Utc::now());
        let updated = active.update(db).await?;
        outbox::insert_outbox_event(
            db, "subscription.suspended", "subscription", sub_id,
            serde_json::json!({"customer_id": customer_id, "status": "suspended"}),
            None, None, None,
        ).await.ok();
        Ok(updated)
    }

    pub async fn update_subscription(
        db: &DatabaseConnection,
        id: i64,
        billing_period_months: Option<i32>,
        auto_renew: Option<bool>,
    ) -> Result<crate::modules::subscription::domain::entities::subscription::Model, AppError> {
        let sub = Self::get_subscription(db, id).await?;
        let mut active: SubscriptionActiveModel = sub.into();
        if let Some(period) = billing_period_months {
            active.billing_period_months = Set(period);
        }
        if let Some(renew) = auto_renew {
            active.auto_renew = Set(renew);
        }
        active.updated_at = Set(chrono::Utc::now());
        Ok(active.update(db).await?)
    }

    pub async fn renew_subscription(
        db: &DatabaseConnection,
        id: i64,
    ) -> Result<crate::modules::subscription::domain::entities::subscription::Model, AppError> {
        use crate::modules::plans::domain::entities::{PlanPricing, PlanPricingColumn};
        use sea_orm::TransactionTrait;

        let txn = db.begin().await?;
        let sub = Subscription::find_by_id(id)
            .one(&txn)
            .await?
            .ok_or_else(|| AppError::NotFound(format!("Subscription {} not found", id)))?;

        if sub.status == "cancelled" {
            return Err(AppError::Validation(
                "Cannot renew a cancelled subscription; use reactivate instead".into(),
            ));
        }

        let now = chrono::Utc::now();
        let period_start = sub.next_billing_date.unwrap_or_else(|| now.date_naive());
        let next_billing =
            now.date_naive() + chrono::Duration::days((sub.billing_period_months as i64) * 30);

        let mut active: SubscriptionActiveModel = sub.clone().into();
        active.status = Set("active".to_string());
        active.next_billing_date = Set(Some(next_billing));
        active.updated_at = Set(now);
        let updated = active.update(&txn).await?;

        // Generate the next billing-cycle invoice from the active plan pricing
        // (same transaction as the renewal so the two never drift apart).
        let mut invoice_id: Option<i64> = None;
        let pricing = PlanPricing::find()
            .filter(PlanPricingColumn::PlanId.eq(sub.plan_id))
            .filter(PlanPricingColumn::BillingPeriodMonths.eq(sub.billing_period_months))
            .filter(PlanPricingColumn::IsActive.eq(true))
            .one(&txn)
            .await?;
        if let Some(p) = pricing {
            let inv = crate::modules::billing::application::services::BillingService::create_invoice(
                &txn,
                sub.customer_id,
                sub.branch_id,
                sub.id,
                period_start,
                next_billing,
                p.price,
            )
            .await?;
            invoice_id = Some(inv.id);
        }

        // Publish the renewal event once, atomically with the state change above.
        outbox::insert_outbox_event(
            &txn,
            "subscription.renewed",
            "subscription",
            updated.id,
            serde_json::json!({
                "subscription_id": updated.id,
                "customer_id": updated.customer_id,
                "action": "renewed",
                "next_billing_date": updated.next_billing_date,
                "invoice_id": invoice_id,
            }),
            None,
            None,
            Some(updated.branch_id),
        )
        .await?;

        txn.commit().await?;

        // When auto-renew is on and the customer has wallet balance, try to
        // settle the new invoice immediately. Best effort — if the balance is
        // insufficient the invoice simply stays pending for the dunning worker.
        if updated.auto_renew {
            if let Some(inv_id) = invoice_id {
                if let Err(e) =
                    crate::modules::payment::application::services::PaymentService::pay_from_wallet(
                        db,
                        inv_id,
                        updated.customer_id,
                        None,
                    )
                    .await
                {
                    info!(
                        subscription_id = updated.id,
                        invoice_id = inv_id,
                        error = %e,
                        "Auto-renew wallet payment not applied (balance may be insufficient)"
                    );
                }
            }
        }

        Ok(updated)
    }
}
