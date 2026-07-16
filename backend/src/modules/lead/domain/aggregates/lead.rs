use crate::modules::lead::domain::value_objects::{LeadId, LeadSource, LeadStatus};

/// Lead aggregate root - represents a sales lead
#[derive(Debug, Clone)]
pub struct Lead {
    pub id: LeadId,
    pub branch_id: i64,
    pub assigned_to: Option<i64>,
    pub name: String,
    pub phone: String,
    pub email: Option<String>,
    pub source: LeadSource,
    pub status: LeadStatus,
    pub interested_plan_id: Option<i64>,
    pub lost_reason: Option<String>,
    pub converted_customer_id: Option<i64>,
}

/// Domain errors for Lead aggregate
#[derive(Debug, Clone, PartialEq)]
pub enum LeadDomainError {
    LeadNotFound(i64),
    AlreadyConverted,
    AlreadyLost,
    InvalidPipelineTransition,
    CannotConvertFromLost,
}

impl std::fmt::Display for LeadDomainError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::LeadNotFound(id) => write!(f, "Lead {} not found", id),
            Self::AlreadyConverted => write!(f, "Lead is already converted"),
            Self::AlreadyLost => write!(f, "Lead is already lost"),
            Self::InvalidPipelineTransition => write!(f, "Invalid pipeline status transition"),
            Self::CannotConvertFromLost => write!(f, "Cannot convert a lost lead"),
        }
    }
}

impl std::error::Error for LeadDomainError {}

impl Lead {
    pub fn new(branch_id: i64, name: String, phone: String, email: Option<String>, source: LeadSource) -> Self {
        Self {
            id: LeadId::new(0),
            branch_id,
            assigned_to: None,
            name,
            phone,
            email,
            source,
            status: LeadStatus::New,
            interested_plan_id: None,
            lost_reason: None,
            converted_customer_id: None,
        }
    }

    pub fn advance(&mut self) -> Result<(), LeadDomainError> {
        self.status = match self.status {
            LeadStatus::New => LeadStatus::Contacted,
            LeadStatus::Contacted => LeadStatus::Interested,
            LeadStatus::Interested => LeadStatus::Surveyed,
            LeadStatus::Surveyed => LeadStatus::Quoted,
            _ => return Err(LeadDomainError::InvalidPipelineTransition),
        };
        Ok(())
    }

    pub fn convert(&mut self, customer_id: i64) -> Result<(), LeadDomainError> {
        if self.status == LeadStatus::Lost {
            return Err(LeadDomainError::CannotConvertFromLost);
        }
        if self.status == LeadStatus::Converted {
            return Err(LeadDomainError::AlreadyConverted);
        }
        self.status = LeadStatus::Converted;
        self.converted_customer_id = Some(customer_id);
        Ok(())
    }

    pub fn mark_lost(&mut self, reason: String) -> Result<(), LeadDomainError> {
        if self.status == LeadStatus::Converted {
            return Err(LeadDomainError::AlreadyConverted);
        }
        self.status = LeadStatus::Lost;
        self.lost_reason = Some(reason);
        Ok(())
    }

    pub fn is_converted(&self) -> bool {
        self.status == LeadStatus::Converted
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new_lead() {
        let lead = Lead::new(1, "Priya".to_string(), "+919876543210".to_string(), None, LeadSource::LandingPage);
        assert_eq!(lead.status, LeadStatus::New);
    }

    #[test]
    fn test_lead_pipeline() {
        let mut lead = Lead::new(1, "Priya".to_string(), "+919876543210".to_string(), None, LeadSource::Referral);
        lead.advance().unwrap(); // contacted
        lead.advance().unwrap(); // interested
        lead.advance().unwrap(); // surveyed
        lead.advance().unwrap(); // quoted
        assert_eq!(lead.status, LeadStatus::Quoted);
        lead.convert(100).unwrap();
        assert!(lead.is_converted());
    }

    #[test]
    fn test_mark_lost() {
        let mut lead = Lead::new(1, "Priya".to_string(), "+919876543210".to_string(), None, LeadSource::WalkIn);
        lead.mark_lost("Not interested".to_string()).unwrap();
        assert_eq!(lead.status, LeadStatus::Lost);
    }
}
