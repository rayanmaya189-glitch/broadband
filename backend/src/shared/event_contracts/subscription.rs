use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SubscriptionCreatedV1 {
    pub subscription_id: i64,
    pub customer_id: i64,
    pub plan_id: i64,
    pub billing_period_months: i32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SubscriptionRenewedV1 {
    pub subscription_id: i64,
    pub customer_id: i64,
    pub next_billing_date: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SubscriptionSuspendedV1 {
    pub subscription_id: i64,
    pub customer_id: i64,
    pub reason: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SubscriptionReactivatedV1 {
    pub subscription_id: i64,
    pub customer_id: i64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SubscriptionCancelledV1 {
    pub subscription_id: i64,
    pub customer_id: i64,
    pub reason: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SubscriptionUpgradedV1 {
    pub subscription_id: i64,
    pub old_plan_id: i64,
    pub new_plan_id: i64,
    pub pro_rata_adjustment: Decimal,
}
