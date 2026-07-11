//! SeaORM-based service for the Ticket domain.

use sea_orm::DatabaseConnection;

use crate::common::errors::app_error::AppError;
use crate::modules::ticket::repository::ticket_repository::TicketRepository;
use crate::modules::ticket::request::ticket_request::*;
use crate::modules::ticket::response::ticket_response::*;

pub struct TicketService<'a> {
    repo: TicketRepository<'a>,
}

impl<'a> TicketService<'a> {
    pub fn new(db: &'a DatabaseConnection) -> Self {
        Self { repo: TicketRepository::new(db) }
    }

    fn to_response(t: crate::modules::ticket::model::ticket_entity::Model) -> TicketResponse {
        TicketResponse {
            id: t.id, ticket_number: t.ticket_number, branch_id: t.branch_id,
            customer_id: t.customer_id, subscription_id: t.subscription_id,
            created_by: t.created_by, assigned_to: t.assigned_to, escalated_to: t.escalated_to,
            category: t.category, subcategory: t.subcategory, priority: t.priority, status: t.status,
            subject: t.subject, description: t.description, source: t.source,
            resolution_notes: t.resolution_notes, sla_response_at: None, sla_resolution_at: None,
            first_response_at: None, resolved_at: t.resolved_at.map(|v| v.into()),
            closed_at: t.closed_at.map(|v| v.into()),
            reopen_count: t.reopen_count,
            satisfaction_rating: t.satisfaction_rating, satisfaction_feedback: t.satisfaction_feedback,
            created_at: t.created_at.into(), updated_at: t.updated_at.into(),
            creator_name: None, assignee_name: None, branch_name: None, customer_name: None,
        }
    }

    pub async fn list(&self, branch_id: Option<i64>, status: Option<&str>, priority: Option<&str>, category: Option<&str>, assigned_to: Option<i64>, customer_id: Option<i64>, page: i64, per_page: i64) -> Result<(Vec<TicketResponse>, i64), AppError> {
        let (tickets, total) = self.repo.list(branch_id, status, priority, category, assigned_to, customer_id, page, per_page).await?;
        Ok((tickets.into_iter().map(Self::to_response).collect(), total))
    }

    pub async fn get_by_id(&self, id: i64) -> Result<TicketResponse, AppError> {
        let t = self.repo.get_by_id(id).await?.ok_or_else(|| AppError::NotFound("Ticket not found".into()))?;
        Ok(Self::to_response(t))
    }

    pub async fn create(&self, branch_id: i64, created_by: i64, req: CreateTicketRequest) -> Result<TicketResponse, AppError> {
        let prefix = format!("TKT-{}", chrono::Utc::now().format("%Y%m"));
        let ticket_number = format!("{}-{:04}", prefix, rand::random::<u16>() % 10000);
        let t = self.repo.create(&ticket_number, branch_id, req.customer_id, req.subscription_id, created_by, &req.category, req.subcategory.as_deref(), &req.priority, &req.subject, &req.description, &req.source).await?;
        Ok(Self::to_response(t))
    }

    pub async fn update(&self, id: i64, req: &UpdateTicketRequest) -> Result<TicketResponse, AppError> {
        let t = self.repo.get_by_id(id).await?.ok_or_else(|| AppError::NotFound("Ticket not found".into()))?;
        let status = req.status.as_deref().unwrap_or(&t.status);
        let notes = req.resolution_notes.as_deref();
        let t = self.repo.update_status(id, status, notes).await?;
        Ok(Self::to_response(t))
    }

    pub async fn update_status(&self, id: i64, status: &str, resolution_notes: Option<&str>) -> Result<TicketResponse, AppError> {
        let t = self.repo.update_status(id, status, resolution_notes).await?;
        Ok(Self::to_response(t))
    }

    pub async fn delete(&self, id: i64) -> Result<MessageResponse, AppError> {
        if !self.repo.delete(id).await? {
            return Err(AppError::NotFound("Ticket not found".into()));
        }
        Ok(MessageResponse { message: "Ticket deleted".into() })
    }

    pub async fn assign(&self, id: i64, assigned_to: i64) -> Result<TicketResponse, AppError> {
        let t = self.repo.assign(id, assigned_to).await?;
        Ok(Self::to_response(t))
    }

    pub async fn escalate(&self, id: i64, escalated_to: i64, new_priority: Option<&str>) -> Result<TicketResponse, AppError> {
        self.repo.create_escalation(id, 0, escalated_to, None, new_priority, "Escalated").await?;
        let t = self.repo.escalate(id, escalated_to, new_priority).await?;
        Ok(Self::to_response(t))
    }

    pub async fn reopen(&self, id: i64) -> Result<TicketResponse, AppError> {
        let t = self.repo.increment_reopen(id).await?;
        Ok(Self::to_response(t))
    }

    pub async fn set_feedback(&self, id: i64, rating: Option<i32>, feedback: Option<&str>) -> Result<TicketResponse, AppError> {
        let t = self.repo.set_feedback(id, rating, feedback).await?;
        Ok(Self::to_response(t))
    }

    pub async fn add_comment(&self, ticket_id: i64, user_id: Option<i64>, is_customer: bool, comment: &str, is_internal: bool) -> Result<TicketCommentResponse, AppError> {
        let c = self.repo.add_comment(ticket_id, user_id, is_customer, comment, is_internal, None).await?;
        Ok(TicketCommentResponse {
            id: c.id, ticket_id: c.ticket_id, user_id: c.user_id, is_customer: c.is_customer,
            comment: c.comment, is_internal: c.is_internal, attachments: c.attachments,
            created_at: c.created_at.into(), updated_at: c.updated_at.into(), user_name: None,
        })
    }

    pub async fn list_comments(&self, ticket_id: i64) -> Result<Vec<TicketCommentResponse>, AppError> {
        let comments = self.repo.list_comments(ticket_id).await?;
        Ok(comments.into_iter().map(|c| TicketCommentResponse {
            id: c.id, ticket_id: c.ticket_id, user_id: c.user_id, is_customer: c.is_customer,
            comment: c.comment, is_internal: c.is_internal, attachments: c.attachments,
            created_at: c.created_at.into(), updated_at: c.updated_at.into(), user_name: None,
        }).collect())
    }

    pub async fn get_escalations(&self, ticket_id: i64) -> Result<Vec<TicketEscalationResponse>, AppError> {
        let escalations = self.repo.list_escalations(ticket_id).await?;
        Ok(escalations.into_iter().map(|e| TicketEscalationResponse {
            id: e.id, ticket_id: e.ticket_id, from_user_id: e.from_user_id, to_user_id: e.to_user_id,
            from_priority: e.from_priority, to_priority: e.to_priority, reason: e.reason,
            escalated_at: e.escalated_at.into(), acknowledged_at: None, created_at: e.created_at.into(),
        }).collect())
    }

    pub async fn get_status_history(&self, ticket_id: i64) -> Result<Vec<TicketStatusHistoryResponse>, AppError> {
        let history = self.repo.list_status_history(ticket_id).await?;
        Ok(history.into_iter().map(|h| TicketStatusHistoryResponse {
            id: h.id, ticket_id: h.ticket_id, old_status: h.old_status, new_status: h.new_status,
            changed_by: h.changed_by, reason: h.reason, created_at: h.created_at.into(),
        }).collect())
    }

    pub async fn get_my_assignments(&self, technician_id: i64) -> Result<Vec<TicketResponse>, AppError> {
        let (tickets, _) = self.repo.get_my_assignments(technician_id, 1, 100).await?;
        Ok(tickets.into_iter().map(Self::to_response).collect())
    }

    pub async fn get_dashboard(&self) -> Result<TicketDashboardResponse, AppError> {
        let (open, _) = self.repo.list(None, Some("open"), None, None, None, None, 1, 1).await?;
        let (in_progress, _) = self.repo.list(None, Some("in_progress"), None, None, None, None, 1, 1).await?;
        let (resolved_today, _) = self.repo.list(None, Some("resolved"), None, None, None, None, 1, 1).await?;
        Ok(TicketDashboardResponse {
            total_open: open.len() as i64,
            total_in_progress: in_progress.len() as i64,
            total_resolved_today: resolved_today.len() as i64,
            total_overdue: 0,
            by_priority: vec![],
            by_category: vec![],
        })
    }
}
