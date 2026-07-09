use sqlx::PgPool;

use crate::common::errors::app_error::AppError;
use crate::modules::ticket::repository::ticket_repository::TicketRepository;
use crate::modules::ticket::request::ticket_request::*;
use crate::modules::ticket::response::ticket_response::*;
use crate::modules::ticket::mapper::ticket_mapper::*;

pub struct TicketService<'a> {
    repo: TicketRepository<'a>,
}

impl<'a> TicketService<'a> {
    pub fn new(pool: &'a PgPool) -> Self { Self { repo: TicketRepository::new(pool) } }

    pub async fn list_tickets(&self, query: TicketQuery) -> Result<TicketListResponse, AppError> {
        let page = query.page.unwrap_or(1);
        let per_page = query.per_page.unwrap_or(20);
        let (tickets, total) = self.repo.list(query.branch_id, query.status.as_deref(), query.priority.as_deref(), query.category.as_deref(), query.assigned_to, query.customer_id, page, per_page).await?;
        let responses: Vec<TicketResponse> = tickets.iter().map(ticket_to_response).collect();
        let total_pages = (total as f64 / per_page as f64).ceil() as i64;
        Ok(TicketListResponse { tickets: responses, total, page, per_page, total_pages })
    }

    pub async fn get_ticket(&self, id: i64) -> Result<TicketResponse, AppError> {
        let ticket = self.repo.get_by_id(id).await?.ok_or_else(|| AppError::NotFound("Ticket not found".into()))?;
        Ok(ticket_to_response(&ticket))
    }

    pub async fn create_ticket(&self, req: CreateTicketRequest, created_by: i64) -> Result<TicketResponse, AppError> {
        let ticket_number = self.generate_ticket_number().await?;
        let ticket = self.repo.create(&ticket_number, req.branch_id, req.customer_id, req.subscription_id, created_by, &req.category, req.subcategory.as_deref(), &req.priority, &req.subject, &req.description, &req.source).await?;
        self.repo.create_status_history(ticket.id, None, &ticket.status, created_by, Some("Ticket created")).await.ok();
        Ok(ticket_to_response(&ticket))
    }

    pub async fn update_ticket(&self, id: i64, user_id: i64, req: UpdateTicketRequest) -> Result<TicketResponse, AppError> {
        let ticket = self.repo.get_by_id(id).await?.ok_or_else(|| AppError::NotFound("Ticket not found".into()))?;
        let updated = if let Some(status) = &req.status {
            let result = self.repo.update_status(id, status, req.resolution_notes.as_deref()).await?;
            self.repo.create_status_history(id, Some(&ticket.status), status, user_id, Some("Status updated")).await.ok();
            result
        } else {
            self.repo.update_status(id, &ticket.status, req.resolution_notes.as_deref()).await?
        };
        Ok(ticket_to_response(&updated))
    }

    pub async fn delete_ticket(&self, id: i64) -> Result<MessageResponse, AppError> {
        if !self.repo.delete(id).await? { return Err(AppError::NotFound("Ticket not found".into())); }
        Ok(MessageResponse { message: "Ticket deleted successfully".into() })
    }

    pub async fn assign_ticket(&self, id: i64, user_id: i64, req: AssignTicketRequest) -> Result<TicketResponse, AppError> {
        let ticket = self.repo.get_by_id(id).await?.ok_or_else(|| AppError::NotFound("Ticket not found".into()))?;
        let result = self.repo.assign(id, req.assigned_to).await.map_err(|_| AppError::NotFound("Ticket not found".into()))?;
        self.repo.create_status_history(id, Some(&ticket.status), &result.status, user_id, Some(&format!("Assigned to user {}", req.assigned_to))).await.ok();
        Ok(ticket_to_response(&result))
    }

    pub async fn escalate_ticket(&self, id: i64, user_id: i64, req: EscalateTicketRequest) -> Result<TicketResponse, AppError> {
        let ticket = self.repo.get_by_id(id).await?.ok_or_else(|| AppError::NotFound("Ticket not found".into()))?;
        let result = self.repo.escalate(id, req.escalated_to, req.new_priority.as_deref()).await.map_err(|_| AppError::NotFound("Ticket not found".into()))?;
        self.repo.create_escalation(id, user_id, req.escalated_to, Some(&ticket.priority), req.new_priority.as_deref(), &req.reason).await.ok();
        self.repo.create_status_history(id, Some(&ticket.status), &result.status, user_id, Some(&req.reason)).await.ok();
        Ok(ticket_to_response(&result))
    }

    pub async fn resolve_ticket(&self, id: i64, user_id: i64, req: ResolveTicketRequest) -> Result<TicketResponse, AppError> {
        let ticket = self.repo.get_by_id(id).await?.ok_or_else(|| AppError::NotFound("Ticket not found".into()))?;
        let result = self.repo.update_status(id, "resolved", Some(&req.resolution_notes)).await.map_err(|_| AppError::NotFound("Ticket not found".into()))?;
        self.repo.create_status_history(id, Some(&ticket.status), "resolved", user_id, Some(&req.resolution_notes)).await.ok();
        Ok(ticket_to_response(&result))
    }

    pub async fn close_ticket(&self, id: i64, user_id: i64, req: CloseTicketRequest) -> Result<TicketResponse, AppError> {
        let ticket = self.repo.get_by_id(id).await?.ok_or_else(|| AppError::NotFound("Ticket not found".into()))?;
        let result = self.repo.update_status(id, "closed", req.closure_notes.as_deref()).await.map_err(|_| AppError::NotFound("Ticket not found".into()))?;
        self.repo.create_status_history(id, Some(&ticket.status), "closed", user_id, req.closure_notes.as_deref()).await.ok();
        Ok(ticket_to_response(&result))
    }

    pub async fn reopen_ticket(&self, id: i64, user_id: i64, req: ReopenTicketRequest) -> Result<TicketResponse, AppError> {
        let ticket = self.repo.get_by_id(id).await?.ok_or_else(|| AppError::NotFound("Ticket not found".into()))?;
        let result = self.repo.increment_reopen(id).await.map_err(|_| AppError::NotFound("Ticket not found".into()))?;
        self.repo.create_status_history(id, Some(&ticket.status), &result.status, user_id, Some(&req.reason)).await.ok();
        Ok(ticket_to_response(&result))
    }

    pub async fn set_feedback(&self, id: i64, req: TicketFeedbackRequest) -> Result<TicketResponse, AppError> {
        let ticket = self.repo.set_feedback(id, req.satisfaction_rating, req.satisfaction_feedback.as_deref()).await.map_err(|_| AppError::NotFound("Ticket not found".into()))?;
        Ok(ticket_to_response(&ticket))
    }

    pub async fn get_comments(&self, ticket_id: i64) -> Result<Vec<TicketCommentResponse>, AppError> {
        let _ = self.repo.get_by_id(ticket_id).await?.ok_or_else(|| AppError::NotFound("Ticket not found".into()))?;
        let comments = self.repo.list_comments(ticket_id).await?;
        Ok(comments.iter().map(comment_to_response).collect())
    }

    pub async fn add_comment(&self, ticket_id: i64, user_id: i64, req: AddCommentRequest) -> Result<TicketCommentResponse, AppError> {
        let _ = self.repo.get_by_id(ticket_id).await?.ok_or_else(|| AppError::NotFound("Ticket not found".into()))?;
        let comment = self.repo.add_comment(ticket_id, Some(user_id), false, &req.comment, req.is_internal.unwrap_or(false), req.attachments).await?;
        Ok(comment_to_response(&comment))
    }

    pub async fn get_my_assignments(&self, assigned_to: i64, page: i64, per_page: i64) -> Result<TicketListResponse, AppError> {
        let (tickets, total) = self.repo.get_my_assignments(assigned_to, page, per_page).await?;
        let responses: Vec<TicketResponse> = tickets.iter().map(ticket_to_response).collect();
        let total_pages = (total as f64 / per_page as f64).ceil() as i64;
        Ok(TicketListResponse { tickets: responses, total, page, per_page, total_pages })
    }

    pub async fn get_dashboard(&self) -> Result<TicketDashboardResponse, AppError> {
        let (total_open, total_in_progress, total_resolved_today, total_overdue) = self.repo.get_dashboard_stats().await?;
        let by_priority: Vec<PriorityCount> = self.repo.get_priority_counts().await?.into_iter().map(|(p, c)| PriorityCount { priority: p, count: c }).collect();
        let by_category: Vec<CategoryCount> = self.repo.get_category_counts().await?.into_iter().map(|(c, n)| CategoryCount { category: c, count: n }).collect();
        Ok(TicketDashboardResponse { total_open, total_in_progress, total_resolved_today, total_overdue, by_priority, by_category })
    }

    // ── Escalation Records ────────────────────────────────

    pub async fn get_escalations(&self, ticket_id: i64) -> Result<Vec<TicketEscalationResponse>, AppError> {
        let _ = self.repo.get_by_id(ticket_id).await?.ok_or_else(|| AppError::NotFound("Ticket not found".into()))?;
        let escalations = self.repo.list_escalations(ticket_id).await?;
        Ok(escalations.iter().map(|e| TicketEscalationResponse { id: e.id, ticket_id: e.ticket_id, from_user_id: e.from_user_id, to_user_id: e.to_user_id, from_priority: e.from_priority.clone(), to_priority: e.to_priority.clone(), reason: e.reason.clone(), escalated_at: e.escalated_at, acknowledged_at: e.acknowledged_at, created_at: e.created_at }).collect())
    }

    // ── Status History ────────────────────────────────────

    pub async fn get_status_history(&self, ticket_id: i64) -> Result<Vec<TicketStatusHistoryResponse>, AppError> {
        let _ = self.repo.get_by_id(ticket_id).await?.ok_or_else(|| AppError::NotFound("Ticket not found".into()))?;
        let history = self.repo.list_status_history(ticket_id).await?;
        Ok(history.iter().map(|h| TicketStatusHistoryResponse { id: h.id, ticket_id: h.ticket_id, old_status: h.old_status.clone(), new_status: h.new_status.clone(), changed_by: h.changed_by, reason: h.reason.clone(), created_at: h.created_at }).collect())
    }

    async fn generate_ticket_number(&self) -> Result<String, AppError> {
        let now = chrono::Utc::now();
        let prefix = format!("TKT-{}-{:02}", now.format("%Y"), now.format("%m"));
        let row: (i64,) = sqlx::query_as("SELECT COUNT(*) + 1 FROM tickets WHERE ticket_number LIKE $1")
            .bind(format!("{}%", prefix))
            .fetch_one(self.repo.pool()).await?;
        Ok(format!("{}-{:04}", prefix, row.0))
    }
}
