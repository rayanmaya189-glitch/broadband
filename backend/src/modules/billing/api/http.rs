use axum::extract::{Query, State};
use axum::http::StatusCode;
use axum::Json;
use serde::{Deserialize, Serialize};
use std::sync::Arc;

use crate::modules::billing::application::services::BillingService;
use crate::shared::app_state::AppState;
use crate::shared::errors::AppError;
use crate::shared::middleware::auth::UserContext;
use crate::shared::primitives::PaginationParams;

#[derive(Debug, Serialize)]
pub struct InvoiceResponse {
    pub id: i64,
    pub invoice_number: String,
    pub customer_id: i64,
    pub total_amount: String,
    pub status: String,
    pub due_date: String,
}

#[derive(Debug, Serialize)]
pub struct PaymentResponse {
    pub id: i64,
    pub payment_number: String,
    pub invoice_id: i64,
    pub amount: String,
    pub payment_method: String,
    pub status: String,
}

#[derive(Debug, Deserialize)]
pub struct CreateInvoiceRequest {
    pub customer_id: i64,
    pub branch_id: i64,
    pub subscription_id: i64,
    pub billing_period_start: String,
    pub billing_period_end: String,
    pub total_amount: String,
}

#[derive(Debug, Deserialize)]
pub struct RecordPaymentRequest {
    pub invoice_id: i64,
    pub customer_id: i64,
    pub branch_id: i64,
    pub amount: String,
    pub payment_method: String,
}

/// GET /api/v1/billing/invoices
pub async fn list_invoices(
    State(state): State<Arc<AppState>>,
    user: UserContext,
    Query(p): Query<PaginationParams>,
) -> Result<Json<serde_json::Value>, AppError> {
    let bid = if user.is_company_wide {
        None
    } else {
        user.branch_id
    };
    let (items, total) = BillingService::list_invoices(&state.db, bid, p.page(), p.limit()).await?;
    let resp: Vec<InvoiceResponse> = items
        .into_iter()
        .map(|i| InvoiceResponse {
            id: i.id,
            invoice_number: i.invoice_number,
            customer_id: i.customer_id,
            total_amount: i.total_amount.to_string(),
            status: i.status,
            due_date: i.due_date.to_string(),
        })
        .collect();
    Ok(Json(serde_json::json!({"items": resp, "total": total})))
}

/// POST /api/v1/billing/invoices
pub async fn create_invoice(
    State(state): State<Arc<AppState>>,
    _user: UserContext,
    Json(req): Json<CreateInvoiceRequest>,
) -> Result<(StatusCode, Json<InvoiceResponse>), AppError> {
    let start: chrono::NaiveDate = req
        .billing_period_start
        .parse()
        .map_err(|_| AppError::Validation("Invalid date".into()))?;
    let end: chrono::NaiveDate = req
        .billing_period_end
        .parse()
        .map_err(|_| AppError::Validation("Invalid date".into()))?;
    let amt: sea_orm::prelude::Decimal = req
        .total_amount
        .parse()
        .map_err(|_| AppError::Validation("Invalid amount".into()))?;
    let inv = BillingService::create_invoice(
        &state.db,
        req.customer_id,
        req.branch_id,
        req.subscription_id,
        start,
        end,
        amt,
    )
    .await?;
    Ok((
        StatusCode::CREATED,
        Json(InvoiceResponse {
            id: inv.id,
            invoice_number: inv.invoice_number,
            customer_id: inv.customer_id,
            total_amount: inv.total_amount.to_string(),
            status: inv.status,
            due_date: inv.due_date.to_string(),
        }),
    ))
}

/// POST /api/v1/billing/payments
pub async fn record_payment(
    State(state): State<Arc<AppState>>,
    _user: UserContext,
    Json(req): Json<RecordPaymentRequest>,
) -> Result<(StatusCode, Json<PaymentResponse>), AppError> {
    let amt: sea_orm::prelude::Decimal = req
        .amount
        .parse()
        .map_err(|_| AppError::Validation("Invalid amount".into()))?;
    let pay = BillingService::record_payment(
        &state.db,
        req.invoice_id,
        req.customer_id,
        req.branch_id,
        amt,
        req.payment_method,
    )
    .await?;
    Ok((
        StatusCode::CREATED,
        Json(PaymentResponse {
            id: pay.id,
            payment_number: pay.payment_number,
            invoice_id: pay.invoice_id,
            amount: pay.amount.to_string(),
            payment_method: pay.payment_method,
            status: pay.status,
        }),
    ))
}

/// GET /api/v1/billing/payments
pub async fn list_payments(
    State(state): State<Arc<AppState>>,
    user: UserContext,
    Query(p): Query<PaginationParams>,
) -> Result<Json<serde_json::Value>, AppError> {
    let bid = if user.is_company_wide {
        None
    } else {
        user.branch_id
    };
    let (items, total) = BillingService::list_payments(&state.db, bid, p.page(), p.limit()).await?;
    let resp: Vec<PaymentResponse> = items
        .into_iter()
        .map(|pay| PaymentResponse {
            id: pay.id,
            payment_number: pay.payment_number,
            invoice_id: pay.invoice_id,
            amount: pay.amount.to_string(),
            payment_method: pay.payment_method,
            status: pay.status,
        })
        .collect();
    Ok(Json(serde_json::json!({"items": resp, "total": total})))
}
