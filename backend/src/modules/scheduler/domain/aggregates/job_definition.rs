use crate::modules::scheduler::domain::value_objects::{JobDefinitionId, JobStatus, JobType};

/// JobDefinition aggregate root - represents a scheduled job configuration
#[derive(Debug, Clone)]
pub struct JobDefinition {
    pub id: JobDefinitionId,
    pub name: String,
    pub job_type: JobType,
    pub cron_expression: String,
    pub is_active: bool,
    pub last_run_at: Option<chrono::DateTime<chrono::Utc>>,
    pub next_run_at: Option<chrono::DateTime<chrono::Utc>>,
    pub status: JobStatus,
}

/// Domain errors for JobDefinition aggregate
#[derive(Debug, Clone, PartialEq)]
pub enum SchedulerDomainError {
    JobNotFound(i64),
    InvalidCronExpression,
    JobAlreadyRunning,
}

impl std::fmt::Display for SchedulerDomainError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::JobNotFound(id) => write!(f, "Job {} not found", id),
            Self::InvalidCronExpression => write!(f, "Invalid cron expression"),
            Self::JobAlreadyRunning => write!(f, "Job is already running"),
        }
    }
}

impl std::error::Error for SchedulerDomainError {}

impl JobDefinition {
    pub fn new(name: String, job_type: JobType, cron_expression: String) -> Result<Self, SchedulerDomainError> {
        if !Self::is_valid_cron(&cron_expression) {
            return Err(SchedulerDomainError::InvalidCronExpression);
        }
        Ok(Self {
            id: JobDefinitionId::new(0),
            name,
            job_type,
            cron_expression,
            is_active: true,
            last_run_at: None,
            next_run_at: None,
            status: JobStatus::Idle,
        })
    }

    fn is_valid_cron(expr: &str) -> bool {
        let parts: Vec<&str> = expr.split_whitespace().collect();
        parts.len() == 5
    }

    pub fn mark_running(&mut self) -> Result<(), SchedulerDomainError> {
        if self.status == JobStatus::Running {
            return Err(SchedulerDomainError::JobAlreadyRunning);
        }
        self.status = JobStatus::Running;
        self.last_run_at = Some(chrono::Utc::now());
        Ok(())
    }

    pub fn mark_completed(&mut self) {
        self.status = JobStatus::Idle;
    }

    pub fn mark_failed(&mut self) {
        self.status = JobStatus::Failed;
    }

    pub fn disable(&mut self) {
        self.is_active = false;
    }

    pub fn enable(&mut self) {
        self.is_active = true;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new_job() {
        let job = JobDefinition::new("Daily Invoice".to_string(), JobType::Billing, "0 1 * * *".to_string());
        assert!(job.is_ok());
        assert_eq!(job.unwrap().status, JobStatus::Idle);
    }

    #[test]
    fn test_invalid_cron() {
        let job = JobDefinition::new("Bad".to_string(), JobType::Billing, "invalid".to_string());
        assert_eq!(job, Err(SchedulerDomainError::InvalidCronExpression));
    }
}
