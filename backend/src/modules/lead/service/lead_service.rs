use sqlx::PgPool;

use crate::common::errors::app_error::AppError;
use crate::modules::lead::repository::lead_repository::LeadRepository;
use crate::modules::lead::request::lead_request::*;
use crate::modules::lead::response::lead_response::*;
use crate::modules::lead::mapper::lead_mapper::*;

pub struct LeadService<'a> {
    repo: LeadRepository<'a>,
}

impl<'a> LeadService<'a> {
    pub fn new(pool: &'a PgPool) -> Self { Self { repo: LeadRepository::new(pool) } }

    pub async fn list_leads(&self, query: LeadQuery) -> Result<LeadListResponse, AppError> {
        let page = query.page.unwrap_or(1);
        let per_page = query.per_page.unwrap_or(20);
        let (leads, total) = self.repo.list(query.branch_id, query.status.as_deref(), query.source.as_deref(), query.assigned_to, page, per_page).await?;
        let responses: Vec<LeadResponse> = leads.iter().map(lead_to_response).collect();
        let total_pages = (total as f64 / per_page as f64).ceil() as i64;
        Ok(LeadListResponse { leads: responses, total, page, per_page, total_pages })
    }

    pub async fn get_lead(&self, id: i64) -> Result<LeadResponse, AppError> {
        let lead = self.repo.get_by_id(id).await?.ok_or_else(|| AppError::NotFound("Lead not found".into()))?;
        Ok(lead_to_response(&lead))
    }

    pub async fn create_lead(&self, req: CreateLeadRequest) -> Result<LeadResponse, AppError> {
        let lead = self.repo.create(req.branch_id, &req.name, &req.phone, req.email.as_deref(), &req.source, req.interested_plan_id, req.estimated_install_date, req.address.as_deref(), req.latitude, req.longitude, req.notes.as_deref()).await?;
        Ok(lead_to_response(&lead))
    }

    pub async fn update_lead(&self, id: i64, req: UpdateLeadRequest) -> Result<LeadResponse, AppError> {
        let lead = self.repo.update(id, req.name.as_deref(), req.phone.as_deref(), req.email.as_deref(), req.source.as_deref(), req.interested_plan_id, req.estimated_install_date, req.address.as_deref(), req.latitude, req.longitude, req.notes.as_deref()).await.map_err(|_| AppError::NotFound("Lead not found".into()))?;
        Ok(lead_to_response(&lead))
    }

    pub async fn update_status(&self, id: i64, req: LeadStatusRequest) -> Result<LeadResponse, AppError> {
        let lead = self.repo.update_status(id, &req.status, req.lost_reason.as_deref()).await.map_err(|_| AppError::NotFound("Lead not found".into()))?;
        Ok(lead_to_response(&lead))
    }

    pub async fn assign_lead(&self, id: i64, req: AssignLeadRequest) -> Result<LeadResponse, AppError> {
        let lead = self.repo.assign(id, req.assigned_to).await.map_err(|_| AppError::NotFound("Lead not found".into()))?;
        Ok(lead_to_response(&lead))
    }

    pub async fn add_activity(&self, id: i64, user_id: i64, req: AddActivityRequest) -> Result<LeadActivityResponse, AppError> {
        let _ = self.repo.get_by_id(id).await?.ok_or_else(|| AppError::NotFound("Lead not found".into()))?;
        let activity = self.repo.add_activity(id, &req.activity_type, &req.description, user_id, req.scheduled_at).await?;
        Ok(activity_to_response(&activity))
    }

    pub async fn get_activities(&self, id: i64) -> Result<Vec<LeadActivityResponse>, AppError> {
        let _ = self.repo.get_by_id(id).await?.ok_or_else(|| AppError::NotFound("Lead not found".into()))?;
        let activities = self.repo.list_activities(id).await?;
        Ok(activities.iter().map(activity_to_response).collect())
    }

    pub async fn convert_lead(&self, id: i64, _req: ConvertLeadRequest) -> Result<LeadResponse, AppError> {
        let lead = self.repo.get_by_id(id).await?.ok_or_else(|| AppError::NotFound("Lead not found".into()))?;
        if lead.status == "converted" { return Err(AppError::Conflict("Lead already converted".into())); }
        let customer_id = 1i64; // TODO: integrate with customer service
        let lead = self.repo.convert(id, customer_id).await?;
        Ok(lead_to_response(&lead))
    }

    pub async fn delete_lead(&self, id: i64) -> Result<MessageResponse, AppError> {
        if !self.repo.delete(id).await? { return Err(AppError::NotFound("Lead not found".into())); }
        Ok(MessageResponse { message: "Lead deleted successfully".into() })
    }

    pub async fn get_pipeline(&self) -> Result<LeadPipelineResponse, AppError> {
        let rows = self.repo.get_pipeline_counts().await?;
        let mut pipeline = LeadPipelineResponse { new: 0, contacted: 0, interested: 0, surveyed: 0, quoted: 0, converted: 0, lost: 0 };
        for (status, count) in rows {
            match status.as_str() {
                "new" => pipeline.new = count, "contacted" => pipeline.contacted = count,
                "interested" => pipeline.interested = count, "surveyed" => pipeline.surveyed = count,
                "quoted" => pipeline.quoted = count, "converted" => pipeline.converted = count,
                "lost" => pipeline.lost = count, _ => {}
            }
        }
        Ok(pipeline)
    }

    pub async fn get_stats(&self) -> Result<LeadStatsResponse, AppError> {
        let (total_leads, converted_this_month) = self.repo.get_stats().await?;
        let conversion_rate = if total_leads > 0 { (converted_this_month as f64 / total_leads as f64) * 100.0 } else { 0.0 };
        let by_source: Vec<SourceCount> = self.repo.get_source_counts().await?.into_iter().map(|(s, c)| SourceCount { source: s, count: c }).collect();
        let by_status: Vec<StatusCount> = self.repo.get_status_counts().await?.into_iter().map(|(s, c)| StatusCount { status: s, count: c }).collect();
        Ok(LeadStatsResponse { total_leads, converted_this_month, conversion_rate, by_source, by_status })
    }
}
