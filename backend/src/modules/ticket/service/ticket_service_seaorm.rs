//! SeaORM-based service for the Ticket domain.

use sea_orm::DatabaseConnection;

use crate::common::errors::app_error::AppError;
use crate::modules::ticket::repository::ticket_repository_seaorm::TicketRepositorySeaorm;
use crate::modules::ticket::request::ticket_request::*;
use crate::modules::ticket::response::ticket_response::*;

pub struct TicketServiceSeaorm<'a> {
    repo: TicketRepositorySeaorm<'a>,
}

impl<'a> TicketServiceSeaorm<'a> {
    pub fn new(db: &'a DatabaseConnection) -> Self {
        Self { repo: TicketRepositorySeaorm::new(db) }
    }

    pub async fn list(&self, branch_id: Option<i64>, status: Option<&str>, priority: Option<&str>, category: Option<&str>, assigned_to: Option<i64>, customer_id: Option<i64>, page: i64, per_page: i64) -> Result<(Vec<TicketResponse>, i64), AppError> {
        let (tickets, total) = self.repo.list(branch_id, status, priority, category, assigned_to, customer_id, page, per_page).await?;
        let responses = tickets.into_iter().map(|t| TicketResponse {
            id: t.id, ticket_number: t.ticket_number, branch_id: t.branch_id,
            customer_id: t.customer_id, subscription_id: t.subscription_id,
            created_by: t.created_by, assigned_to: t.assigned_to, escalated_to: t.escalated_to,
            category: t.category, subcategory: t.subcategory, priority: t.priority, status: t.status,
            subject: t.subject, description: t.description, source: t.source,
            resolution_notes: t.resolution_notes, reopen_count: t.reopen_count,
            satisfaction_rating: t.satisfaction_rating, satisfaction_feedback: t.satisfaction_feedback,
            created_at: t.created_at.into(), updated_at: t.updated_at.into(),
        }).collect();
        Ok((responses, total))
    }

    pub async fn create(&self, branch_id: i64, created_by: i64, req: CreateTicketRequest) -> Result<TicketResponse, AppError> {
        // Generate ticket number
        let prefix = format!("TKT-{}", chrono::Utc::now().format("%Y%m"));
        let ticket_number = format!("{}-{:04}", prefix, rand::random::<u16>() % 10000);
        let t = self.repo.create(&ticket_number, branch_id, req.customer_id, req.subscription_id, created_by, &req.category, req.subcategory.as_deref(), &req.priority, &req.subject, &req.description, &req.source).await?;
        Ok(TicketResponse {
            id: t.id, ticket_number: t.ticket_number, branch_id: t.branch_id,
            customer_id: t.customer_id, subscription_id: t.subscription_id,
            created_by: t.created_by, assigned_to: t.assigned_to, escalated_to: t.escalated_to,
            category: t.category, subcategory: t.subcategory, priority: t.priority, status: t.status,
            subject: t.subject, description: t.description, source: t.source,
            resolution_notes: t.resolution_notes, reopen_count: t.reopen_count,
            satisfaction_rating: t.satisfaction_rating, satisfaction_feedback: t.satisfaction_feedback,
            created_at: t.created_at.into(), updated_at: t.updated_at.into(),
        })
    }

    pub async fn update_status(&self, id: i64, status: &str, resolution_notes: Option<&str>) -> Result<TicketResponse, AppError> {
        let t = self.repo.update_status(id, status, resolution_notes).await?;
        Ok(TicketResponse {
            id: t.id, ticket_number: t.ticket_number, branch_id: t.branch_id,
            customer_id: t.customer_id, subscription_id: t.subscription_id,
            created_by: t.created_by, assigned_to: t.assigned_to, escalated_to: t.escalated_to,
            category: t.category, subcategory: t.subcategory, priority: t.priority, status: t.status,
            subject: t.subject, description: t.description, source: t.source,
            resolution_notes: t.resolution_notes, reopen_count: t.reopen_count,
            satisfaction_rating: t.satisfaction_rating, satisfaction_feedback: t.satisfaction_feedback,
            created_at: t.created_at.into(), updated_at: t.updated_at.into(),
        })
    }

    pub async fn assign(&self, id: i64, assigned_to: i64) -> Result<TicketResponse, AppError> {
        let t = self.repo.assign(id, assigned_to).await?;
        Ok(TicketResponse {
            id: t.id, ticket_number: t.ticket_number, branch_id: t.branch_id,
            customer_id: t.customer_id, subscription_id: t.subscription_id,
            created_by: t.created_by, assigned_to: t.assigned_to, escalated_to: t.escalated_to,
            category: t.category, subcategory: t.subcategory, priority: t.priority, status: t.status,
            subject: t.subject, description: t.description, source: t.source,
            resolution_notes: t.resolution_notes, reopen_count: t.reopen_count,
            satisfaction_rating: t.satisfaction_rating, satisfaction_feedback: t.satisfaction_feedback,
            created_at: t.created_at.into(), updated_at: t.updated_at.into(),
        })
    }

    pub async fn add_comment(&self, ticket_id: i64, user_id: Option<i64>, is_customer: bool, comment: &str, is_internal: bool) -> Result<TicketCommentResponse, AppError> {
        let c = self.repo.add_comment(ticket_id, user_id, is_customer, comment, is_internal, None).await?;
        Ok(TicketCommentResponse {
            id: c.id, ticket_id: c.ticket_id, user_id: c.user_id, is_customer: c.is_customer,
            comment: c.comment, is_internal: c.is_internal,
            created_at: c.created_at.into(), updated_at: c.updated_at.into(),
        })
    }

    pub async fn list_comments(&self, ticket_id: i64) -> Result<Vec<TicketCommentResponse>, AppError> {
        let comments = self.repo.list_comments(ticket_id).await?;
        Ok(comments.into_iter().map(|c| TicketCommentResponse {
            id: c.id, ticket_id: c.ticket_id, user_id: c.user_id, is_customer: c.is_customer,
            comment: c.comment, is_internal: c.is_internal,
            created_at: c.created_at.into(), updated_at: c.updated_at.into(),
        }).collect())
    }
}
