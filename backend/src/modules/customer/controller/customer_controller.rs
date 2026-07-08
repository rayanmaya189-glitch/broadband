use axum::extract::{Json, Path, Query, State};
use validator::Validate;

use crate::app::SharedState;
use crate::common::errors::app_error::AppError;
use crate::modules::customer::request::customer_request::*;
use crate::modules::customer::response::customer_response::*;
use crate::modules::customer::service::customer_service::CustomerService;

pub async fn list_customers(State(state): State<SharedState>, Query(query): Query<ListCustomersQuery>) -> Result<Json<crate::common::utils::helpers::PaginatedResponse<CustomerResponse>>, AppError> {
    let svc = CustomerService::new(&state.db);
    Ok(Json(svc.list_customers(&query).await?))
}

pub async fn create_customer(State(state): State<SharedState>, Json(req): Json<CreateCustomerRequest>) -> Result<Json<CustomerResponse>, AppError> {
    req.validate()?;
    let svc = CustomerService::new(&state.db);
    Ok(Json(svc.create_customer(&req).await?))
}

pub async fn get_customer(State(state): State<SharedState>, Path(id): Path<i64>) -> Result<Json<CustomerResponse>, AppError> {
    let svc = CustomerService::new(&state.db);
    Ok(Json(svc.get_customer(id).await?))
}

pub async fn update_customer(State(state): State<SharedState>, Path(id): Path<i64>, Json(req): Json<UpdateCustomerRequest>) -> Result<Json<CustomerResponse>, AppError> {
    req.validate()?;
    let svc = CustomerService::new(&state.db);
    Ok(Json(svc.update_customer(id, &req).await?))
}

pub async fn update_status(State(state): State<SharedState>, Path(id): Path<i64>, Json(req): Json<CustomerStatusTransition>) -> Result<Json<CustomerResponse>, AppError> {
    let svc = CustomerService::new(&state.db);
    Ok(Json(svc.update_status(id, &req).await?))
}

pub async fn delete_customer(State(state): State<SharedState>, Path(id): Path<i64>) -> Result<Json<MessageResponse>, AppError> {
    let svc = CustomerService::new(&state.db);
    Ok(Json(svc.delete_customer(id).await?))
}
