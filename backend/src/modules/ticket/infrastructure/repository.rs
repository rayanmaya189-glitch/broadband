use sea_orm::{
    ActiveModelTrait, ColumnTrait, DatabaseConnection, EntityTrait, PaginatorTrait, QueryFilter,
    QueryOrder, QuerySelect, Set,
};

use crate::modules::ticket::domain::entities::{ticket, ticket_comment};
use crate::shared::errors::AppError;

pub struct TicketRepository<'a> {
    db: &'a DatabaseConnection,
}

impl<'a> TicketRepository<'a> {
    pub fn new(db: &'a DatabaseConnection) -> Self {
        Self { db }
    }

    pub async fn find_by_id(&self, id: i64) -> Result<Option<ticket::Model>, AppError> {
        Ok(ticket::Entity::find_by_id(id).one(self.db).await?)
    }

    pub async fn find_by_number(
        &self,
        ticket_number: &str,
    ) -> Result<Option<ticket::Model>, AppError> {
        Ok(ticket::Entity::find()
            .filter(ticket::Column::TicketNumber.eq(ticket_number))
            .one(self.db)
            .await?)
    }

    pub async fn list_tickets(
        &self,
        branch_id: Option<i64>,
        status: Option<&str>,
        priority: Option<&str>,
        assigned_to: Option<i64>,
        limit: i64,
        offset: i64,
    ) -> Result<Vec<ticket::Model>, AppError> {
        let mut query = ticket::Entity::find();
        if let Some(bid) = branch_id {
            query = query.filter(ticket::Column::BranchId.eq(bid));
        }
        if let Some(s) = status {
            query = query.filter(ticket::Column::Status.eq(s));
        }
        if let Some(p) = priority {
            query = query.filter(ticket::Column::Priority.eq(p));
        }
        if let Some(uid) = assigned_to {
            query = query.filter(ticket::Column::AssignedTo.eq(uid));
        }
        Ok(query
            .order_by_desc(ticket::Column::CreatedAt)
            .limit(limit as u64)
            .offset(offset as u64)
            .all(self.db)
            .await?)
    }

    pub async fn count_tickets(&self, branch_id: Option<i64>) -> Result<i64, AppError> {
        let mut query = ticket::Entity::find();
        if let Some(bid) = branch_id {
            query = query.filter(ticket::Column::BranchId.eq(bid));
        }
        Ok(query.count(self.db).await? as i64)
    }

    pub async fn create_ticket(
        &self,
        ticket_number: String,
        branch_id: i64,
        customer_id: Option<i64>,
        created_by: i64,
        category: String,
        subcategory: Option<String>,
        priority: String,
        subject: String,
        description: String,
        source: String,
    ) -> Result<ticket::Model, AppError> {
        let model = ticket::ActiveModel {
            ticket_number: Set(ticket_number),
            branch_id: Set(branch_id),
            customer_id: Set(customer_id),
            created_by: Set(created_by),
            category: Set(category),
            subcategory: Set(subcategory),
            priority: Set(priority),
            status: Set("open".to_string()),
            subject: Set(subject),
            description: Set(description),
            source: Set(source),
            ..Default::default()
        };
        Ok(model.insert(self.db).await?)
    }

    pub async fn update_ticket(
        &self,
        model: ticket::Model,
        status: Option<String>,
        priority: Option<String>,
        assigned_to: Option<i64>,
        resolution_notes: Option<String>,
    ) -> Result<ticket::Model, AppError> {
        let mut active: ticket::ActiveModel = model.into();
        if let Some(v) = status {
            active.status = Set(v);
        }
        if let Some(v) = priority {
            active.priority = Set(v);
        }
        if let Some(v) = assigned_to {
            active.assigned_to = Set(Some(v));
        }
        if let Some(v) = resolution_notes {
            active.resolution_notes = Set(Some(v));
        }
        active.updated_at = Set(chrono::Utc::now());
        Ok(active.update(self.db).await?)
    }

    pub async fn add_comment(
        &self,
        ticket_id: i64,
        user_id: Option<i64>,
        is_customer: bool,
        comment: String,
        is_internal: bool,
    ) -> Result<ticket_comment::Model, AppError> {
        let now = chrono::Utc::now();
        let model = ticket_comment::ActiveModel {
            ticket_id: Set(ticket_id),
            user_id: Set(user_id),
            is_customer: Set(is_customer),
            comment: Set(comment),
            is_internal: Set(is_internal),
            created_at: Set(now),
            updated_at: Set(now),
            ..Default::default()
        };
        Ok(model.insert(self.db).await?)
    }

    pub async fn list_comments(
        &self,
        ticket_id: i64,
    ) -> Result<Vec<ticket_comment::Model>, AppError> {
        Ok(ticket_comment::Entity::find()
            .filter(ticket_comment::Column::TicketId.eq(ticket_id))
            .order_by_asc(ticket_comment::Column::CreatedAt)
            .all(self.db)
            .await?)
    }
}
