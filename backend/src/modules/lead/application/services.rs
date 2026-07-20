use crate::modules::lead::domain::entities::{
    Lead, LeadActiveModel, LeadActivity, LeadActivityActiveModel, LeadColumn,
};
use crate::shared::errors::AppError;
use sea_orm::{
    ActiveModelTrait, ColumnTrait, DatabaseConnection, EntityTrait, PaginatorTrait, QueryFilter,
    QueryOrder, Set,
};

pub struct LeadService;

impl LeadService {
    pub async fn list_leads(
        db: &DatabaseConnection,
        branch_id: Option<i64>,
        _page: u64,
        _limit: u64,
    ) -> Result<
        (
            Vec<crate::modules::lead::domain::entities::lead::Model>,
            u64,
        ),
        AppError,
    > {
        let mut query = Lead::find();
        if let Some(bid) = branch_id {
            query = query.filter(LeadColumn::BranchId.eq(bid));
        }
        let t = query.clone().count(db).await?;
        Ok((query.all(db).await?, t))
    }

    pub async fn create_lead(
        db: &DatabaseConnection,
        branch_id: i64,
        name: String,
        phone: String,
        email: Option<String>,
        source: String,
    ) -> Result<crate::modules::lead::domain::entities::lead::Model, AppError> {
        let now = chrono::Utc::now();
        let lead = LeadActiveModel {
            branch_id: Set(branch_id),
            name: Set(name),
            phone: Set(phone),
            email: Set(email),
            source: Set(source),
            status: Set("new".to_string()),
            created_at: Set(now),
            updated_at: Set(now),
            ..Default::default()
        };
        Ok(lead.insert(db).await?)
    }

    pub async fn get_lead(
        db: &DatabaseConnection,
        id: i64,
    ) -> Result<crate::modules::lead::domain::entities::lead::Model, AppError> {
        Lead::find_by_id(id)
            .one(db)
            .await?
            .ok_or_else(|| AppError::NotFound(format!("Lead {} not found", id)))
    }

    pub async fn update_lead_status(
        db: &DatabaseConnection,
        id: i64,
        new_status: &str,
    ) -> Result<crate::modules::lead::domain::entities::lead::Model, AppError> {
        let lead = Lead::find_by_id(id)
            .one(db)
            .await?
            .ok_or_else(|| AppError::NotFound(format!("Lead {} not found", id)))?;
        let mut active = <crate::modules::lead::domain::entities::lead::Entity as sea_orm::EntityTrait>::ActiveModel::from(lead);
        active.status = Set(new_status.to_string());
        active.updated_at = Set(chrono::Utc::now());
        Ok(active.update(db).await?)
    }

    /// Link a converted lead to the newly created customer
    pub async fn link_customer(
        db: &DatabaseConnection,
        lead_id: i64,
        customer_id: i64,
    ) -> Result<(), AppError> {
        use sea_orm::{ActiveModelTrait, EntityTrait, Set};
        let lead = Lead::find_by_id(lead_id)
            .one(db)
            .await?
            .ok_or_else(|| AppError::NotFound(format!("Lead {} not found", lead_id)))?;
        let mut active: crate::modules::lead::domain::entities::lead::ActiveModel = lead.into();
        active.converted_customer_id = Set(Some(customer_id));
        active.updated_at = Set(chrono::Utc::now());
        active.update(db).await?;
        Ok(())
    }

    pub async fn log_activity(
        db: &DatabaseConnection,
        lead_id: i64,
        activity_type: String,
        description: String,
        performed_by: i64,
    ) -> Result<crate::modules::lead::domain::entities::lead_activity::Model, AppError> {
        let act = LeadActivityActiveModel {
            lead_id: Set(lead_id),
            activity_type: Set(activity_type),
            description: Set(description),
            performed_by: Set(performed_by),
            created_at: Set(chrono::Utc::now()),
            ..Default::default()
        };
        Ok(act.insert(db).await?)
    }

    pub async fn get_activities(
        db: &DatabaseConnection,
        lead_id: i64,
    ) -> Result<Vec<crate::modules::lead::domain::entities::lead_activity::Model>, AppError> {
        Ok(LeadActivity::find()
            .filter(
                crate::modules::lead::domain::entities::lead_activity::Column::LeadId.eq(lead_id),
            )
            .order_by_desc(crate::modules::lead::domain::entities::lead_activity::Column::CreatedAt)
            .all(db)
            .await?)
    }

    pub async fn update_lead(
        db: &DatabaseConnection,
        id: i64,
        name: Option<String>,
        phone: Option<String>,
        email: Option<String>,
        source: Option<String>,
        notes: Option<String>,
    ) -> Result<crate::modules::lead::domain::entities::lead::Model, AppError> {
        let lead = Lead::find_by_id(id)
            .one(db)
            .await?
            .ok_or_else(|| AppError::NotFound(format!("Lead {} not found", id)))?;
        let mut active: crate::modules::lead::domain::entities::lead::ActiveModel = lead.into();
        if let Some(v) = name {
            active.name = Set(v);
        }
        if let Some(v) = phone {
            active.phone = Set(v);
        }
        if let Some(v) = email {
            active.email = Set(Some(v));
        }
        if let Some(v) = source {
            active.source = Set(v);
        }
        if let Some(v) = notes {
            active.notes = Set(Some(v));
        }
        active.updated_at = Set(chrono::Utc::now());
        Ok(active.update(db).await?)
    }

    pub async fn assign_lead(
        db: &DatabaseConnection,
        id: i64,
        assigned_to: i64,
    ) -> Result<crate::modules::lead::domain::entities::lead::Model, AppError> {
        let lead = Lead::find_by_id(id)
            .one(db)
            .await?
            .ok_or_else(|| AppError::NotFound(format!("Lead {} not found", id)))?;
        let mut active: crate::modules::lead::domain::entities::lead::ActiveModel = lead.into();
        active.assigned_to = Set(Some(assigned_to));
        active.updated_at = Set(chrono::Utc::now());
        Ok(active.update(db).await?)
    }

    pub async fn get_pipeline(
        db: &DatabaseConnection,
        branch_id: Option<i64>,
    ) -> Result<serde_json::Value, AppError> {
        let mut query = Lead::find();
        if let Some(bid) = branch_id {
            query = query.filter(LeadColumn::BranchId.eq(bid));
        }
        let leads = query.all(db).await?;

        let mut grouped: std::collections::HashMap<String, Vec<serde_json::Value>> =
            std::collections::HashMap::new();
        for lead in &leads {
            let entry = serde_json::json!({
                "id": lead.id,
                "name": lead.name,
                "phone": lead.phone,
                "source": lead.source,
                "assigned_to": lead.assigned_to,
                "created_at": lead.created_at.to_rfc3339(),
            });
            grouped.entry(lead.status.clone()).or_default().push(entry);
        }

        let pipeline: serde_json::Value = grouped
            .into_iter()
            .map(|(status, leads)| {
                let count = leads.len() as u64;
                (status, serde_json::json!({"count": count, "leads": leads}))
            })
            .collect();
        Ok(pipeline)
    }

    pub async fn get_stats(
        db: &DatabaseConnection,
        branch_id: Option<i64>,
    ) -> Result<serde_json::Value, AppError> {
        let mut query = Lead::find();
        if let Some(bid) = branch_id {
            query = query.filter(LeadColumn::BranchId.eq(bid));
        }
        let leads = query.all(db).await?;

        let total = leads.len() as u64;
        let mut status_counts: std::collections::HashMap<String, u64> =
            std::collections::HashMap::new();
        let mut source_counts: std::collections::HashMap<String, u64> =
            std::collections::HashMap::new();
        let converted = leads.iter().filter(|l| l.status == "converted").count() as u64;
        let assigned = leads.iter().filter(|l| l.assigned_to.is_some()).count() as u64;

        for lead in &leads {
            *status_counts.entry(lead.status.clone()).or_insert(0) += 1;
            *source_counts.entry(lead.source.clone()).or_insert(0) += 1;
        }

        let conversion_rate = if total > 0 {
            (converted as f64 / total as f64) * 100.0
        } else {
            0.0
        };

        Ok(serde_json::json!({
            "total": total,
            "converted": converted,
            "assigned": assigned,
            "conversion_rate": conversion_rate,
            "by_status": status_counts,
            "by_source": source_counts,
        }))
    }
}
