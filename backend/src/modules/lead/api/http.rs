use crate::modules::lead::application::services::LeadService;
use crate::shared::app_state::AppState;
use crate::shared::errors::AppError;
use crate::shared::middleware::auth::{require_permission, UserContext};
use crate::shared::primitives::PaginationParams;
use axum::extract::{Path, Query, State};
use axum::http::StatusCode;
use axum::Json;
use serde::{Deserialize, Serialize};
use std::sync::Arc;

#[derive(Debug, Serialize)]
pub struct LeadResponse {
    pub id: i64,
    pub name: String,
    pub phone: String,
    pub status: String,
    pub source: String,
}

#[derive(Debug, Deserialize)]
pub struct CreateLeadRequest {
    pub name: String,
    pub phone: String,
    #[serde(default)]
    pub email: Option<String>,
    pub source: String,
}

pub async fn list_leads(
    State(state): State<Arc<AppState>>,
    user: UserContext,
    Query(p): Query<PaginationParams>,
) -> Result<Json<serde_json::Value>, AppError> {
    require_permission(&user, "lead.view").map_err(|e| AppError::Forbidden(e.1))?;
    let bid = if user.is_company_wide {
        None
    } else {
        user.branch_id
    };
    let (leads, total) = LeadService::list_leads(&state.db, bid, p.page(), p.limit()).await?;
    let items: Vec<LeadResponse> = leads
        .into_iter()
        .map(|l| LeadResponse {
            id: l.id,
            name: l.name,
            phone: l.phone,
            status: l.status,
            source: l.source,
        })
        .collect();
    Ok(Json(
        serde_json::json!({"items": items, "total": total, "page": p.page(), "limit": p.limit()}),
    ))
}

pub async fn create_lead(
    State(state): State<Arc<AppState>>,
    user: UserContext,
    Json(req): Json<CreateLeadRequest>,
) -> Result<(StatusCode, Json<LeadResponse>), AppError> {
    require_permission(&user, "lead.create").map_err(|e| AppError::Forbidden(e.1))?;
    let l = LeadService::create_lead(
        &state.db,
        user.branch_id.unwrap_or(0),
        req.name,
        req.phone,
        req.email,
        req.source,
    )
    .await?;
    // Publish event to outbox
    if let Err(e) = crate::infrastructure::messaging::outbox::insert_outbox_event(
        &state.db,
        "lead.created",
        "lead",
        l.id,
        serde_json::json!({"lead_id": l.id, "name": l.name, "status": l.status}),
        None,
        Some(user.user_id),
        user.branch_id,
    )
    .await
    {
        tracing::error!(error = %e, "Failed to publish lead.created event");
    }

    Ok((
        StatusCode::CREATED,
        Json(LeadResponse {
            id: l.id,
            name: l.name,
            phone: l.phone,
            status: l.status,
            source: l.source,
        }),
    ))
}

pub async fn update_lead_status(
    State(state): State<Arc<AppState>>,
    user: UserContext,
    Path(id): Path<i64>,
    Json(req): Json<UpdateStatusRequest>,
) -> Result<Json<LeadResponse>, AppError> {
    require_permission(&user, "lead.status.update").map_err(|e| AppError::Forbidden(e.1))?;
    let l = LeadService::update_lead_status(&state.db, id, &req.status).await?;
    // Publish event to outbox
    if let Err(e) = crate::infrastructure::messaging::outbox::insert_outbox_event(
        &state.db,
        "lead.status.updated",
        "lead",
        l.id,
        serde_json::json!({"lead_id": l.id, "new_status": l.status}),
        None,
        Some(user.user_id),
        user.branch_id,
    )
    .await
    {
        tracing::error!(error = %e, "Failed to publish lead.status.updated event");
    }

    Ok(Json(LeadResponse {
        id: l.id,
        name: l.name,
        phone: l.phone,
        status: l.status,
        source: l.source,
    }))
}

#[derive(Debug, Deserialize)]
pub struct UpdateStatusRequest {
    pub status: String,
}

pub async fn get_lead(
    State(state): State<Arc<AppState>>,
    user: UserContext,
    Path(id): Path<i64>,
) -> Result<Json<LeadResponse>, AppError> {
    require_permission(&user, "lead.view").map_err(|e| AppError::Forbidden(e.1))?;
    let l = LeadService::get_lead(&state.db, id).await?;
    Ok(Json(LeadResponse {
        id: l.id,
        name: l.name,
        phone: l.phone,
        status: l.status,
        source: l.source,
    }))
}

#[derive(Debug, Deserialize)]
pub struct UpdateLeadRequest {
    #[serde(default)]
    pub name: Option<String>,
    #[serde(default)]
    pub phone: Option<String>,
    #[serde(default)]
    pub email: Option<String>,
    #[serde(default)]
    pub source: Option<String>,
    #[serde(default)]
    pub notes: Option<String>,
}

pub async fn update_lead(
    State(state): State<Arc<AppState>>,
    user: UserContext,
    Path(id): Path<i64>,
    Json(req): Json<UpdateLeadRequest>,
) -> Result<Json<LeadResponse>, AppError> {
    require_permission(&user, "lead.update").map_err(|e| AppError::Forbidden(e.1))?;
    let l = LeadService::update_lead(
        &state.db,
        id,
        req.name,
        req.phone,
        req.email,
        req.source,
        req.notes,
    )
    .await?;
    Ok(Json(LeadResponse {
        id: l.id,
        name: l.name,
        phone: l.phone,
        status: l.status,
        source: l.source,
    }))
}

#[derive(Debug, Deserialize)]
pub struct AssignLeadRequest {
    pub assigned_to: i64,
}

pub async fn assign_lead(
    State(state): State<Arc<AppState>>,
    user: UserContext,
    Path(id): Path<i64>,
    Json(req): Json<AssignLeadRequest>,
) -> Result<Json<LeadResponse>, AppError> {
    require_permission(&user, "lead.assign").map_err(|e| AppError::Forbidden(e.1))?;
    let l = LeadService::assign_lead(&state.db, id, req.assigned_to).await?;
    if let Err(e) = crate::infrastructure::messaging::outbox::insert_outbox_event(
        &state.db,
        "lead.assigned",
        "lead",
        id,
        serde_json::json!({"lead_id": id, "assigned_to": req.assigned_to}),
        None,
        Some(user.user_id),
        user.branch_id,
    )
    .await
    {
        tracing::error!(lead_id = id, error = %e, "Failed to publish lead.assigned event");
    }
    Ok(Json(LeadResponse {
        id: l.id,
        name: l.name,
        phone: l.phone,
        status: l.status,
        source: l.source,
    }))
}

#[derive(Debug, Deserialize)]
pub struct LogActivityRequest {
    pub activity_type: String,
    pub description: String,
}

#[derive(Debug, Serialize)]
pub struct ActivityResponse {
    pub id: i64,
    pub lead_id: i64,
    pub activity_type: String,
    pub description: String,
    pub performed_by: i64,
    pub created_at: String,
}

pub async fn log_activity(
    State(state): State<Arc<AppState>>,
    user: UserContext,
    Path(id): Path<i64>,
    Json(req): Json<LogActivityRequest>,
) -> Result<(StatusCode, Json<ActivityResponse>), AppError> {
    require_permission(&user, "lead.update").map_err(|e| AppError::Forbidden(e.1))?;
    let act = LeadService::log_activity(
        &state.db,
        id,
        req.activity_type,
        req.description,
        user.user_id,
    )
    .await?;
    Ok((
        StatusCode::CREATED,
        Json(ActivityResponse {
            id: act.id,
            lead_id: act.lead_id,
            activity_type: act.activity_type,
            description: act.description,
            performed_by: act.performed_by,
            created_at: act.created_at.to_rfc3339(),
        }),
    ))
}

pub async fn list_activities(
    State(state): State<Arc<AppState>>,
    user: UserContext,
    Path(id): Path<i64>,
) -> Result<Json<Vec<ActivityResponse>>, AppError> {
    require_permission(&user, "lead.view").map_err(|e| AppError::Forbidden(e.1))?;
    let acts = LeadService::get_activities(&state.db, id).await?;
    let items: Vec<ActivityResponse> = acts
        .into_iter()
        .map(|a| ActivityResponse {
            id: a.id,
            lead_id: a.lead_id,
            activity_type: a.activity_type,
            description: a.description,
            performed_by: a.performed_by,
            created_at: a.created_at.to_rfc3339(),
        })
        .collect();
    Ok(Json(items))
}

pub async fn get_pipeline(
    State(state): State<Arc<AppState>>,
    user: UserContext,
) -> Result<Json<serde_json::Value>, AppError> {
    require_permission(&user, "lead.view").map_err(|e| AppError::Forbidden(e.1))?;
    let bid = if user.is_company_wide {
        None
    } else {
        user.branch_id
    };
    let pipeline = LeadService::get_pipeline(&state.db, bid).await?;
    Ok(Json(pipeline))
}

pub async fn get_stats(
    State(state): State<Arc<AppState>>,
    user: UserContext,
) -> Result<Json<serde_json::Value>, AppError> {
    require_permission(&user, "lead.view").map_err(|e| AppError::Forbidden(e.1))?;
    let bid = if user.is_company_wide {
        None
    } else {
        user.branch_id
    };
    let stats = LeadService::get_stats(&state.db, bid).await?;
    Ok(Json(stats))
}

#[derive(Debug, Deserialize)]
pub struct ConvertLeadRequest {
    #[serde(default)]
    pub plan_id: Option<i64>,
    #[serde(default)]
    pub address: Option<serde_json::Value>,
}

#[derive(Debug, Serialize)]
pub struct ConvertLeadResponse {
    pub lead_id: i64,
    pub customer_id: i64,
    pub customer_code: String,
    pub status: String,
}

/// POST /api/v1/leads/:id/convert
/// Converts a qualified lead into a customer, creates customer record,
/// links the lead, and publishes lead.converted event.
pub async fn convert_lead(
    State(state): State<Arc<AppState>>,
    user: UserContext,
    Path(id): Path<i64>,
    Json(req): Json<ConvertLeadRequest>,
) -> Result<(StatusCode, Json<ConvertLeadResponse>), AppError> {
    require_permission(&user, "lead.convert").map_err(|e| AppError::Forbidden(e.1))?;

    // 1. Fetch the lead
    let lead = LeadService::get_lead(&state.db, id).await?;

    // 2. Validate lead is in a convertible state
    let convertible_statuses = ["quoted", "interested", "surveyed"];
    if !convertible_statuses.contains(&lead.status.as_str()) {
        return Err(AppError::Validation(format!(
            "Lead in status '{}' cannot be converted. Must be: quoted, interested, or surveyed",
            lead.status
        )));
    }

    // 3. Create customer from lead data
    let branch_id = user.branch_id.unwrap_or(lead.branch_id);
    let customer =
        crate::modules::customer::application::services::CustomerService::create_customer(
            &state.db,
            branch_id,
            lead.name.clone(),
            lead.email.clone(),
            lead.phone.clone(),
            None,
        )
        .await?;

    // 4. Link lead to the new customer
    LeadService::update_lead_status(&state.db, id, "converted").await?;
    LeadService::link_customer(&state.db, id, customer.id).await?;

    // 5. Add installation address if provided
    if let Some(addr) = req.address {
        let line1 = addr["line1"].as_str().unwrap_or("");
        let city = addr["city"].as_str().unwrap_or("");
        let state_val = addr["state"].as_str().unwrap_or("");
        let pincode = addr["pincode"].as_str().unwrap_or("");
        if !line1.is_empty() && !city.is_empty() {
            let _ = crate::modules::customer::application::services::CustomerService::add_address(
                &state.db,
                customer.id,
                "installation".to_string(),
                line1.to_string(),
                addr["line2"].as_str().map(|s| s.to_string()),
                city.to_string(),
                state_val.to_string(),
                pincode.to_string(),
                addr["landmark"].as_str().map(|s| s.to_string()),
            )
            .await;
        }
    }

    // 6. Log activity
    let _ = LeadService::log_activity(
        &state.db,
        id,
        "converted".to_string(),
        format!("Lead converted to customer {}", customer.customer_code),
        user.user_id,
    )
    .await;

    // 7. Publish lead.converted event
    let payload = serde_json::json!({
        "lead_id": id,
        "customer_id": customer.id,
        "customer_code": customer.customer_code,
        "lead_name": lead.name,
        "plan_id": req.plan_id,
    });
    if let Err(e) = crate::infrastructure::messaging::outbox::insert_outbox_event(
        &state.db,
        "lead.converted",
        "lead",
        id,
        payload,
        None,
        Some(user.user_id),
        user.branch_id,
    )
    .await
    {
        tracing::error!(lead_id = id, error = %e, "Failed to publish lead.converted event");
    }

    tracing::info!(
        lead_id = id,
        customer_id = customer.id,
        customer_code = %customer.customer_code,
        "Lead successfully converted to customer"
    );

    Ok((
        StatusCode::CREATED,
        Json(ConvertLeadResponse {
            lead_id: id,
            customer_id: customer.id,
            customer_code: customer.customer_code,
            status: "converted".to_string(),
        }),
    ))
}
