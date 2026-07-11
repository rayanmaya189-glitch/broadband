//! SeaORM-based service for the Lead domain.

use sea_orm::DatabaseConnection;

use crate::common::errors::app_error::AppError;
use crate::modules::lead::repository::lead_repository::LeadRepository;
use crate::modules::lead::request::lead_request::*;
use crate::modules::lead::response::lead_response::*;

pub struct LeadService<'a> {
    repo: LeadRepository<'a>,
}

impl<'a> LeadService<'a> {
    pub fn new(db: &'a DatabaseConnection) -> Self {
        Self { repo: LeadRepository::new(db) }
    }

    fn to_response(l: crate::modules::lead::model::lead_entity::Model) -> LeadResponse {
        LeadResponse {
            id: l.id, branch_id: l.branch_id, assigned_to: l.assigned_to, name: l.name,
            phone: l.phone, email: l.email, source: l.source, status: l.status,
            interested_plan_id: l.interested_plan_id, estimated_install_date: l.estimated_install_date,
            address: l.address, latitude: l.latitude, longitude: l.longitude,
            lost_reason: l.lost_reason, notes: l.notes, converted_customer_id: l.converted_customer_id,
            converted_at: l.converted_at.map(|v| v.into()),
            created_at: l.created_at.into(), updated_at: l.updated_at.into(),
            assigned_to_name: None, branch_name: None,
        }
    }

    pub async fn list(&self, branch_id: Option<i64>, status: Option<&str>, source: Option<&str>, assigned_to: Option<i64>, page: i64, per_page: i64) -> Result<(Vec<LeadResponse>, i64), AppError> {
        let (leads, total) = self.repo.list(branch_id, status, source, assigned_to, page, per_page).await?;
        Ok((leads.into_iter().map(Self::to_response).collect(), total))
    }

    pub async fn get_by_id(&self, id: i64) -> Result<LeadResponse, AppError> {
        let l = self.repo.get_by_id(id).await?.ok_or_else(|| AppError::NotFound("Lead not found".into()))?;
        Ok(Self::to_response(l))
    }

    pub async fn create(&self, branch_id: i64, req: CreateLeadRequest) -> Result<LeadResponse, AppError> {
        let l = self.repo.create(branch_id, &req.name, &req.phone, req.email.as_deref(), &req.source, req.interested_plan_id, req.estimated_install_date, req.address.as_deref(), req.latitude, req.longitude, req.notes.as_deref()).await?;
        Ok(Self::to_response(l))
    }

    pub async fn update(&self, id: i64, req: UpdateLeadRequest) -> Result<LeadResponse, AppError> {
        let l = self.repo.update(id, req.name.as_deref(), req.phone.as_deref(), req.email.as_deref(), req.source.as_deref(), req.interested_plan_id, req.estimated_install_date, req.address.as_deref(), req.latitude, req.longitude, req.notes.as_deref()).await?;
        Ok(Self::to_response(l))
    }

    pub async fn update_status(&self, id: i64, status: &str, lost_reason: Option<&str>) -> Result<LeadResponse, AppError> {
        let l = self.repo.update_status(id, status, lost_reason).await?;
        Ok(Self::to_response(l))
    }

    pub async fn assign(&self, id: i64, assigned_to: i64) -> Result<LeadResponse, AppError> {
        let l = self.repo.assign(id, assigned_to).await?;
        Ok(Self::to_response(l))
    }

    pub async fn convert(&self, id: i64, customer_id: i64) -> Result<LeadResponse, AppError> {
        let l = self.repo.convert(id, customer_id).await?;
        Ok(Self::to_response(l))
    }

    pub async fn delete(&self, id: i64) -> Result<MessageResponse, AppError> {
        if !self.repo.delete(id).await? { return Err(AppError::NotFound("Lead not found".into())); }
        Ok(MessageResponse { message: "Lead deleted".into() })
    }

    pub async fn list_activities(&self, lead_id: i64) -> Result<Vec<LeadActivityResponse>, AppError> {
        let activities = self.repo.list_activities(lead_id).await?;
        Ok(activities.into_iter().map(|a| LeadActivityResponse {
            id: a.id, lead_id: a.lead_id, activity_type: a.activity_type,
            description: a.description, performed_by: a.performed_by,
            scheduled_at: a.scheduled_at.map(|v| v.into()),
            completed_at: None, created_at: a.created_at.into(),
            performer_name: None,
        }).collect())
    }

    pub async fn add_activity(&self, lead_id: i64, req: AddLeadActivityRequest) -> Result<LeadActivityResponse, AppError> {
        let a = self.repo.add_activity(lead_id, &req.activity_type, &req.description, 0, req.scheduled_at).await?;
        Ok(LeadActivityResponse {
            id: a.id, lead_id: a.lead_id, activity_type: a.activity_type,
            description: a.description, performed_by: a.performed_by,
            scheduled_at: a.scheduled_at.map(|v| v.into()),
            completed_at: None, created_at: a.created_at.into(),
            performer_name: None,
        })
    }

    pub async fn get_pipeline(&self) -> Result<LeadPipelineResponse, AppError> {
        let (leads, _) = self.repo.list(None, None, None, None, 1, 100000).await?;
        Ok(LeadPipelineResponse {
            new: leads.iter().filter(|l| l.status == "new").count() as i64,
            contacted: leads.iter().filter(|l| l.status == "contacted").count() as i64,
            interested: leads.iter().filter(|l| l.status == "interested").count() as i64,
            surveyed: leads.iter().filter(|l| l.status == "surveyed").count() as i64,
            quoted: leads.iter().filter(|l| l.status == "quoted").count() as i64,
            converted: leads.iter().filter(|l| l.status == "converted").count() as i64,
            lost: leads.iter().filter(|l| l.status == "lost").count() as i64,
        })
    }

    pub async fn get_stats(&self) -> Result<serde_json::Value, AppError> {
        let (leads, total) = self.repo.list(None, None, None, None, 1, 100000).await?;
        let converted_this_month = leads.iter().filter(|l| l.status == "converted").count() as i64;
        let conversion_rate = if total > 0 { converted_this_month as f64 / total as f64 * 100.0 } else { 0.0 };
        Ok(serde_json::json!({
            "total_leads": total,
            "converted_this_month": converted_this_month,
            "conversion_rate": conversion_rate,
        }))
    }
}
