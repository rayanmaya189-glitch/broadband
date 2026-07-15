use axum::extract::{Path, State};
use axum::http::StatusCode;
use axum::Json;
use serde::{Deserialize, Serialize};
use std::sync::Arc;

use crate::shared::app_state::AppState;
use crate::shared::errors::AppError;
use crate::shared::middleware::auth::UserContext;
use crate::modules::plans::application::services::PlanService;

#[derive(Debug, Serialize)]
pub struct PlanResponse {
    pub id: i64, pub slug: String, pub name: String,    #[serde(default)]
    pub description: Option<String>,
    pub speed_label: String, pub download_mbps: i32, pub upload_mbps: i32,
    pub burst_mbps: Option<i32>, pub is_popular: bool, pub is_business: bool,
    pub is_active: bool, pub pricing: Vec<PricingResponse>,
}

#[derive(Debug, Serialize)]
pub struct PricingResponse {
    pub billing_period_months: i32, pub price: String, pub savings: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct CreatePlanRequest {
    pub slug: String, pub name: String,    #[serde(default)]
    pub description: Option<String>,
    pub speed_label: String, pub download_mbps: i32, pub upload_mbps: i32,
    #[serde(default)]
    pub burst_mbps: Option<i32>,
    #[serde(default)]
    pub is_business: Option<bool>,
}

#[derive(Debug, Deserialize)]
pub struct UpdatePricingRequest {
    pub billing_period_months: i32, pub price: String,
}

pub async fn list_plans(State(state): State<Arc<AppState>>) -> Result<Json<Vec<PlanResponse>>, AppError> {
    let plans = PlanService::list_active_plans(&state.db).await?;
    let mut responses = Vec::new();
    for plan in plans {
        let (_, pricing) = PlanService::get_plan_with_pricing(&state.db, plan.id).await?;
        responses.push(PlanResponse {
            id: plan.id, slug: plan.slug, name: plan.name, description: plan.description,
            speed_label: plan.speed_label, download_mbps: plan.download_mbps,
            upload_mbps: plan.upload_mbps, burst_mbps: plan.burst_mbps,
            is_popular: plan.is_popular, is_business: plan.is_business, is_active: plan.is_active,
            pricing: pricing.into_iter().map(|p| PricingResponse {
                billing_period_months: p.billing_period_months, price: p.price.to_string(),
                savings: p.savings.map(|s| s.to_string()),
            }).collect(),
        });
    }
    Ok(Json(responses))
}

pub async fn get_plan(State(state): State<Arc<AppState>>, Path(id): Path<i64>) -> Result<Json<PlanResponse>, AppError> {
    let (plan, pricing) = PlanService::get_plan_with_pricing(&state.db, id).await?;
    Ok(Json(PlanResponse {
        id: plan.id, slug: plan.slug, name: plan.name, description: plan.description,
        speed_label: plan.speed_label, download_mbps: plan.download_mbps,
        upload_mbps: plan.upload_mbps, burst_mbps: plan.burst_mbps,
        is_popular: plan.is_popular, is_business: plan.is_business, is_active: plan.is_active,
        pricing: pricing.into_iter().map(|p| PricingResponse {
            billing_period_months: p.billing_period_months, price: p.price.to_string(),
            savings: p.savings.map(|s| s.to_string()),
        }).collect(),
    }))
}

pub async fn create_plan(
    State(state): State<Arc<AppState>>, user: UserContext, Json(req): Json<CreatePlanRequest>,
) -> Result<(StatusCode, Json<PlanResponse>), AppError> {
    if !user.is_company_wide { return Err(AppError::Forbidden("Insufficient permissions".to_string())); }
    let plan = PlanService::create_plan(&state.db, req.slug, req.name, req.description, req.speed_label, req.download_mbps, req.upload_mbps, req.burst_mbps, req.is_business.unwrap_or(false)).await?;
    Ok((StatusCode::CREATED, Json(PlanResponse {
        id: plan.id, slug: plan.slug, name: plan.name, description: plan.description,
        speed_label: plan.speed_label, download_mbps: plan.download_mbps,
        upload_mbps: plan.upload_mbps, burst_mbps: plan.burst_mbps,
        is_popular: plan.is_popular, is_business: plan.is_business, is_active: plan.is_active, pricing: vec![],
    })))
}

pub async fn update_pricing(
    State(state): State<Arc<AppState>>, user: UserContext,
    Path(id): Path<i64>, Json(req): Json<UpdatePricingRequest>,
) -> Result<StatusCode, AppError> {
    if !user.is_company_wide { return Err(AppError::Forbidden("Insufficient permissions".to_string())); }
    let price: sea_orm::prelude::Decimal = req.price.parse().map_err(|_| AppError::Validation("Invalid price".into()))?;
    PlanService::update_pricing(&state.db, id, req.billing_period_months, price).await?;
    Ok(StatusCode::OK)
}

pub async fn approve_plan(State(state): State<Arc<AppState>>, user: UserContext, Path(id): Path<i64>) -> Result<StatusCode, AppError> {
    if !user.is_company_wide { return Err(AppError::Forbidden("Insufficient permissions".to_string())); }
    PlanService::approve_plan(&state.db, id, user.user_id).await?;
    Ok(StatusCode::OK)
}

pub async fn deactivate_plan(State(state): State<Arc<AppState>>, user: UserContext, Path(id): Path<i64>) -> Result<StatusCode, AppError> {
    if !user.is_company_wide { return Err(AppError::Forbidden("Insufficient permissions".to_string())); }
    PlanService::deactivate_plan(&state.db, id).await?;
    Ok(StatusCode::NO_CONTENT)
}
