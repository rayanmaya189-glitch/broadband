use axum::extract::{Path, State};
use axum::http::StatusCode;
use axum::Json;
use serde::{Deserialize, Serialize};
use std::sync::Arc;

use crate::modules::plans::application::services::PlanService;
use crate::shared::app_state::AppState;
use crate::shared::errors::AppError;
use crate::shared::middleware::auth::{require_permission, UserContext};

#[derive(Debug, Serialize)]
pub struct PlanResponse {
    pub id: i64,
    pub slug: String,
    pub name: String,
    #[serde(default)]
    pub description: Option<String>,
    pub speed_label: String,
    pub download_mbps: i32,
    pub upload_mbps: i32,
    pub burst_mbps: Option<i32>,
    pub is_popular: bool,
    pub is_business: bool,
    pub is_active: bool,
    pub pricing: Vec<PricingResponse>,
}

#[derive(Debug, Serialize)]
pub struct PricingResponse {
    pub billing_period_months: i32,
    pub price: String,
    pub savings: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct CreatePlanRequest {
    pub slug: String,
    pub name: String,
    #[serde(default)]
    pub description: Option<String>,
    pub speed_label: String,
    pub download_mbps: i32,
    pub upload_mbps: i32,
    #[serde(default)]
    pub burst_mbps: Option<i32>,
    #[serde(default)]
    pub is_business: Option<bool>,
}

#[derive(Debug, Deserialize)]
pub struct UpdatePricingRequest {
    pub billing_period_months: i32,
    pub price: String,
}

pub async fn list_plans(
    State(state): State<Arc<AppState>>,
) -> Result<Json<Vec<PlanResponse>>, AppError> {
    let plans = PlanService::list_active_plans(&state.db).await?;
    let mut responses = Vec::new();
    for plan in plans {
        let (_, pricing) = PlanService::get_plan_with_pricing(&state.db, plan.id).await?;
        responses.push(PlanResponse {
            id: plan.id,
            slug: plan.slug,
            name: plan.name,
            description: plan.description,
            speed_label: plan.speed_label,
            download_mbps: plan.download_mbps,
            upload_mbps: plan.upload_mbps,
            burst_mbps: plan.burst_mbps,
            is_popular: plan.is_popular,
            is_business: plan.is_business,
            is_active: plan.is_active,
            pricing: pricing
                .into_iter()
                .map(|p| PricingResponse {
                    billing_period_months: p.billing_period_months,
                    price: p.price.to_string(),
                    savings: p.savings.map(|s| s.to_string()),
                })
                .collect(),
        });
    }
    Ok(Json(responses))
}

pub async fn get_plan(
    State(state): State<Arc<AppState>>,
    Path(id): Path<i64>,
) -> Result<Json<PlanResponse>, AppError> {
    let (plan, pricing) = PlanService::get_plan_with_pricing(&state.db, id).await?;
    Ok(Json(PlanResponse {
        id: plan.id,
        slug: plan.slug,
        name: plan.name,
        description: plan.description,
        speed_label: plan.speed_label,
        download_mbps: plan.download_mbps,
        upload_mbps: plan.upload_mbps,
        burst_mbps: plan.burst_mbps,
        is_popular: plan.is_popular,
        is_business: plan.is_business,
        is_active: plan.is_active,
        pricing: pricing
            .into_iter()
            .map(|p| PricingResponse {
                billing_period_months: p.billing_period_months,
                price: p.price.to_string(),
                savings: p.savings.map(|s| s.to_string()),
            })
            .collect(),
    }))
}

pub async fn create_plan(
    State(state): State<Arc<AppState>>,
    user: UserContext,
    Json(req): Json<CreatePlanRequest>,
) -> Result<(StatusCode, Json<PlanResponse>), AppError> {
    require_permission(&user, "plan.create").map_err(|e| AppError::Forbidden(e.1))?;
    if !user.is_company_wide {
        return Err(AppError::Forbidden("Insufficient permissions".to_string()));
    }
    let plan = PlanService::create_plan(
        &state.db,
        req.slug,
        req.name,
        req.description,
        req.speed_label,
        req.download_mbps,
        req.upload_mbps,
        req.burst_mbps,
        req.is_business.unwrap_or(false),
    )
    .await?;
    if let Err(e) = crate::infrastructure::messaging::outbox::insert_outbox_event(
        &state.db,
        "plan.created",
        "plan",
        plan.id,
        serde_json::json!({"plan_id": plan.id, "slug": plan.slug, "name": plan.name}),
        None,
        Some(user.user_id),
        user.branch_id,
    )
    .await
    {
        tracing::error!(error = %e, "Failed to publish plan.created event");
    }
    Ok((
        StatusCode::CREATED,
        Json(PlanResponse {
            id: plan.id,
            slug: plan.slug,
            name: plan.name,
            description: plan.description,
            speed_label: plan.speed_label,
            download_mbps: plan.download_mbps,
            upload_mbps: plan.upload_mbps,
            burst_mbps: plan.burst_mbps,
            is_popular: plan.is_popular,
            is_business: plan.is_business,
            is_active: plan.is_active,
            pricing: vec![],
        }),
    ))
}

pub async fn update_pricing(
    State(state): State<Arc<AppState>>,
    user: UserContext,
    Path(id): Path<i64>,
    Json(req): Json<UpdatePricingRequest>,
) -> Result<StatusCode, AppError> {
    require_permission(&user, "plan.pricing.update").map_err(|e| AppError::Forbidden(e.1))?;
    if !user.is_company_wide {
        return Err(AppError::Forbidden("Insufficient permissions".to_string()));
    }
    let price: sea_orm::prelude::Decimal = req
        .price
        .parse()
        .map_err(|_| AppError::Validation("Invalid price".into()))?;
    PlanService::update_pricing(&state.db, id, req.billing_period_months, price).await?;
    if let Err(e) = crate::infrastructure::messaging::outbox::insert_outbox_event(
        &state.db,
        "plan.pricing.updated",
        "plan",
        id,
        serde_json::json!({"plan_id": id}),
        None,
        Some(user.user_id),
        user.branch_id,
    )
    .await
    {
        tracing::error!(error = %e, "Failed to publish plan.pricing.updated event");
    }
    Ok(StatusCode::OK)
}

pub async fn approve_plan(
    State(state): State<Arc<AppState>>,
    user: UserContext,
    Path(id): Path<i64>,
) -> Result<StatusCode, AppError> {
    require_permission(&user, "plan.approve").map_err(|e| AppError::Forbidden(e.1))?;
    if !user.is_company_wide {
        return Err(AppError::Forbidden("Insufficient permissions".to_string()));
    }
    PlanService::approve_plan(&state.db, id, user.user_id).await?;
    if let Err(e) = crate::infrastructure::messaging::outbox::insert_outbox_event(
        &state.db,
        "plan.approved",
        "plan",
        id,
        serde_json::json!({"plan_id": id}),
        None,
        Some(user.user_id),
        user.branch_id,
    )
    .await
    {
        tracing::error!(error = %e, "Failed to publish plan.approved event");
    }
    Ok(StatusCode::OK)
}

pub async fn deactivate_plan(
    State(state): State<Arc<AppState>>,
    user: UserContext,
    Path(id): Path<i64>,
) -> Result<StatusCode, AppError> {
    require_permission(&user, "plan.deactivate").map_err(|e| AppError::Forbidden(e.1))?;
    if !user.is_company_wide {
        return Err(AppError::Forbidden("Insufficient permissions".to_string()));
    }
    PlanService::deactivate_plan(&state.db, id).await?;
    if let Err(e) = crate::infrastructure::messaging::outbox::insert_outbox_event(
        &state.db,
        "plan.deactivated",
        "plan",
        id,
        serde_json::json!({"plan_id": id}),
        None,
        Some(user.user_id),
        user.branch_id,
    )
    .await
    {
        tracing::error!(error = %e, "Failed to publish plan.deactivated event");
    }
    Ok(StatusCode::NO_CONTENT)
}
