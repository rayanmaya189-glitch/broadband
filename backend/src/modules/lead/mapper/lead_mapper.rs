use crate::modules::lead::model::lead_entity;
use crate::modules::lead::model::lead_activity_entity;
use crate::modules::lead::response::lead_response::{LeadResponse, LeadActivityResponse};

pub fn lead_to_response(lead: &lead_entity::Model) -> LeadResponse {
    LeadResponse {
        id: lead.id,
        branch_id: lead.branch_id,
        assigned_to: lead.assigned_to,
        name: lead.name.clone(),
        phone: lead.phone.clone(),
        email: lead.email.clone(),
        source: lead.source.clone(),
        status: lead.status.clone(),
        interested_plan_id: lead.interested_plan_id,
        estimated_install_date: lead.estimated_install_date,
        address: lead.address.clone(),
        latitude: lead.latitude,
        longitude: lead.longitude,
        lost_reason: lead.lost_reason.clone(),
        notes: lead.notes.clone(),
        converted_customer_id: lead.converted_customer_id,
        converted_at: lead.converted_at.map(|v| v.into()),
        created_at: lead.created_at.into(),
        updated_at: lead.updated_at.into(),
        assigned_to_name: None,
        branch_name: None,
    }
}

pub fn activity_to_response(activity: &lead_activity_entity::Model) -> LeadActivityResponse {
    LeadActivityResponse {
        id: activity.id,
        lead_id: activity.lead_id,
        activity_type: activity.activity_type.clone(),
        description: activity.description.clone(),
        performed_by: activity.performed_by,
        scheduled_at: activity.scheduled_at.map(|v| v.into()),
        completed_at: activity.completed_at.map(|v| v.into()),
        created_at: activity.created_at.into(),
        performer_name: None,
    }
}
