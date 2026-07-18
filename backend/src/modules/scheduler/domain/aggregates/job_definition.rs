use crate::modules::scheduler::domain::value_objects::{JobStatus, JobType};

/// JobDefinition aggregate root - represents a scheduled job
#[derive(Debug, Clone, PartialEq)]
pub struct JobDefinition {
    pub id: i64,
    pub name: String,
    pub job_type: JobType,
    pub cron_expression: String,
    pub is_active: bool,
    pub status: JobStatus,
    pub max_retries: i32,
    pub timeout_seconds: i32,
}

/// Domain errors for JobDefinition aggregate
#[derive(Debug, Clone, PartialEq)]
pub enum SchedulerDomainError {
    JobNotFound(i64),
    InvalidCronExpression,
    JobAlreadyActive,
    JobAlreadyInactive,
}

impl std::fmt::Display for SchedulerDomainError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::JobNotFound(id) => write!(f, "Job {} not found", id),
            Self::InvalidCronExpression => write!(f, "Invalid cron expression"),
            Self::JobAlreadyActive => write!(f, "Job is already active"),
            Self::JobAlreadyInactive => write!(f, "Job is already inactive"),
        }
    }
}

impl std::error::Error for SchedulerDomainError {}

impl JobDefinition {
    pub fn new(
        name: String,
        job_type: JobType,
        cron_expression: String,
    ) -> Result<Self, SchedulerDomainError> {
        // Basic cron validation: should have 5 parts separated by spaces
        let parts: Vec<&str> = cron_expression.split_whitespace().collect();
        if parts.len() != 5 {
            return Err(SchedulerDomainError::InvalidCronExpression);
        }
        Ok(Self {
            id: 0,
            name,
            job_type,
            cron_expression,
            is_active: true,
            status: JobStatus::Idle,
            max_retries: 3,
            timeout_seconds: 300,
        })
    }

    pub fn activate(&mut self) -> Result<(), SchedulerDomainError> {
        if self.is_active {
            return Err(SchedulerDomainError::JobAlreadyActive);
        }
        self.status = JobStatus::Idle;
        self.is_active = true;
        Ok(())
    }

    pub fn deactivate(&mut self) -> Result<(), SchedulerDomainError> {
        if !self.is_active {
            return Err(SchedulerDomainError::JobAlreadyInactive);
        }
        self.is_active = false;
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new_job() {
        let job = JobDefinition::new(
            "Generate Invoices".to_string(),
            JobType::Billing,
            "0 1 1 * *".to_string(),
        );
        assert!(job.is_ok());
        assert_eq!(job.unwrap().status, JobStatus::Idle);
    }

    #[test]
    fn test_invalid_cron() {
        let job = JobDefinition::new(
            "Invalid".to_string(),
            JobType::Billing,
            "invalid".to_string(),
        );
        assert_eq!(job, Err(SchedulerDomainError::InvalidCronExpression));
    }
}
