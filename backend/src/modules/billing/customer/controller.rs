use axum::extract::{Json, Path, State};

use crate::app::SharedState;
use crate::common::errors::app_error::AppError;
use crate::common::middleware::auth_middleware::UserContext;
use crate::modules::billing::request::billing_request::{InvoiceQuery, PaymentQuery};
use crate::modules::billing::response::billing_response::*;
use crate::modules::billing::service::billing_service::BillingService;

/// Get current customer's invoices.
pub async fn get_my_invoices(
    State(state): State<SharedState>,
    user: UserContext,
) -> Result<Json<InvoiceListResponse>, AppError> {
    let svc = BillingService::new(&state.db_seaorm);
    let query = InvoiceQuery {
        status: None,
        customer_id: Some(user.user_id),
        branch_id: None,
        page: None,
        per_page: None,
    };
    Ok(Json(svc.list_invoices(query).await?))
}

/// Get specific invoice (customer: only own).
pub async fn get_invoice(
    State(state): State<SharedState>,
    user: UserContext,
    Path(id): Path<i64>,
) -> Result<Json<InvoiceResponse>, AppError> {
    let svc = BillingService::new(&state.db_seaorm);
    let invoice = svc.get_invoice(id).await?;
    if invoice.customer_id != user.user_id {
        return Err(AppError::Forbidden("Access denied".into()));
    }
    Ok(Json(invoice))
}

/// Get line items for own invoice.
pub async fn get_line_items(
    State(state): State<SharedState>,
    user: UserContext,
    Path(id): Path<i64>,
) -> Result<Json<Vec<InvoiceLineItemResponse>>, AppError> {
    let svc = BillingService::new(&state.db_seaorm);
    let invoice = svc.get_invoice(id).await?;
    if invoice.customer_id != user.user_id {
        return Err(AppError::Forbidden("Access denied".into()));
    }
    Ok(Json(svc.get_line_items(id).await?))
}

/// Get my payment history.
pub async fn get_my_payments(
    State(state): State<SharedState>,
    user: UserContext,
) -> Result<Json<PaymentListResponse>, AppError> {
    let svc = BillingService::new(&state.db_seaorm);
    let query = PaymentQuery {
        status: None,
        customer_id: Some(user.user_id),
        branch_id: None,
        page: None,
        per_page: None,
    };
    Ok(Json(svc.list_payments(query).await?))
}

/// Make a payment (customer self-service).
pub async fn make_payment(
    State(state): State<SharedState>,
    user: UserContext,
    Json(req): Json<crate::modules::billing::request::billing_request::RecordPaymentRequest>,
) -> Result<Json<PaymentResponse>, AppError> {
    let svc = BillingService::new(&state.db_seaorm);
    // Verify the invoice belongs to this customer
    let invoice = svc.get_invoice(req.invoice_id).await?;
    if invoice.customer_id != user.user_id {
        return Err(AppError::Forbidden("Access denied".into()));
    }
    Ok(Json(svc.record_payment(req).await?))
}
