//! Customer event types.
//!
//! These events are published when customer-related actions occur
//! and are consumed by other modules (billing, network, notification).

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

/// Published when a new customer is created.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CustomerCreatedEvent {
    pub customer_id: i64,
    pub customer_code: String,
    pub first_name: String,
    pub last_name: Option<String>,
    pub email: Option<String>,
    pub phone: String,
    pub branch_id: i64,
    pub created_by: Option<i64>,
    pub timestamp: DateTime<Utc>,
}

/// Published when a customer is updated.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CustomerUpdatedEvent {
    pub customer_id: i64,
    pub changed_fields: Vec<String>,
    pub updated_by: Option<i64>,
    pub timestamp: DateTime<Utc>,
}

/// Published when a customer is suspended.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CustomerSuspendedEvent {
    pub customer_id: i64,
    pub reason: Option<String>,
    pub suspended_by: Option<i64>,
    pub timestamp: DateTime<Utc>,
}

/// Published when a customer is deactivated.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CustomerDeactivatedEvent {
    pub customer_id: i64,
    pub reason: Option<String>,
    pub deactivated_by: Option<i64>,
    pub timestamp: DateTime<Utc>,
}

/// Published when a customer's KYC is verified.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CustomerKycVerifiedEvent {
    pub customer_id: i64,
    pub verified_by: Option<i64>,
    pub timestamp: DateTime<Utc>,
}
