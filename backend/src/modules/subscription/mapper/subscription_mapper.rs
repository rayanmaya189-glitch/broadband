use crate::modules::subscription::model::subscription::Subscription;
use crate::modules::subscription::response::subscription_response::SubscriptionResponse;

pub fn subscription_to_response(s: &Subscription) -> SubscriptionResponse {
    SubscriptionResponse {
        id: s.id, customer_id: s.customer_id, branch_id: s.branch_id, plan_id: s.plan_id,
        status: s.status.clone(), billing_period_months: s.billing_period_months,
        start_date: s.start_date, end_date: s.end_date, next_billing_date: s.next_billing_date,
        auto_renew: s.auto_renew, created_at: s.created_at, updated_at: s.updated_at,
    }
}
