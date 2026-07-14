use sea_orm::DatabaseConnection;
use crate::common::errors::app_error::AppError;
use crate::modules::crm::repository::interaction_repository::CrmInteractionRepository;
use crate::modules::crm::repository::note_repository::NoteRepository;
use crate::modules::crm::repository::tag_repository::TagRepository;
use crate::modules::crm::repository::segment_repository::SegmentRepository;
use crate::modules::crm::request::crm_request::*;
use crate::modules::crm::response::crm_response::*;

pub struct CrmService<'a> {
    db: &'a DatabaseConnection,
}

impl<'a> CrmService<'a> {
    pub fn new(db: &'a DatabaseConnection) -> Self { Self { db } }

    pub async fn list_interactions(&self, customer_id: Option<i64>, branch_id: Option<i64>, interaction_type: Option<&str>, page: i64, per_page: i64) -> Result<(Vec<InteractionResponse>, i64), AppError> {
        let repo = CrmInteractionRepository::new(self.db);
        let (items, total) = repo.list(customer_id, branch_id, interaction_type, page, per_page).await?;
        Ok((items.into_iter().map(|i| InteractionResponse {
            id: i.id, customer_id: i.customer_id, branch_id: i.branch_id, user_id: i.user_id,
            interaction_type: i.interaction_type, subject: i.subject, body: i.body, channel: i.channel,
            duration_seconds: i.duration_seconds, sentiment: i.sentiment,
            follow_up_date: i.follow_up_date, follow_up_done: i.follow_up_done,
            created_at: i.created_at.into(), updated_at: i.updated_at.into(),
        }).collect(), total))
    }

    pub async fn create_interaction(&self, customer_id: i64, branch_id: i64, user_id: i64, req: CreateInteractionRequest) -> Result<InteractionResponse, AppError> {
        let repo = CrmInteractionRepository::new(self.db);
        let i = repo.create(customer_id, branch_id, user_id, &req.interaction_type, &req.subject, req.body.as_deref(), &req.channel, req.duration_seconds, req.sentiment.as_deref(), req.follow_up_date).await?;
        Ok(InteractionResponse {
            id: i.id, customer_id: i.customer_id, branch_id: i.branch_id, user_id: i.user_id,
            interaction_type: i.interaction_type, subject: i.subject, body: i.body, channel: i.channel,
            duration_seconds: i.duration_seconds, sentiment: i.sentiment,
            follow_up_date: i.follow_up_date, follow_up_done: i.follow_up_done,
            created_at: i.created_at.into(), updated_at: i.updated_at.into(),
        })
    }

    pub async fn list_notes(&self, customer_id: i64, page: i64, per_page: i64) -> Result<(Vec<NoteResponse>, i64), AppError> {
        let repo = NoteRepository::new(self.db);
        let (items, total) = repo.list(customer_id, page, per_page).await?;
        Ok((items.into_iter().map(|n| NoteResponse {
            id: n.id, customer_id: n.customer_id, branch_id: n.branch_id, user_id: n.user_id,
            title: n.title, content: n.content, priority: n.priority, is_pinned: n.is_pinned,
            expires_at: n.expires_at.map(|v| v.into()),
            created_at: n.created_at.into(), updated_at: n.updated_at.into(),
        }).collect(), total))
    }

    pub async fn create_note(&self, customer_id: i64, branch_id: i64, user_id: i64, req: CreateNoteRequest) -> Result<NoteResponse, AppError> {
        let repo = NoteRepository::new(self.db);
        let n = repo.create(customer_id, branch_id, user_id, &req.title, &req.content, &req.priority, req.is_pinned.unwrap_or(false), req.expires_at).await?;
        Ok(NoteResponse {
            id: n.id, customer_id: n.customer_id, branch_id: n.branch_id, user_id: n.user_id,
            title: n.title, content: n.content, priority: n.priority, is_pinned: n.is_pinned,
            expires_at: n.expires_at.map(|v| v.into()),
            created_at: n.created_at.into(), updated_at: n.updated_at.into(),
        })
    }

    pub async fn list_tags(&self, branch_id: Option<i64>, category: Option<&str>) -> Result<Vec<TagResponse>, AppError> {
        let repo = TagRepository::new(self.db);
        let items = repo.list(branch_id, category).await?;
        Ok(items.into_iter().map(|t| TagResponse {
            id: t.id, name: t.name, color: t.color, category: t.category, usage_count: t.usage_count,
        }).collect())
    }

    pub async fn create_tag(&self, branch_id: Option<i64>, req: CreateTagRequest) -> Result<TagResponse, AppError> {
        let repo = TagRepository::new(self.db);
        let t = repo.create(branch_id, &req.name, req.color.as_deref(), &req.category).await?;
        Ok(TagResponse { id: t.id, name: t.name, color: t.color, category: t.category, usage_count: t.usage_count })
    }

    pub async fn assign_tag(&self, customer_id: i64, tag_id: i64) -> Result<MessageResponse, AppError> {
        let repo = TagRepository::new(self.db);
        repo.assign_to_customer(customer_id, tag_id).await?;
        Ok(MessageResponse { message: "Tag assigned".into() })
    }

    pub async fn list_segments(&self, branch_id: Option<i64>) -> Result<Vec<SegmentResponse>, AppError> {
        let repo = SegmentRepository::new(self.db);
        let items = repo.list(branch_id).await?;
        Ok(items.into_iter().map(|s| SegmentResponse {
            id: s.id, name: s.name, description: s.description, criteria: s.criteria,
            customer_count: s.customer_count, is_dynamic: s.is_dynamic,
        }).collect())
    }

    pub async fn create_segment(&self, branch_id: Option<i64>, req: CreateSegmentRequest) -> Result<SegmentResponse, AppError> {
        let repo = SegmentRepository::new(self.db);
        let s = repo.create(branch_id, &req.name, req.description.as_deref(), req.criteria, req.is_dynamic.unwrap_or(false)).await?;
        Ok(SegmentResponse { id: s.id, name: s.name, description: s.description, criteria: s.criteria, customer_count: s.customer_count, is_dynamic: s.is_dynamic })
    }
}
