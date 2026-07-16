use crate::modules::ticket::domain::value_objects::{TicketId, TicketPriority, TicketStatus};

/// Ticket aggregate root - represents a support ticket
#[derive(Debug, Clone)]
pub struct Ticket {
    pub id: TicketId,
    pub ticket_number: String,
    pub branch_id: i64,
    pub customer_id: Option<i64>,
    pub subscription_id: Option<i64>,
    pub created_by: i64,
    pub assigned_to: Option<i64>,
    pub category: String,
    pub subcategory: Option<String>,
    pub priority: TicketPriority,
    pub status: TicketStatus,
    pub subject: String,
    pub description: String,
    pub source: String,
    pub resolution_notes: Option<String>,
    pub satisfaction_rating: Option<i32>,
}

/// Domain errors for Ticket aggregate
#[derive(Debug, Clone, PartialEq)]
pub enum TicketDomainError {
    TicketNotFound(i64),
    AlreadyResolved,
    AlreadyClosed,
    CannotReopenResolvedTicket,
    InvalidPriority,
    InvalidCategory,
    CannotAssignClosedTicket,
}

impl std::fmt::Display for TicketDomainError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::TicketNotFound(id) => write!(f, "Ticket {} not found", id),
            Self::AlreadyResolved => write!(f, "Ticket is already resolved"),
            Self::AlreadyClosed => write!(f, "Ticket is already closed"),
            Self::CannotReopenResolvedTicket => write!(f, "Cannot reopen a resolved ticket"),
            Self::InvalidPriority => write!(f, "Invalid ticket priority"),
            Self::InvalidCategory => write!(f, "Invalid ticket category"),
            Self::CannotAssignClosedTicket => write!(f, "Cannot assign a closed ticket"),
        }
    }
}

impl std::error::Error for TicketDomainError {}

impl Ticket {
    pub fn new(
        ticket_number: String,
        branch_id: i64,
        customer_id: Option<i64>,
        created_by: i64,
        category: String,
        priority: TicketPriority,
        subject: String,
        description: String,
        source: String,
    ) -> Result<Self, TicketDomainError> {
        Ok(Self {
            id: TicketId::new(0),
            ticket_number,
            branch_id,
            customer_id,
            subscription_id: None,
            created_by,
            assigned_to: None,
            category,
            subcategory: None,
            priority,
            status: TicketStatus::Open,
            subject,
            description,
            source,
            resolution_notes: None,
            satisfaction_rating: None,
        })
    }

    pub fn assign(&mut self, assigned_to: i64) -> Result<(), TicketDomainError> {
        if self.status == TicketStatus::Closed {
            return Err(TicketDomainError::CannotAssignClosedTicket);
        }
        self.assigned_to = Some(assigned_to);
        self.status = TicketStatus::Assigned;
        Ok(())
    }

    pub fn start_progress(&mut self) {
        self.status = TicketStatus::InProgress;
    }

    pub fn resolve(&mut self, notes: Option<String>) -> Result<(), TicketDomainError> {
        if self.status == TicketStatus::Resolved {
            return Err(TicketDomainError::AlreadyResolved);
        }
        self.status = TicketStatus::Resolved;
        self.resolution_notes = notes;
        Ok(())
    }

    pub fn close(&mut self) -> Result<(), TicketDomainError> {
        if self.status == TicketStatus::Closed {
            return Err(TicketDomainError::AlreadyClosed);
        }
        self.status = TicketStatus::Closed;
        Ok(())
    }

    pub fn reopen(&mut self) -> Result<(), TicketDomainError> {
        if self.status == TicketStatus::Resolved {
            return Err(TicketDomainError::CannotReopenResolvedTicket);
        }
        self.status = TicketStatus::Open;
        self.assigned_to = None;
        Ok(())
    }

    pub fn escalate(&mut self) {
        self.priority = match self.priority {
            TicketPriority::Low => TicketPriority::Medium,
            TicketPriority::Medium => TicketPriority::High,
            TicketPriority::High | TicketPriority::Critical => TicketPriority::Critical,
        };
        self.status = TicketStatus::Escalated;
    }

    pub fn is_open(&self) -> bool {
        !matches!(self.status, TicketStatus::Resolved | TicketStatus::Closed)
    }

    pub fn rate(&mut self, rating: i32, _feedback: Option<String>) {
        self.satisfaction_rating = Some(rating.clamp(1, 5));
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new_ticket() {
        let ticket = Ticket::new(
            "TKT-001".to_string(), 1, Some(1), 1,
            "connectivity".to_string(), TicketPriority::Medium,
            "No internet".to_string(), "Customer has no internet".to_string(),
            "customer".to_string(),
        );
        assert!(ticket.is_ok());
        let ticket = ticket.unwrap();
        assert!(ticket.is_open());
    }

    #[test]
    fn test_ticket_lifecycle() {
        let mut ticket = Ticket::new(
            "TKT-001".to_string(), 1, Some(1), 1,
            "connectivity".to_string(), TicketPriority::High,
            "No internet".to_string(), "Description".to_string(),
            "customer".to_string(),
        ).unwrap();
        ticket.assign(2).unwrap();
        assert_eq!(ticket.status, TicketStatus::Assigned);
        ticket.start_progress();
        ticket.resolve(Some("Fixed".to_string())).unwrap();
        assert_eq!(ticket.status, TicketStatus::Resolved);
        ticket.close().unwrap();
        assert_eq!(ticket.status, TicketStatus::Closed);
    }

    #[test]
    fn test_escalate_ticket() {
        let mut ticket = Ticket::new(
            "TKT-001".to_string(), 1, None, 1,
            "connectivity".to_string(), TicketPriority::Low,
            "Issue".to_string(), "Desc".to_string(),
            "phone".to_string(),
        ).unwrap();
        ticket.escalate();
        assert_eq!(ticket.priority, TicketPriority::Medium);
        assert_eq!(ticket.status, TicketStatus::Escalated);
    }
}
