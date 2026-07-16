use async_trait::async_trait;
use sea_orm::{DatabaseConnection};
use crate::shared::errors::AppError;

pub type TicketModel = crate::modules::ticket::domain::entities::ticket::Model;
pub type TicketCommentModel = crate::modules::ticket::domain::entities::ticket_comment::Model;

#[async_trait]
pub trait TicketServiceTrait: Send + Sync {
    async fn list_tickets(
        &self,
        db: &DatabaseConnection,
        branch_id: Option<i64>,
    ) -> Result<Vec<TicketModel>, AppError>;

    async fn get_ticket(
        &self,
        db: &DatabaseConnection,
        id: i64,
    ) -> Result<TicketModel, AppError>;

    async fn create_ticket(
        &self,
        db: &DatabaseConnection,
        branch_id: i64,
        created_by: i64,
        category: String,
        priority: String,
        subject: String,
        description: String,
        source: String,
        customer_id: Option<i64>,
    ) -> Result<TicketModel, AppError>;

    async fn assign_ticket(
        &self,
        db: &DatabaseConnection,
        id: i64,
        assigned_to: i64,
    ) -> Result<TicketModel, AppError>;

    async fn resolve_ticket(
        &self,
        db: &DatabaseConnection,
        id: i64,
        resolved_by: i64,
        resolution_notes: Option<String>,
    ) -> Result<TicketModel, AppError>;

    async fn add_comment(
        &self,
        db: &DatabaseConnection,
        ticket_id: i64,
        user_id: i64,
        comment: String,
    ) -> Result<TicketCommentModel, AppError>;

    async fn get_comments(
        &self,
        db: &DatabaseConnection,
        ticket_id: i64,
    ) -> Result<Vec<TicketCommentModel>, AppError>;
}
