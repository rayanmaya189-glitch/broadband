use crate::modules::installation::domain::value_objects::{InstallationId, InstallationStatus};

/// InstallationOrder aggregate root - represents a fiber installation order
#[derive(Debug, Clone)]
pub struct InstallationOrder {
    pub id: InstallationId,
    pub customer_id: i64,
    pub branch_id: i64,
    pub subscription_id: Option<i64>,
    pub assigned_technician_id: Option<i64>,
    pub status: InstallationStatus,
    pub scheduled_date: Option<chrono::NaiveDate>,
    pub completed_at: Option<chrono::DateTime<chrono::Utc>>,
    pub installation_type: String,
    pub notes: Option<String>,
}

/// Domain errors for InstallationOrder aggregate
#[derive(Debug, Clone, PartialEq)]
pub enum InstallationDomainError {
    InstallationNotFound(i64),
    AlreadyCompleted,
    AlreadyCancelled,
    InvalidStatusTransition,
    CannotCompleteWithoutSchedule,
}

impl std::fmt::Display for InstallationDomainError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::InstallationNotFound(id) => write!(f, "Installation {} not found", id),
            Self::AlreadyCompleted => write!(f, "Installation is already completed"),
            Self::AlreadyCancelled => write!(f, "Installation is already cancelled"),
            Self::InvalidStatusTransition => write!(f, "Invalid installation status transition"),
            Self::CannotCompleteWithoutSchedule => write!(f, "Cannot complete an unscheduled installation"),
        }
    }
}

impl std::error::Error for InstallationDomainError {}

impl InstallationOrder {
    pub fn new(customer_id: i64, branch_id: i64) -> Self {
        Self {
            id: InstallationId::new(0),
            customer_id,
            branch_id,
            subscription_id: None,
            assigned_technician_id: None,
            status: InstallationStatus::Pending,
            scheduled_date: None,
            completed_at: None,
            installation_type: "new".to_string(),
            notes: None,
        }
    }

    pub fn schedule(&mut self, date: chrono::NaiveDate) {
        self.status = InstallationStatus::Scheduled;
        self.scheduled_date = Some(date);
    }

    pub fn start(&mut self) {
        self.status = InstallationStatus::InProgress;
    }

    pub fn complete(&mut self) -> Result<(), InstallationDomainError> {
        if self.status == InstallationStatus::Completed {
            return Err(InstallationDomainError::AlreadyCompleted);
        }
        if self.scheduled_date.is_none() {
            return Err(InstallationDomainError::CannotCompleteWithoutSchedule);
        }
        self.status = InstallationStatus::Completed;
        self.completed_at = Some(chrono::Utc::now());
        Ok(())
    }

    pub fn cancel(&mut self) -> Result<(), InstallationDomainError> {
        if self.status == InstallationStatus::Completed {
            return Err(InstallationDomainError::AlreadyCompleted);
        }
        self.status = InstallationStatus::Cancelled;
        Ok(())
    }

    pub fn is_completed(&self) -> bool {
        self.status == InstallationStatus::Completed
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new_installation() {
        let inst = InstallationOrder::new(1, 1);
        assert_eq!(inst.status, InstallationStatus::Pending);
    }

    #[test]
    fn test_installation_lifecycle() {
        let mut inst = InstallationOrder::new(1, 1);
        inst.schedule(chrono::Utc::now().date_naive());
        inst.start();
        inst.complete().unwrap();
        assert!(inst.is_completed());
    }
}
