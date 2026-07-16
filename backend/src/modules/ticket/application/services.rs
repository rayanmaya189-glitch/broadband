use crate::modules::ticket::domain::entities::ticket_comment::Column as TicketCommentColumn;
use crate::modules::ticket::domain::entities::{
    Ticket, TicketActiveModel, TicketColumn, TicketComment, TicketCommentActiveModel,
};
use crate::shared::errors::AppError;
use sea_orm::{ActiveModelTrait, ColumnTrait, DatabaseConnection, EntityTrait, QueryFilter, Set};

pub struct TicketService;

impl TicketService {
    pub async fn list_tickets(
        db: &DatabaseConnection,
        branch_id: Option<i64>,
    ) -> Result<Vec<crate::modules::ticket::domain::entities::ticket::Model>, AppError> {
        let mut query = Ticket::find();
        if let Some(bid) = branch_id {
            query = query.filter(TicketColumn::BranchId.eq(bid));
        }
        Ok(query.all(db).await?)
    }

    pub async fn get_ticket(
        db: &DatabaseConnection,
        id: i64,
    ) -> Result<crate::modules::ticket::domain::entities::ticket::Model, AppError> {
        Ticket::find_by_id(id)
            .one(db)
            .await?
            .ok_or_else(|| AppError::NotFound(format!("Ticket {} not found", id)))
    }

    pub async fn create_ticket(
        db: &DatabaseConnection,
        branch_id: i64,
        created_by: i64,
        category: String,
        priority: String,
        subject: String,
        description: String,
        source: String,
        customer_id: Option<i64>,
    ) -> Result<crate::modules::ticket::domain::entities::ticket::Model, AppError> {
        let now = chrono::Utc::now();
        let ticket_number = format!(
            "TKT-{}-{:04}",
            now.format("%Y%m"),
            now.timestamp_millis() % 10000
        );
        let ticket = TicketActiveModel {
            ticket_number: Set(ticket_number),
            branch_id: Set(branch_id),
            created_by: Set(created_by),
            category: Set(category),
            priority: Set(priority),
            subject: Set(subject),
            description: Set(description),
            source: Set(source),
            status: Set("open".to_string()),
            customer_id: Set(customer_id),
            created_at: Set(now),
            updated_at: Set(now),
            ..Default::default()
        };
        Ok(ticket.insert(db).await?)
    }

    pub async fn assign_ticket(
        db: &DatabaseConnection,
        id: i64,
        assigned_to: i64,
    ) -> Result<crate::modules::ticket::domain::entities::ticket::Model, AppError> {
        let ticket = Self::get_ticket(db, id).await?;
        let mut active: TicketActiveModel = ticket.into();
        active.assigned_to = Set(Some(assigned_to));
        active.status = Set("assigned".to_string());
        active.updated_at = Set(chrono::Utc::now());
        Ok(active.update(db).await?)
    }

    pub async fn resolve_ticket(
        db: &DatabaseConnection,
        id: i64,
        _resolved_by: i64,
        resolution_notes: Option<String>,
    ) -> Result<crate::modules::ticket::domain::entities::ticket::Model, AppError> {
        let ticket = Self::get_ticket(db, id).await?;
        let mut active: TicketActiveModel = ticket.into();
        active.status = Set("resolved".to_string());
        active.resolved_at = Set(Some(chrono::Utc::now()));
        active.resolution_notes = Set(resolution_notes);
        active.updated_at = Set(chrono::Utc::now());
        Ok(active.update(db).await?)
    }

    pub async fn add_comment(
        db: &DatabaseConnection,
        ticket_id: i64,
        user_id: i64,
        comment: String,
    ) -> Result<crate::modules::ticket::domain::entities::ticket_comment::Model, AppError> {
        let now = chrono::Utc::now();
        let c = TicketCommentActiveModel {
            ticket_id: Set(ticket_id),
            user_id: Set(Some(user_id)),
            comment: Set(comment),
            is_customer: Set(false),
            is_internal: Set(false),
            created_at: Set(now),
            updated_at: Set(now),
            ..Default::default()
        };
        Ok(c.insert(db).await?)
    }

    pub async fn get_comments(
        db: &DatabaseConnection,
        ticket_id: i64,
    ) -> Result<Vec<crate::modules::ticket::domain::entities::ticket_comment::Model>, AppError>
    {
        Ok(TicketComment::find()
            .filter(TicketCommentColumn::TicketId.eq(ticket_id))
            .all(db)
            .await?)
    }
}


