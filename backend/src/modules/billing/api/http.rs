use axum::extract::{Path, Query, State};
use axum::http::StatusCode;
use axum::Json;
use serde::{Deserialize, Serialize};
use std::sync::Arc;

use crate::modules::billing::application::services::BillingService;
use crate::shared::app_state::AppState;
use crate::shared::errors::AppError;
use crate::shared::middleware::auth::{require_permission, UserContext};
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
    user: UserContext,
    Json(req): Json<CreateInvoiceRequest>,
) -> Result<(StatusCode, Json<InvoiceResponse>), AppError> {
    require_permission(&user, "billing.invoice.create").map_err(|e| AppError::Forbidden(e.1))?;
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

    let payload = serde_json::json!({
        "invoice_id": inv.id,
        "invoice_number": inv.invoice_number,
        "customer_id": inv.customer_id,
        "total_amount": inv.total_amount,
        "due_date": inv.due_date,
    });
    if let Err(e) = crate::infrastructure::messaging::outbox::insert_outbox_event(
        &state.db,
        "invoice.generated",
        "invoice",
        inv.id,
        payload,
        None,
        None,
        Some(inv.branch_id),
    )
    .await
    {
        tracing::error!(invoice_id = inv.id, error = %e, "Failed to publish invoice.generated event");
    }

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
    user: UserContext,
    Json(req): Json<RecordPaymentRequest>,
) -> Result<(StatusCode, Json<PaymentResponse>), AppError> {
    require_permission(&user, "billing.payment.record").map_err(|e| AppError::Forbidden(e.1))?;
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

    let payload = serde_json::json!({
        "payment_id": pay.id,
        "payment_number": pay.payment_number,
        "invoice_id": pay.invoice_id,
        "customer_id": pay.customer_id,
        "amount": pay.amount,
        "payment_method": pay.payment_method,
        "status": pay.status,
    });
    if let Err(e) = crate::infrastructure::messaging::outbox::insert_outbox_event(
        &state.db,
        "payment.completed",
        "payment",
        pay.id,
        payload,
        None,
        None,
        Some(pay.branch_id),
    )
    .await
    {
        tracing::error!(payment_id = pay.id, error = %e, "Failed to publish payment.completed event");
    }

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

/// GET /api/v1/billing/invoices/overdue
/// List overdue invoices (due_date < today, status = pending)
pub async fn list_overdue_invoices(
    State(state): State<Arc<AppState>>,
    user: UserContext,
) -> Result<Json<serde_json::Value>, AppError> {
    let bid = if user.is_company_wide {
        None
    } else {
        user.branch_id
    };
    let items = BillingService::list_overdue_invoices(&state.db, bid).await?;
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
    Ok(Json(
        serde_json::json!({ "items": resp, "total": resp.len() }),
    ))
}

/// POST /api/v1/billing/invoices/auto-generate
/// Auto-generate invoices for subscriptions due for billing
pub async fn auto_generate_invoices(
    State(state): State<Arc<AppState>>,
    user: UserContext,
) -> Result<Json<serde_json::Value>, AppError> {
    require_permission(&user, "billing.invoice.auto_generate")
        .map_err(|e| AppError::Forbidden(e.1))?;
    let count = BillingService::auto_generate_invoices(&state.db).await?;
    Ok(Json(serde_json::json!({
        "generated": count,
        "message": format!("{} invoice(s) auto-generated", count),
    })))
}

// ─── Invoice Detail ──────────────────────────────────────────────────────────

/// GET /api/v1/billing/invoices/:id
pub async fn get_invoice(
    State(state): State<Arc<AppState>>,
    _user: UserContext,
    Path(id): Path<i64>,
) -> Result<Json<InvoiceResponse>, AppError> {
    let inv = BillingService::get_invoice(&state.db, id).await?;
    Ok(Json(InvoiceResponse {
        id: inv.id,
        invoice_number: inv.invoice_number,
        customer_id: inv.customer_id,
        total_amount: inv.total_amount.to_string(),
        status: inv.status,
        due_date: inv.due_date.to_string(),
    }))
}

// ─── Invoice Send ────────────────────────────────────────────────────────────

#[derive(Debug, Serialize)]
pub struct SendInvoiceResponse {
    pub invoice_id: i64,
    pub status: String,
    pub message: String,
}

/// POST /api/v1/billing/invoices/:id/send
pub async fn send_invoice(
    State(state): State<Arc<AppState>>,
    user: UserContext,
    Path(id): Path<i64>,
) -> Result<Json<SendInvoiceResponse>, AppError> {
    require_permission(&user, "billing.invoice.send").map_err(|e| AppError::Forbidden(e.1))?;
    let inv = BillingService::send_invoice(&state.db, id).await?;
    if let Err(e) = crate::infrastructure::messaging::outbox::insert_outbox_event(
        &state.db,
        "invoice.sent",
        "invoice",
        inv.id,
        serde_json::json!({"invoice_id": inv.id, "invoice_number": inv.invoice_number}),
        None,
        Some(user.user_id),
        Some(inv.branch_id),
    )
    .await
    {
        tracing::error!(invoice_id = inv.id, error = %e, "Failed to publish invoice.sent event");
    }
    Ok(Json(SendInvoiceResponse {
        invoice_id: inv.id,
        status: "sent".to_string(),
        message: "Invoice sent to customer".to_string(),
    }))
}

// ─── Invoice Void ────────────────────────────────────────────────────────────

#[derive(Debug, Deserialize)]
pub struct VoidInvoiceRequest {
    pub reason: String,
}

/// POST /api/v1/billing/invoices/:id/void
pub async fn void_invoice(
    State(state): State<Arc<AppState>>,
    user: UserContext,
    Path(id): Path<i64>,
    Json(req): Json<VoidInvoiceRequest>,
) -> Result<Json<InvoiceResponse>, AppError> {
    require_permission(&user, "billing.invoice.void").map_err(|e| AppError::Forbidden(e.1))?;
    let inv = BillingService::void_invoice(&state.db, id, &req.reason).await?;
    if let Err(e) = crate::infrastructure::messaging::outbox::insert_outbox_event(
        &state.db,
        "invoice.voided",
        "invoice",
        inv.id,
        serde_json::json!({"invoice_id": inv.id, "reason": req.reason}),
        None,
        Some(user.user_id),
        Some(inv.branch_id),
    )
    .await
    {
        tracing::error!(invoice_id = inv.id, error = %e, "Failed to publish invoice.voided event");
    }
    Ok(Json(InvoiceResponse {
        id: inv.id,
        invoice_number: inv.invoice_number,
        customer_id: inv.customer_id,
        total_amount: inv.total_amount.to_string(),
        status: inv.status,
        due_date: inv.due_date.to_string(),
    }))
}

// ─── Refunds ─────────────────────────────────────────────────────────────────

#[derive(Debug, Serialize)]
pub struct RefundResponse {
    pub id: i64,
    pub refund_number: String,
    pub payment_id: i64,
    pub customer_id: i64,
    pub amount: String,
    pub reason: String,
    pub status: String,
}

#[derive(Debug, Deserialize)]
pub struct RequestRefundRequest {
    pub payment_id: i64,
    pub invoice_id: i64,
    pub customer_id: i64,
    pub amount: String,
    pub reason: String,
}

/// POST /api/v1/billing/refunds
pub async fn request_refund(
    State(state): State<Arc<AppState>>,
    user: UserContext,
    Json(req): Json<RequestRefundRequest>,
) -> Result<(StatusCode, Json<RefundResponse>), AppError> {
    require_permission(&user, "billing.invoice.refund")
        .map_err(|e| AppError::Forbidden(e.1))?;
    let amt: sea_orm::prelude::Decimal = req
        .amount
        .parse()
        .map_err(|_| AppError::Validation("Invalid amount".into()))?;
    let refund =
        BillingService::request_refund(&state.db, req.payment_id, req.invoice_id, req.customer_id, amt, req.reason, user.user_id).await?;
    Ok((
        StatusCode::CREATED,
        Json(RefundResponse {
            id: refund.id,
            refund_number: refund.refund_number,
            payment_id: refund.payment_id,
            customer_id: refund.customer_id,
            amount: refund.amount.to_string(),
            reason: refund.reason,
            status: refund.status,
        }),
    ))
}

/// PUT /api/v1/billing/refunds/:id/approve
pub async fn approve_refund(
    State(state): State<Arc<AppState>>,
    user: UserContext,
    Path(id): Path<i64>,
) -> Result<Json<RefundResponse>, AppError> {
    require_permission(&user, "billing.invoice.refund")
        .map_err(|e| AppError::Forbidden(e.1))?;
    let refund = BillingService::approve_refund(&state.db, id, user.user_id).await?;
    Ok(Json(RefundResponse {
        id: refund.id,
        refund_number: refund.refund_number,
        payment_id: refund.payment_id,
        customer_id: refund.customer_id,
        amount: refund.amount.to_string(),
        reason: refund.reason,
        status: refund.status,
    }))
}

/// PUT /api/v1/billing/refunds/:id/reject
pub async fn reject_refund(
    State(state): State<Arc<AppState>>,
    user: UserContext,
    Path(id): Path<i64>,
    Json(req): Json<VoidInvoiceRequest>,
) -> Result<Json<RefundResponse>, AppError> {
    require_permission(&user, "billing.invoice.refund")
        .map_err(|e| AppError::Forbidden(e.1))?;
    let refund = BillingService::reject_refund(&state.db, id, user.user_id, &req.reason).await?;
    Ok(Json(RefundResponse {
        id: refund.id,
        refund_number: refund.refund_number,
        payment_id: refund.payment_id,
        customer_id: refund.customer_id,
        amount: refund.amount.to_string(),
        reason: refund.reason,
        status: refund.status,
    }))
}

// ─── Discounts ───────────────────────────────────────────────────────────────

#[derive(Debug, Serialize)]
pub struct DiscountResponse {
    pub id: i64,
    pub name: String,
    pub code: Option<String>,
    pub discount_type: String,
    pub value: String,
    pub valid_from: String,
    pub valid_until: String,
    pub is_active: bool,
}

#[derive(Debug, Deserialize)]
pub struct CreateDiscountRequest {
    pub name: String,
    #[serde(default)]
    pub code: Option<String>,
    pub discount_type: String,
    pub value: String,
    pub valid_from: String,
    pub valid_until: String,
}

/// GET /api/v1/billing/discounts
pub async fn list_discounts(
    State(state): State<Arc<AppState>>,
    _user: UserContext,
) -> Result<Json<Vec<DiscountResponse>>, AppError> {
    let items = BillingService::list_discounts(&state.db).await?;
    Ok(Json(
        items
            .into_iter()
            .map(|d| DiscountResponse {
                id: d.id,
                name: d.name,
                code: d.code,
                discount_type: d.discount_type,
                value: d.value.to_string(),
                valid_from: d.valid_from.to_string(),
                valid_until: d.valid_until.to_string(),
                is_active: d.is_active,
            })
            .collect(),
    ))
}

/// POST /api/v1/billing/discounts
pub async fn create_discount(
    State(state): State<Arc<AppState>>,
    user: UserContext,
    Json(req): Json<CreateDiscountRequest>,
) -> Result<(StatusCode, Json<DiscountResponse>), AppError> {
    require_permission(&user, "billing.discount.create")
        .map_err(|e| AppError::Forbidden(e.1))?;
    let val: sea_orm::prelude::Decimal = req
        .value
        .parse()
        .map_err(|_| AppError::Validation("Invalid discount value".into()))?;
    let valid_from: chrono::NaiveDate = req
        .valid_from
        .parse()
        .map_err(|_| AppError::Validation("Invalid valid_from date".into()))?;
    let valid_until: chrono::NaiveDate = req
        .valid_until
        .parse()
        .map_err(|_| AppError::Validation("Invalid valid_until date".into()))?;
    let d = BillingService::create_discount(
        &state.db,
        req.name,
        req.code,
        req.discount_type,
        val,
        valid_from,
        valid_until,
        user.user_id,
    )
    .await?;
    Ok((
        StatusCode::CREATED,
        Json(DiscountResponse {
            id: d.id,
            name: d.name,
            code: d.code,
            discount_type: d.discount_type,
            value: d.value.to_string(),
            valid_from: d.valid_from.to_string(),
            valid_until: d.valid_until.to_string(),
            is_active: d.is_active,
        }),
    ))
}

// ─── Dunning Config ──────────────────────────────────────────────────────────

#[derive(Debug, Serialize)]
pub struct DunningConfigResponse {
    pub reminder_days: Vec<i32>,
    pub suspension_day: i32,
    pub termination_day: i32,
    pub late_fee_percent: String,
    pub late_fee_cap_percent: String,
    pub channels: Vec<String>,
}

/// GET /api/v1/billing/dunning/config
pub async fn get_dunning_config(
    State(state): State<Arc<AppState>>,
    _user: UserContext,
) -> Result<Json<DunningConfigResponse>, AppError> {
    let config = BillingService::get_dunning_config(&state.db).await?;
    Ok(Json(config))
}

// ─── Tax Config ──────────────────────────────────────────────────────────────

#[derive(Debug, Serialize)]
pub struct TaxConfigResponse {
    pub cgst_rate: String,
    pub sgst_rate: String,
    pub igst_rate: String,
    pub applicable_state: String,
    pub hsn_code: String,
    pub sac_code: String,
    pub tax_name: String,
}

/// GET /api/v1/billing/tax/config
pub async fn get_tax_config(
    State(state): State<Arc<AppState>>,
    _user: UserContext,
) -> Result<Json<TaxConfigResponse>, AppError> {
    let config = BillingService::get_tax_config(&state.db).await?;
    Ok(Json(config))
}
