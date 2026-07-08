use axum::extract::{Json, Path, Query, State};

use crate::app::SharedState;
use crate::common::errors::app_error::AppError;
use crate::modules::subscription::request::subscription_request::*;
use crate::modules::subscription::response::subscription_response::*;
use crate::modules::subscription::service::subscription_service::SubscriptionService;

pub async fn list_subscriptions(State(state): State<SharedState>, Query(query): Query<ListSubscriptionsQuery>) -> Result<Json<crate::common::utils::helpers::PaginatedResponse<SubscriptionResponse>>, AppError> {
    let svc = SubscriptionService::new(&state.db);
    Ok(Json(svc.list_subscriptions(&query).await?))
}

pub async fn create_subscription(State(state): State<SharedState>, Json(req): Json<CreateSubscriptionRequest>) -> Result<Json<SubscriptionResponse>, AppError> {
    let svc = SubscriptionService::new(&state.db);
    Ok(Json(svc.create_subscription(&req).await?))
}

pub async fn get_subscription(State(state): State<SharedState>, Path(id): Path<i64>) -> Result<Json<SubscriptionResponse>, AppError> {
    let svc = SubscriptionService::new(&state.db);
    Ok(Json(svc.get_subscription(id).await?))
}

pub async fn suspend_subscription(State(state): State<SharedState>, Path(id): Path<i64>) -> Result<Json<SubscriptionResponse>, AppError> {
    let svc = SubscriptionService::new(&state.db);
    Ok(Json(svc.suspend_subscription(id).await?))
}

pub async fn reactivate_subscription(State(state): State<SharedState>, Path(id): Path<i64>) -> Result<Json<SubscriptionResponse>, AppError> {
    let svc = SubscriptionService::new(&state.db);
    Ok(Json(svc.reactivate_subscription(id).await?))
}

pub async fn cancel_subscription(State(state): State<SharedState>, Path(id): Path<i64>) -> Result<Json<MessageResponse>, AppError> {
    let svc = SubscriptionService::new(&state.db);
    Ok(Json(svc.cancel_subscription(id).await?))
}
