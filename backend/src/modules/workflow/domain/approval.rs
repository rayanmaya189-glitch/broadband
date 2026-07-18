use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::fmt;

use crate::shared::errors::AppError;

/// Approval workflow types
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum ApprovalWorkflowType {
    PlanCreation,
    PlanPricingChange,
    CustomerSuspension,
    LargePayment,
    NetworkChange,
    DeviceDecommission,
    Custom(String),
}

impl fmt::Display for ApprovalWorkflowType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::PlanCreation => write!(f, "plan_creation"),
            Self::PlanPricingChange => write!(f, "plan_pricing_change"),
            Self::CustomerSuspension => write!(f, "customer_suspension"),
            Self::LargePayment => write!(f, "large_payment"),
            Self::NetworkChange => write!(f, "network_change"),
            Self::DeviceDecommission => write!(f, "device_decommission"),
            Self::Custom(name) => write!(f, "custom_{}", name),
        }
    }
}

impl ApprovalWorkflowType {
    pub fn from_str(s: &str) -> Self {
        match s {
            "plan_creation" => Self::PlanCreation,
            "plan_pricing_change" => Self::PlanPricingChange,
            "customer_suspension" => Self::CustomerSuspension,
            "large_payment" => Self::LargePayment,
            "network_change" => Self::NetworkChange,
            "device_decommission" => Self::DeviceDecommission,
            _ => Self::Custom(s.to_string()),
        }
    }
}

impl ApprovalStatus {
    pub fn from_str(s: &str) -> Self {
        match s {
            "pending" => Self::Pending,
            "approved" => Self::Approved,
            "rejected" => Self::Rejected,
            "cancelled" => Self::Cancelled,
            "expired" => Self::Expired,
            _ => Self::Pending,
        }
    }
}

/// Approval request status
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum ApprovalStatus {
    Pending,
    Approved,
    Rejected,
    Cancelled,
    Expired,
}

impl fmt::Display for ApprovalStatus {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Pending => write!(f, "pending"),
            Self::Approved => write!(f, "approved"),
            Self::Rejected => write!(f, "rejected"),
            Self::Cancelled => write!(f, "cancelled"),
            Self::Expired => write!(f, "expired"),
        }
    }
}

/// Approval request aggregate root
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ApprovalRequest {
    pub id: i64,
    pub workflow_type: ApprovalWorkflowType,
    pub resource_type: String,
    pub resource_id: i64,
    pub requested_by: i64,
    pub branch_id: Option<i64>,
    pub status: ApprovalStatus,
    pub payload: serde_json::Value,
    pub reason: Option<String>,
    pub reviewer_id: Option<i64>,
    pub reviewer_comment: Option<String>,
    pub requested_at: DateTime<Utc>,
    pub reviewed_at: Option<DateTime<Utc>>,
    pub expires_at: Option<DateTime<Utc>>,
}

impl ApprovalRequest {
    /// Create a new approval request
    pub fn new(
        workflow_type: ApprovalWorkflowType,
        resource_type: String,
        resource_id: i64,
        requested_by: i64,
        branch_id: Option<i64>,
        payload: serde_json::Value,
    ) -> Self {
        let now = Utc::now();
        Self {
            id: 0,
            workflow_type,
            resource_type,
            resource_id,
            requested_by,
            branch_id,
            status: ApprovalStatus::Pending,
            payload,
            reason: None,
            reviewer_id: None,
            reviewer_comment: None,
            requested_at: now,
            reviewed_at: None,
            expires_at: Some(now + chrono::Duration::hours(72)), // 72 hour expiry
        }
    }

    /// Check if this approval request can be reviewed
    pub fn can_review(&self) -> bool {
        self.status == ApprovalStatus::Pending
            && self.expires_at.map(|e| e > Utc::now()).unwrap_or(true)
    }

    /// Approve the request
    pub fn approve(&mut self, reviewer_id: i64, comment: Option<String>) -> Result<(), AppError> {
        if !self.can_review() {
            return Err(AppError::BadRequest(
                "Approval request cannot be reviewed".to_string(),
            ));
        }

        self.status = ApprovalStatus::Approved;
        self.reviewer_id = Some(reviewer_id);
        self.reviewer_comment = comment;
        self.reviewed_at = Some(Utc::now());

        Ok(())
    }

    /// Reject the request
    pub fn reject(&mut self, reviewer_id: i64, comment: String) -> Result<(), AppError> {
        if !self.can_review() {
            return Err(AppError::BadRequest(
                "Approval request cannot be reviewed".to_string(),
            ));
        }

        self.status = ApprovalStatus::Rejected;
        self.reviewer_id = Some(reviewer_id);
        self.reviewer_comment = Some(comment);
        self.reviewed_at = Some(Utc::now());

        Ok(())
    }

    /// Cancel the request (by requester)
    pub fn cancel(&mut self, user_id: i64) -> Result<(), AppError> {
        if self.requested_by != user_id {
            return Err(AppError::Forbidden(
                "Only the requester can cancel".to_string(),
            ));
        }

        if self.status != ApprovalStatus::Pending {
            return Err(AppError::BadRequest(
                "Only pending requests can be cancelled".to_string(),
            ));
        }

        self.status = ApprovalStatus::Cancelled;
        self.reviewed_at = Some(Utc::now());

        Ok(())
    }

    /// Check if expired and update status
    pub fn check_expiry(&mut self) -> bool {
        if self.status == ApprovalStatus::Pending {
            if let Some(expires_at) = self.expires_at {
                if expires_at <= Utc::now() {
                    self.status = ApprovalStatus::Expired;
                    self.reviewed_at = Some(Utc::now());
                    return true;
                }
            }
        }
        false
    }
}

/// Approval workflow configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ApprovalWorkflowConfig {
    pub workflow_type: ApprovalWorkflowType,
    pub required_approvers: u32,
    pub expiry_hours: u32,
    pub allowed_reviewer_roles: Vec<String>,
    pub auto_approve_conditions: Option<serde_json::Value>,
}

impl ApprovalWorkflowConfig {
    pub fn default_configs() -> Vec<Self> {
        vec![
            Self {
                workflow_type: ApprovalWorkflowType::PlanCreation,
                required_approvers: 1,
                expiry_hours: 72,
                allowed_reviewer_roles: vec!["super_admin".to_string(), "isp_owner".to_string()],
                auto_approve_conditions: None,
            },
            Self {
                workflow_type: ApprovalWorkflowType::PlanPricingChange,
                required_approvers: 2,
                expiry_hours: 48,
                allowed_reviewer_roles: vec![
                    "super_admin".to_string(),
                    "isp_owner".to_string(),
                    "finance_manager".to_string(),
                ],
                auto_approve_conditions: None,
            },
            Self {
                workflow_type: ApprovalWorkflowType::CustomerSuspension,
                required_approvers: 1,
                expiry_hours: 24,
                allowed_reviewer_roles: vec![
                    "super_admin".to_string(),
                    "isp_owner".to_string(),
                    "noc_engineer".to_string(),
                ],
                auto_approve_conditions: None,
            },
            Self {
                workflow_type: ApprovalWorkflowType::LargePayment,
                required_approvers: 1,
                expiry_hours: 48,
                allowed_reviewer_roles: vec![
                    "super_admin".to_string(),
                    "isp_owner".to_string(),
                    "finance_manager".to_string(),
                ],
                auto_approve_conditions: None,
            },
            Self {
                workflow_type: ApprovalWorkflowType::NetworkChange,
                required_approvers: 1,
                expiry_hours: 24,
                allowed_reviewer_roles: vec![
                    "super_admin".to_string(),
                    "isp_owner".to_string(),
                    "network_admin".to_string(),
                ],
                auto_approve_conditions: None,
            },
            Self {
                workflow_type: ApprovalWorkflowType::DeviceDecommission,
                required_approvers: 1,
                expiry_hours: 48,
                allowed_reviewer_roles: vec![
                    "super_admin".to_string(),
                    "isp_owner".to_string(),
                    "network_admin".to_string(),
                ],
                auto_approve_conditions: None,
            },
        ]
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_approval_request_new() {
        let request = ApprovalRequest::new(
            ApprovalWorkflowType::PlanCreation,
            "plan".to_string(),
            1,
            100,
            Some(1),
            serde_json::json!({"name": "Test Plan"}),
        );

        assert_eq!(request.status, ApprovalStatus::Pending);
        assert_eq!(request.workflow_type, ApprovalWorkflowType::PlanCreation);
        assert!(request.can_review());
    }

    #[test]
    fn test_approval_request_approve() {
        let mut request = ApprovalRequest::new(
            ApprovalWorkflowType::PlanCreation,
            "plan".to_string(),
            1,
            100,
            Some(1),
            serde_json::json!({}),
        );

        assert!(request.approve(200, Some("Looks good".to_string())).is_ok());
        assert_eq!(request.status, ApprovalStatus::Approved);
        assert_eq!(request.reviewer_id, Some(200));
        assert!(!request.can_review());
    }

    #[test]
    fn test_approval_request_reject() {
        let mut request = ApprovalRequest::new(
            ApprovalWorkflowType::PlanCreation,
            "plan".to_string(),
            1,
            100,
            Some(1),
            serde_json::json!({}),
        );

        assert!(request.reject(200, "Not ready".to_string()).is_ok());
        assert_eq!(request.status, ApprovalStatus::Rejected);
    }

    #[test]
    fn test_approval_request_cancel_by_non_requester_fails() {
        let mut request = ApprovalRequest::new(
            ApprovalWorkflowType::PlanCreation,
            "plan".to_string(),
            1,
            100,
            Some(1),
            serde_json::json!({}),
        );

        assert!(request.cancel(999).is_err());
    }

    #[test]
    fn test_approval_request_cancel_by_requester() {
        let mut request = ApprovalRequest::new(
            ApprovalWorkflowType::PlanCreation,
            "plan".to_string(),
            1,
            100,
            Some(1),
            serde_json::json!({}),
        );

        assert!(request.cancel(100).is_ok());
        assert_eq!(request.status, ApprovalStatus::Cancelled);
    }

    #[test]
    fn test_cannot_approve_already_approved() {
        let mut request = ApprovalRequest::new(
            ApprovalWorkflowType::PlanCreation,
            "plan".to_string(),
            1,
            100,
            Some(1),
            serde_json::json!({}),
        );

        request.approve(200, None).unwrap();
        assert!(request.approve(300, None).is_err());
    }
}
