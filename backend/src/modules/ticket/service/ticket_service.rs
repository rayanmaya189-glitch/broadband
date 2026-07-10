use sqlx::PgPool;

use crate::common::errors::app_error::AppError;
use crate::modules::ticket::repository::ticket_repository::TicketRepository;
use crate::modules::ticket::request::ticket_request::*;
use crate::modules::ticket::response::ticket_response::*;

pub struct TicketService<'a> {
    repo: TicketRepository<'a>,
}

impl<'a> TicketService<'a> {
    pub fn new(pool: &'a PgPool) -> Self { Self { repo: TicketRepository::new(pool) } }

    pub async fn list_tickets(&self, query: TicketQuery) -> Result<TicketListResponse, AppError> {
        let page = query.page.unwrap_or(1);
        let per_page = query.per_page.unwrap_or(20);
        let (tickets, total) = self.repo.list(query.branch_id, query.status.as_deref(), query.priority.as_deref(), query.category.as_deref(), query.assigned_to, query.customer_id, page, per_page).await?;
        let responses: Vec<TicketResponse> = tickets.into_iter().map(|t| TicketResponse {
            id: t.id, ticket_number: t.ticket_number, branch_id: t.branch_id, customer_id: t.customer_id, subscription_id: t.subscription_id, created_by: t.created_by, assigned_to: t.assigned_to, escalated_to: t.escalated_to, category: t.category, subcategory: t.subcategory, priority: t.priority, status: t.status, subject: t.subject, description: t.description, source: t.source, resolution_notes: t.resolution_notes, sla_response_at: t.sla_response_at, sla_resolution_at: t.sla_resolution_at, first_response_at: t.first_response_at, resolved_at: t.resolved_at, closed_at: t.closed_at, reopen_count: t.reopen_count, satisfaction_rating: t.satisfaction_rating, satisfaction_feedback: t.satisfaction_feedback, created_at: t.created_at, updated_at: t.updated_at, creator_name: None, assignee_name: None, branch_name: None, customer_name: None,
        }).collect();
        let total_pages = (total as f64 / per_page as f64).ceil() as i64;
        Ok(TicketListResponse { tickets: responses, total, page, per_page, total_pages })
    }

    pub async fn get_ticket(&self, id: i64) -> Result<TicketResponse, AppError> {
        let t = self.repo.get_by_id(id).await?.ok_or_else(|| AppError::NotFound("Ticket not found".into()))?;
        Ok(TicketResponse { id: t.id, ticket_number: t.ticket_number, branch_id: t.branch_id, customer_id: t.customer_id, subscription_id: t.subscription_id, created_by: t.created_by, assigned_to: t.assigned_to, escalated_to: t.escalated_to, category: t.category, subcategory: t.subcategory, priority: t.priority, status: t.status, subject: t.subject, description: t.description, source: t.source, resolution_notes: t.resolution_notes, sla_response_at: t.sla_response_at, sla_resolution_at: t.sla_resolution_at, first_response_at: t.first_response_at, resolved_at: t.resolved_at, closed_at: t.closed_at, reopen_count: t.reopen_count, satisfaction_rating: t.satisfaction_rating, satisfaction_feedback: t.satisfaction_feedback, created_at: t.created_at, updated_at: t.updated_at, creator_name: None, assignee_name: None, branch_name: None, customer_name: None })
    }

    pub async fn create_ticket(&self, req: CreateTicketRequest, created_by: i64) -> Result<TicketResponse, AppError> {
        let ticket_number = self.generate_ticket_number().await?;
        let t = self.repo.create(&ticket_number, req.branch_id, req.customer_id, req.subscription_id, created_by, &req.category, req.subcategory.as_deref(), &req.priority, &req.subject, &req.description, &req.source).await?;
        self.repo.create_status_history(t.id, None, &t.status, created_by, Some("Ticket created")).await.ok();
        Ok(TicketResponse { id: t.id, ticket_number: t.ticket_number, branch_id: t.branch_id, customer_id: t.customer_id, subscription_id: t.subscription_id, created_by: t.created_by, assigned_to: t.assigned_to, escalated_to: t.escalated_to, category: t.category, subcategory: t.subcategory, priority: t.priority, status: t.status, subject: t.subject, description: t.description, source: t.source, resolution_notes: t.resolution_notes, sla_response_at: t.sla_response_at, sla_resolution_at: t.sla_resolution_at, first_response_at: t.first_response_at, resolved_at: t.resolved_at, closed_at: t.closed_at, reopen_count: t.reopen_count, satisfaction_rating: t.satisfaction_rating, satisfaction_feedback: t.satisfaction_feedback, created_at: t.created_at, updated_at: t.updated_at, creator_name: None, assignee_name: None, branch_name: None, customer_name: None })
    }

    pub async fn update_ticket(&self, id: i64, user_id: i64, req: UpdateTicketRequest) -> Result<TicketResponse, AppError> {
        let old = self.repo.get_by_id(id).await?.ok_or_else(|| AppError::NotFound("Ticket not found".into()))?;
        let updated = if let Some(status) = &req.status {
            let result = self.repo.update_status(id, status, req.resolution_notes.as_deref()).await?;
            self.repo.create_status_history(id, Some(&old.status), status, user_id, Some("Status updated")).await.ok();
            result
        } else {
            self.repo.update_status(id, &old.status, req.resolution_notes.as_deref()).await?
        };
        Ok(TicketResponse { id: updated.id, ticket_number: updated.ticket_number, branch_id: updated.branch_id, customer_id: updated.customer_id, subscription_id: updated.subscription_id, created_by: updated.created_by, assigned_to: updated.assigned_to, escalated_to: updated.escalated_to, category: updated.category, subcategory: updated.subcategory, priority: updated.priority, status: updated.status, subject: updated.subject, description: updated.description, source: updated.source, resolution_notes: updated.resolution_notes, sla_response_at: updated.sla_response_at, sla_resolution_at: updated.sla_resolution_at, first_response_at: updated.first_response_at, resolved_at: updated.resolved_at, closed_at: updated.closed_at, reopen_count: updated.reopen_count, satisfaction_rating: updated.satisfaction_rating, satisfaction_feedback: updated.satisfaction_feedback, created_at: updated.created_at, updated_at: updated.updated_at, creator_name: None, assignee_name: None, branch_name: None, customer_name: None })
    }

    pub async fn delete_ticket(&self, id: i64) -> Result<MessageResponse, AppError> {
        if !self.repo.delete(id).await? { return Err(AppError::NotFound("Ticket not found".into())); }
        Ok(MessageResponse { message: "Ticket deleted successfully".into() })
    }

    pub async fn assign_ticket(&self, id: i64, user_id: i64, req: AssignTicketRequest) -> Result<TicketResponse, AppError> {
        let old = self.repo.get_by_id(id).await?.ok_or_else(|| AppError::NotFound("Ticket not found".into()))?;
        let result = self.repo.assign(id, req.assigned_to).await.map_err(|_| AppError::NotFound("Ticket not found".into()))?;
        self.repo.create_status_history(id, Some(&old.status), &result.status, user_id, Some(&format!("Assigned to user {}", req.assigned_to))).await.ok();
        Ok(TicketResponse { id: result.id, ticket_number: result.ticket_number, branch_id: result.branch_id, customer_id: result.customer_id, subscription_id: result.subscription_id, created_by: result.created_by, assigned_to: result.assigned_to, escalated_to: result.escalated_to, category: result.category, subcategory: result.subcategory, priority: result.priority, status: result.status, subject: result.subject, description: result.description, source: result.source, resolution_notes: result.resolution_notes, sla_response_at: result.sla_response_at, sla_resolution_at: result.sla_resolution_at, first_response_at: result.first_response_at, resolved_at: result.resolved_at, closed_at: result.closed_at, reopen_count: result.reopen_count, satisfaction_rating: result.satisfaction_rating, satisfaction_feedback: result.satisfaction_feedback, created_at: result.created_at, updated_at: result.updated_at, creator_name: None, assignee_name: None, branch_name: None, customer_name: None })
    }

    pub async fn escalate_ticket(&self, id: i64, user_id: i64, req: EscalateTicketRequest) -> Result<TicketResponse, AppError> {
        let old = self.repo.get_by_id(id).await?.ok_or_else(|| AppError::NotFound("Ticket not found".into()))?;
        let result = self.repo.escalate(id, req.escalated_to, req.new_priority.as_deref()).await.map_err(|_| AppError::NotFound("Ticket not found".into()))?;
        self.repo.create_escalation(id, user_id, req.escalated_to, Some(&old.priority), req.new_priority.as_deref(), &req.reason).await.ok();
        self.repo.create_status_history(id, Some(&old.status), &result.status, user_id, Some(&req.reason)).await.ok();
        Ok(TicketResponse { id: result.id, ticket_number: result.ticket_number, branch_id: result.branch_id, customer_id: result.customer_id, subscription_id: result.subscription_id, created_by: result.created_by, assigned_to: result.assigned_to, escalated_to: result.escalated_to, category: result.category, subcategory: result.subcategory, priority: result.priority, status: result.status, subject: result.subject, description: result.description, source: result.source, resolution_notes: result.resolution_notes, sla_response_at: result.sla_response_at, sla_resolution_at: result.sla_resolution_at, first_response_at: result.first_response_at, resolved_at: result.resolved_at, closed_at: result.closed_at, reopen_count: result.reopen_count, satisfaction_rating: result.satisfaction_rating, satisfaction_feedback: result.satisfaction_feedback, created_at: result.created_at, updated_at: result.updated_at, creator_name: None, assignee_name: None, branch_name: None, customer_name: None })
    }

    pub async fn resolve_ticket(&self, id: i64, user_id: i64, req: ResolveTicketRequest) -> Result<TicketResponse, AppError> {
        let old = self.repo.get_by_id(id).await?.ok_or_else(|| AppError::NotFound("Ticket not found".into()))?;
        let result = self.repo.update_status(id, "resolved", Some(&req.resolution_notes)).await.map_err(|_| AppError::NotFound("Ticket not found".into()))?;
        self.repo.create_status_history(id, Some(&old.status), "resolved", user_id, Some(&req.resolution_notes)).await.ok();
        Ok(TicketResponse { id: result.id, ticket_number: result.ticket_number, branch_id: result.branch_id, customer_id: result.customer_id, subscription_id: result.subscription_id, created_by: result.created_by, assigned_to: result.assigned_to, escalated_to: result.escalated_to, category: result.category, subcategory: result.subcategory, priority: result.priority, status: result.status, subject: result.subject, description: result.description, source: result.source, resolution_notes: result.resolution_notes, sla_response_at: result.sla_response_at, sla_resolution_at: result.sla_resolution_at, first_response_at: result.first_response_at, resolved_at: result.resolved_at, closed_at: result.closed_at, reopen_count: result.reopen_count, satisfaction_rating: result.satisfaction_rating, satisfaction_feedback: result.satisfaction_feedback, created_at: result.created_at, updated_at: result.updated_at, creator_name: None, assignee_name: None, branch_name: None, customer_name: None })
    }

    pub async fn close_ticket(&self, id: i64, user_id: i64, req: CloseTicketRequest) -> Result<TicketResponse, AppError> {
        let old = self.repo.get_by_id(id).await?.ok_or_else(|| AppError::NotFound("Ticket not found".into()))?;
        let result = self.repo.update_status(id, "closed", req.closure_notes.as_deref()).await.map_err(|_| AppError::NotFound("Ticket not found".into()))?;
        self.repo.create_status_history(id, Some(&old.status), "closed", user_id, req.closure_notes.as_deref()).await.ok();
        Ok(TicketResponse { id: result.id, ticket_number: result.ticket_number, branch_id: result.branch_id, customer_id: result.customer_id, subscription_id: result.subscription_id, created_by: result.created_by, assigned_to: result.assigned_to, escalated_to: result.escalated_to, category: result.category, subcategory: result.subcategory, priority: result.priority, status: result.status, subject: result.subject, description: result.description, source: result.source, resolution_notes: result.resolution_notes, sla_response_at: result.sla_response_at, sla_resolution_at: result.sla_resolution_at, first_response_at: result.first_response_at, resolved_at: result.resolved_at, closed_at: result.closed_at, reopen_count: result.reopen_count, satisfaction_rating: result.satisfaction_rating, satisfaction_feedback: result.satisfaction_feedback, created_at: result.created_at, updated_at: result.updated_at, creator_name: None, assignee_name: None, branch_name: None, customer_name: None })
    }

    pub async fn reopen_ticket(&self, id: i64, user_id: i64, req: ReopenTicketRequest) -> Result<TicketResponse, AppError> {
        let old = self.repo.get_by_id(id).await?.ok_or_else(|| AppError::NotFound("Ticket not found".into()))?;
        let result = self.repo.increment_reopen(id).await.map_err(|_| AppError::NotFound("Ticket not found".into()))?;
        self.repo.create_status_history(id, Some(&old.status), &result.status, user_id, Some(&req.reason)).await.ok();
        Ok(TicketResponse { id: result.id, ticket_number: result.ticket_number, branch_id: result.branch_id, customer_id: result.customer_id, subscription_id: result.subscription_id, created_by: result.created_by, assigned_to: result.assigned_to, escalated_to: result.escalated_to, category: result.category, subcategory: result.subcategory, priority: result.priority, status: result.status, subject: result.subject, description: result.description, source: result.source, resolution_notes: result.resolution_notes, sla_response_at: result.sla_response_at, sla_resolution_at: result.sla_resolution_at, first_response_at: result.first_response_at, resolved_at: result.resolved_at, closed_at: result.closed_at, reopen_count: result.reopen_count, satisfaction_rating: result.satisfaction_rating, satisfaction_feedback: result.satisfaction_feedback, created_at: result.created_at, updated_at: result.updated_at, creator_name: None, assignee_name: None, branch_name: None, customer_name: None })
    }

    pub async fn set_feedback(&self, id: i64, req: TicketFeedbackRequest) -> Result<TicketResponse, AppError> {
        let t = self.repo.set_feedback(id, req.satisfaction_rating, req.satisfaction_feedback.as_deref()).await.map_err(|_| AppError::NotFound("Ticket not found".into()))?;
        Ok(TicketResponse { id: t.id, ticket_number: t.ticket_number, branch_id: t.branch_id, customer_id: t.customer_id, subscription_id: t.subscription_id, created_by: t.created_by, assigned_to: t.assigned_to, escalated_to: t.escalated_to, category: t.category, subcategory: t.subcategory, priority: t.priority, status: t.status, subject: t.subject, description: t.description, source: t.source, resolution_notes: t.resolution_notes, sla_response_at: t.sla_response_at, sla_resolution_at: t.sla_resolution_at, first_response_at: t.first_response_at, resolved_at: t.resolved_at, closed_at: t.closed_at, reopen_count: t.reopen_count, satisfaction_rating: t.satisfaction_rating, satisfaction_feedback: t.satisfaction_feedback, created_at: t.created_at, updated_at: t.updated_at, creator_name: None, assignee_name: None, branch_name: None, customer_name: None })
    }

    pub async fn get_comments(&self, ticket_id: i64) -> Result<Vec<TicketCommentResponse>, AppError> {
        let _ = self.repo.get_by_id(ticket_id).await?.ok_or_else(|| AppError::NotFound("Ticket not found".into()))?;
        let comments = self.repo.list_comments(ticket_id).await?;
        Ok(comments.into_iter().map(|c| TicketCommentResponse { id: c.id, ticket_id: c.ticket_id, user_id: c.user_id, is_customer: c.is_customer, comment: c.comment, is_internal: c.is_internal, attachments: c.attachments, created_at: c.created_at, user_name: None }).collect())
    }

    pub async fn add_comment(&self, ticket_id: i64, user_id: i64, req: AddCommentRequest) -> Result<TicketCommentResponse, AppError> {
        let _ = self.repo.get_by_id(ticket_id).await?.ok_or_else(|| AppError::NotFound("Ticket not found".into()))?;
        let c = self.repo.add_comment(ticket_id, Some(user_id), false, &req.comment, req.is_internal.unwrap_or(false), req.attachments).await?;
        Ok(TicketCommentResponse { id: c.id, ticket_id: c.ticket_id, user_id: c.user_id, is_customer: c.is_customer, comment: c.comment, is_internal: c.is_internal, attachments: c.attachments, created_at: c.created_at, user_name: None })
    }

    pub async fn get_my_assignments(&self, assigned_to: i64, page: i64, per_page: i64) -> Result<TicketListResponse, AppError> {
        let (tickets, total) = self.repo.get_my_assignments(assigned_to, page, per_page).await?;
        let responses: Vec<TicketResponse> = tickets.into_iter().map(|t| TicketResponse {
            id: t.id, ticket_number: t.ticket_number, branch_id: t.branch_id, customer_id: t.customer_id, subscription_id: t.subscription_id, created_by: t.created_by, assigned_to: t.assigned_to, escalated_to: t.escalated_to, category: t.category, subcategory: t.subcategory, priority: t.priority, status: t.status, subject: t.subject, description: t.description, source: t.source, resolution_notes: t.resolution_notes, sla_response_at: t.sla_response_at, sla_resolution_at: t.sla_resolution_at, first_response_at: t.first_response_at, resolved_at: t.resolved_at, closed_at: t.closed_at, reopen_count: t.reopen_count, satisfaction_rating: t.satisfaction_rating, satisfaction_feedback: t.satisfaction_feedback, created_at: t.created_at, updated_at: t.updated_at, creator_name: None, assignee_name: None, branch_name: None, customer_name: None,
        }).collect();
        let total_pages = (total as f64 / per_page as f64).ceil() as i64;
        Ok(TicketListResponse { tickets: responses, total, page, per_page, total_pages })
    }

    pub async fn get_dashboard(&self) -> Result<TicketDashboardResponse, AppError> {
        let (total_open, total_in_progress, total_resolved_today, total_overdue) = self.repo.get_dashboard_stats().await?;
        let by_priority: Vec<PriorityCount> = self.repo.get_priority_counts().await?.into_iter().map(|(p, c)| PriorityCount { priority: p, count: c }).collect();
        let by_category: Vec<CategoryCount> = self.repo.get_category_counts().await?.into_iter().map(|(c, n)| CategoryCount { category: c, count: n }).collect();
        Ok(TicketDashboardResponse { total_open, total_in_progress, total_resolved_today, total_overdue, by_priority, by_category })
    }

    pub async fn get_escalations(&self, ticket_id: i64) -> Result<Vec<TicketEscalationResponse>, AppError> {
        let _ = self.repo.get_by_id(ticket_id).await?.ok_or_else(|| AppError::NotFound("Ticket not found".into()))?;
        let escalations = self.repo.list_escalations(ticket_id).await?;
        Ok(escalations.into_iter().map(|e| TicketEscalationResponse { id: e.id, ticket_id: e.ticket_id, from_user_id: e.from_user_id, to_user_id: e.to_user_id, from_priority: e.from_priority, to_priority: e.to_priority, reason: e.reason, escalated_at: e.escalated_at, acknowledged_at: e.acknowledged_at, created_at: e.created_at }).collect())
    }

    pub async fn get_status_history(&self, ticket_id: i64) -> Result<Vec<TicketStatusHistoryResponse>, AppError> {
        let _ = self.repo.get_by_id(ticket_id).await?.ok_or_else(|| AppError::NotFound("Ticket not found".into()))?;
        let history = self.repo.list_status_history(ticket_id).await?;
        Ok(history.into_iter().map(|h| TicketStatusHistoryResponse { id: h.id, ticket_id: h.ticket_id, old_status: h.old_status, new_status: h.new_status, changed_by: h.changed_by, reason: h.reason, created_at: h.created_at }).collect())
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
