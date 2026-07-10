//! SeaORM-based repository for the Ticket domain.

use sea_orm::{
    ActiveModelTrait, ColumnTrait, DatabaseConnection, EntityTrait, IntoActiveModel,
    PaginatorTrait, QueryFilter, QueryOrder, Set,
};

use crate::common::errors::app_error::AppError;
use crate::modules::ticket::model::ticket_entity::{self, Model as TicketModel};
use crate::modules::ticket::model::ticket_comment_entity::{self, Model as TicketCommentModel};
use crate::modules::ticket::model::ticket_escalation_entity::{self, Model as TicketEscalationModel};
use crate::modules::ticket::model::ticket_status_history_entity::{self, Model as TicketStatusHistoryModel};

pub struct TicketRepositorySeaorm<'a> {
    db: &'a DatabaseConnection,
}

impl<'a> TicketRepositorySeaorm<'a> {
    pub fn new(db: &'a DatabaseConnection) -> Self { Self { db } }

    pub async fn list(&self, branch_id: Option<i64>, status: Option<&str>, priority: Option<&str>, category: Option<&str>, assigned_to: Option<i64>, customer_id: Option<i64>, page: i64, per_page: i64) -> Result<(Vec<TicketModel>, i64), AppError> {
        let page_size = per_page as u64;
        let page_num = if per_page > 0 { ((page - 1).max(0) as u64 * page_size) / page_size } else { 0 };
        let mut select = ticket_entity::Entity::find();
        if let Some(bid) = branch_id { select = select.filter(ticket_entity::Column::BranchId.eq(bid)); }
        if let Some(s) = status { select = select.filter(ticket_entity::Column::Status.eq(s)); }
        if let Some(p) = priority { select = select.filter(ticket_entity::Column::Priority.eq(p)); }
        if let Some(c) = category { select = select.filter(ticket_entity::Column::Category.eq(c)); }
        if let Some(at) = assigned_to { select = select.filter(ticket_entity::Column::AssignedTo.eq(at)); }
        if let Some(cid) = customer_id { select = select.filter(ticket_entity::Column::CustomerId.eq(cid)); }
        let total = select.clone().count(self.db).await?;
        let tickets = select.order_by_desc(ticket_entity::Column::CreatedAt).paginate(self.db, page_size).fetch_page(page_num).await?;
        Ok((tickets, total as i64))
    }

    pub async fn get_by_id(&self, id: i64) -> Result<Option<TicketModel>, AppError> {
        Ok(ticket_entity::Entity::find_by_id(id).one(self.db).await?)
    }

    pub async fn create(&self, ticket_number: &str, branch_id: i64, customer_id: Option<i64>, subscription_id: Option<i64>, created_by: i64, category: &str, subcategory: Option<&str>, priority: &str, subject: &str, description: &str, source: &str) -> Result<TicketModel, AppError> {
        let now = chrono::Utc::now();
        let active = ticket_entity::ActiveModel {
            ticket_number: Set(ticket_number.to_owned()),
            branch_id: Set(branch_id),
            customer_id: Set(customer_id),
            subscription_id: Set(subscription_id),
            created_by: Set(created_by),
            category: Set(category.to_owned()),
            subcategory: Set(subcategory.map(|s| s.to_owned())),
            priority: Set(priority.to_owned()),
            status: Set("open".to_owned()),
            subject: Set(subject.to_owned()),
            description: Set(description.to_owned()),
            source: Set(source.to_owned()),
            created_at: Set(now.into()),
            updated_at: Set(now.into()),
            ..Default::default()
        };
        Ok(active.insert(self.db).await?)
    }

    pub async fn update_status(&self, id: i64, status: &str, resolution_notes: Option<&str>) -> Result<TicketModel, AppError> {
        let existing = ticket_entity::Entity::find_by_id(id).one(self.db).await?
            .ok_or_else(|| AppError::NotFound("Ticket not found".into()))?;
        let mut active = existing.into_active_model();
        active.status = Set(status.to_owned());
        if let Some(rn) = resolution_notes { active.resolution_notes = Set(Some(rn.to_owned())); }
        if status == "resolved" { active.resolved_at = Set(Some(chrono::Utc::now().into())); }
        if status == "closed" { active.closed_at = Set(Some(chrono::Utc::now().into())); }
        active.updated_at = Set(chrono::Utc::now().into());
        Ok(active.update(self.db).await?)
    }

    pub async fn assign(&self, id: i64, assigned_to: i64) -> Result<TicketModel, AppError> {
        let existing = ticket_entity::Entity::find_by_id(id).one(self.db).await?
            .ok_or_else(|| AppError::NotFound("Ticket not found".into()))?;
        let mut active = existing.into_active_model();
        active.assigned_to = Set(Some(assigned_to));
        active.status = Set("assigned".to_owned());
        active.updated_at = Set(chrono::Utc::now().into());
        Ok(active.update(self.db).await?)
    }

    pub async fn escalate(&self, id: i64, escalated_to: i64, new_priority: Option<&str>) -> Result<TicketModel, AppError> {
        let existing = ticket_entity::Entity::find_by_id(id).one(self.db).await?
            .ok_or_else(|| AppError::NotFound("Ticket not found".into()))?;
        let mut active = existing.into_active_model();
        active.escalated_to = Set(Some(escalated_to));
        if let Some(p) = new_priority { active.priority = Set(p.to_owned()); }
        active.status = Set("escalated".to_owned());
        active.updated_at = Set(chrono::Utc::now().into());
        Ok(active.update(self.db).await?)
    }

    pub async fn increment_reopen(&self, id: i64) -> Result<TicketModel, AppError> {
        let existing = ticket_entity::Entity::find_by_id(id).one(self.db).await?
            .ok_or_else(|| AppError::NotFound("Ticket not found".into()))?;
        let mut active = existing.into_active_model();
        active.reopen_count = Set(active.reopen_count.clone().unwrap_or(0) + 1);
        active.status = Set("open".to_owned());
        active.updated_at = Set(chrono::Utc::now().into());
        Ok(active.update(self.db).await?)
    }

    pub async fn set_feedback(&self, id: i64, rating: Option<i32>, feedback: Option<&str>) -> Result<TicketModel, AppError> {
        let existing = ticket_entity::Entity::find_by_id(id).one(self.db).await?
            .ok_or_else(|| AppError::NotFound("Ticket not found".into()))?;
        let mut active = existing.into_active_model();
        active.satisfaction_rating = Set(rating);
        active.satisfaction_feedback = Set(feedback.map(|s| s.to_owned()));
        active.updated_at = Set(chrono::Utc::now().into());
        Ok(active.update(self.db).await?)
    }

    pub async fn get_my_assignments(&self, assigned_to: i64, page: i64, per_page: i64) -> Result<(Vec<TicketModel>, i64), AppError> {
        let page_size = per_page as u64;
        let page_num = if per_page > 0 { ((page - 1).max(0) as u64 * page_size) / page_size } else { 0 };
        let select = ticket_entity::Entity::find()
            .filter(ticket_entity::Column::AssignedTo.eq(assigned_to))
            .filter(ticket_entity::Column::Status.is_not_in(vec!["resolved", "closed"]));
        let total = select.clone().count(self.db).await?;
        let tickets = select.order_by_asc(ticket_entity::Column::CreatedAt).paginate(self.db, page_size).fetch_page(page_num).await?;
        Ok((tickets, total as i64))
    }

    pub async fn delete(&self, id: i64) -> Result<bool, AppError> {
        let result = ticket_entity::Entity::delete_by_id(id).exec(self.db).await?;
        Ok(result.rows_affected > 0)
    }

    // ──── Comments ────

    pub async fn list_comments(&self, ticket_id: i64) -> Result<Vec<TicketCommentModel>, AppError> {
        Ok(ticket_comment_entity::Entity::find()
            .filter(ticket_comment_entity::Column::TicketId.eq(ticket_id))
            .order_by_asc(ticket_comment_entity::Column::CreatedAt)
            .all(self.db).await?)
    }

    pub async fn add_comment(&self, ticket_id: i64, user_id: Option<i64>, is_customer: bool, comment: &str, is_internal: bool, attachments: Option<serde_json::Value>) -> Result<TicketCommentModel, AppError> {
        let now = chrono::Utc::now();
        let active = ticket_comment_entity::ActiveModel {
            ticket_id: Set(ticket_id),
            user_id: Set(user_id),
            is_customer: Set(is_customer),
            comment: Set(comment.to_owned()),
            is_internal: Set(is_internal),
            attachments: Set(attachments),
            created_at: Set(now.into()),
            updated_at: Set(now.into()),
            ..Default::default()
        };
        Ok(active.insert(self.db).await?)
    }

    // ──── Escalations ────

    pub async fn create_escalation(&self, ticket_id: i64, from_user_id: i64, to_user_id: i64, from_priority: Option<&str>, to_priority: Option<&str>, reason: &str) -> Result<TicketEscalationModel, AppError> {
        let now = chrono::Utc::now();
        let active = ticket_escalation_entity::ActiveModel {
            ticket_id: Set(ticket_id),
            from_user_id: Set(from_user_id),
            to_user_id: Set(to_user_id),
            from_priority: Set(from_priority.map(|s| s.to_owned())),
            to_priority: Set(to_priority.map(|s| s.to_owned())),
            reason: Set(reason.to_owned()),
            escalated_at: Set(now.into()),
            created_at: Set(now.into()),
            ..Default::default()
        };
        Ok(active.insert(self.db).await?)
    }

    pub async fn list_escalations(&self, ticket_id: i64) -> Result<Vec<TicketEscalationModel>, AppError> {
        Ok(ticket_escalation_entity::Entity::find()
            .filter(ticket_escalation_entity::Column::TicketId.eq(ticket_id))
            .order_by_asc(ticket_escalation_entity::Column::EscalatedAt)
            .all(self.db).await?)
    }

    // ──── Status History ────

    pub async fn create_status_history(&self, ticket_id: i64, old_status: Option<&str>, new_status: &str, changed_by: i64, reason: Option<&str>) -> Result<TicketStatusHistoryModel, AppError> {
        let now = chrono::Utc::now();
        let active = ticket_status_history_entity::ActiveModel {
            ticket_id: Set(ticket_id),
            old_status: Set(old_status.map(|s| s.to_owned())),
            new_status: Set(new_status.to_owned()),
            changed_by: Set(changed_by),
            reason: Set(reason.map(|s| s.to_owned())),
            created_at: Set(now.into()),
            ..Default::default()
        };
        Ok(active.insert(self.db).await?)
    }

    pub async fn list_status_history(&self, ticket_id: i64) -> Result<Vec<TicketStatusHistoryModel>, AppError> {
        Ok(ticket_status_history_entity::Entity::find()
            .filter(ticket_status_history_entity::Column::TicketId.eq(ticket_id))
            .order_by_asc(ticket_status_history_entity::Column::CreatedAt)
            .all(self.db).await?)
    }
}
