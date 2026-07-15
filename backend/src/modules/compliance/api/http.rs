use axum::extract::State;
use axum::http::StatusCode;
use axum::Json;
use serde::{Deserialize, Serialize};
use std::sync::Arc;

use crate::shared::app_state::AppState;
use crate::shared::errors::AppError;
use crate::shared::middleware::auth::UserContext;
use crate::modules::compliance::application::services::ComplianceService;

// ── KYC Verifications ──

#[derive(Debug, Serialize)]
pub struct KycResponse {
    pub id: i64,
    pub customer_id: i64,
    pub document_type: String,
    pub status: String,
    pub provider: Option<String>,
    pub verified_at: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct CreateKycRequest {
    pub customer_id: i64,
    pub document_type: String,
    pub document_number_hash: String,
    #[serde(default)]
    pub provider: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct UpdateKycStatusRequest {
    pub status: String,
    #[serde(default)]
    pub rejection_reason: Option<String>,
    #[serde(default)]
    pub provider_reference: Option<String>,
}

pub async fn list_kyc_verifications(
    State(state): State<Arc<AppState>>,
    _user: UserContext,
) -> Result<Json<Vec<KycResponse>>, AppError> {
    let kycs = ComplianceService::list_kyc_verifications(&state.db).await?;
    Ok(Json(kycs.into_iter().map(|k| KycResponse {
        id: k.id, customer_id: k.customer_id, document_type: k.document_type,
        status: k.status, provider: k.provider,
        verified_at: k.verified_at.map(|v| v.to_rfc3339()),
    }).collect()))
}

pub async fn create_kyc_verification(
    State(state): State<Arc<AppState>>,
    _user: UserContext,
    Json(req): Json<CreateKycRequest>,
) -> Result<(StatusCode, Json<KycResponse>), AppError> {
    let kyc = ComplianceService::create_kyc_verification(
        &state.db, req.customer_id, req.document_type, req.document_number_hash, req.provider,
    ).await?;
    Ok((StatusCode::CREATED, Json(KycResponse {
        id: kyc.id, customer_id: kyc.customer_id, document_type: kyc.document_type,
        status: kyc.status, provider: kyc.provider, verified_at: None,
    })))
}

pub async fn update_kyc_status(
    State(state): State<Arc<AppState>>,
    axum::extract::Path(id): axum::extract::Path<i64>,
    _user: UserContext,
    Json(req): Json<UpdateKycStatusRequest>,
) -> Result<Json<KycResponse>, AppError> {
    let kyc = ComplianceService::update_kyc_status(
        &state.db, id, req.status, req.rejection_reason, req.provider_reference,
    ).await?;
    Ok(Json(KycResponse {
        id: kyc.id, customer_id: kyc.customer_id, document_type: kyc.document_type,
        status: kyc.status, provider: kyc.provider,
        verified_at: kyc.verified_at.map(|v| v.to_rfc3339()),
    }))
}

pub async fn get_customer_kyc(
    State(state): State<Arc<AppState>>,
    axum::extract::Path(customer_id): axum::extract::Path<i64>,
    _user: UserContext,
) -> Result<Json<Vec<KycResponse>>, AppError> {
    let kycs = ComplianceService::get_kyc_by_customer(&state.db, customer_id).await?;
    Ok(Json(kycs.into_iter().map(|k| KycResponse {
        id: k.id, customer_id: k.customer_id, document_type: k.document_type,
        status: k.status, provider: k.provider,
        verified_at: k.verified_at.map(|v| v.to_rfc3339()),
    }).collect()))
}

// ── Consent Management ──

#[derive(Debug, Serialize)]
pub struct ConsentResponse {
    pub id: i64,
    pub customer_id: i64,
    pub consent_type: String,
    pub granted: bool,
    pub collection_channel: String,
    pub revoked_at: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct GrantConsentRequest {
    pub customer_id: i64,
    pub consent_type: String,
    pub collection_channel: String,
    #[serde(default)]
    pub ip_address: Option<String>,
    #[serde(default)]
    pub user_agent: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct RevokeConsentRequest {
    pub customer_id: i64,
    pub consent_type: String,
}

pub async fn list_consents(
    State(state): State<Arc<AppState>>,
    axum::extract::Path(customer_id): axum::extract::Path<i64>,
    _user: UserContext,
) -> Result<Json<Vec<ConsentResponse>>, AppError> {
    let consents = ComplianceService::list_consents(&state.db, customer_id).await?;
    Ok(Json(consents.into_iter().map(|c| ConsentResponse {
        id: c.id, customer_id: c.customer_id, consent_type: c.consent_type,
        granted: c.granted, collection_channel: c.collection_channel,
        revoked_at: c.revoked_at.map(|r| r.to_rfc3339()),
    }).collect()))
}

pub async fn grant_consent(
    State(state): State<Arc<AppState>>,
    _user: UserContext,
    Json(req): Json<GrantConsentRequest>,
) -> Result<(StatusCode, Json<ConsentResponse>), AppError> {
    let c = ComplianceService::grant_consent(
        &state.db, req.customer_id, req.consent_type, req.collection_channel,
        req.ip_address, req.user_agent,
    ).await?;
    Ok((StatusCode::CREATED, Json(ConsentResponse {
        id: c.id, customer_id: c.customer_id, consent_type: c.consent_type,
        granted: c.granted, collection_channel: c.collection_channel, revoked_at: None,
    })))
}

pub async fn revoke_consent(
    State(state): State<Arc<AppState>>,
    _user: UserContext,
    Json(req): Json<RevokeConsentRequest>,
) -> Result<StatusCode, AppError> {
    ComplianceService::revoke_consent(&state.db, req.customer_id, req.consent_type).await?;
    Ok(StatusCode::OK)
}

pub async fn check_consent(
    State(state): State<Arc<AppState>>,
    axum::extract::Path((customer_id, consent_type)): axum::extract::Path<(i64, String)>,
    _user: UserContext,
) -> Result<Json<serde_json::Value>, AppError> {
    let has = ComplianceService::has_consent(&state.db, customer_id, &consent_type).await?;
    Ok(Json(serde_json::json!({ "customer_id": customer_id, "consent_type": consent_type, "granted": has })))
}

// ── Data Retention Policies ──

#[derive(Debug, Serialize)]
pub struct RetentionPolicyResponse {
    pub id: i64,
    pub entity_type: String,
    pub retention_days: i32,
    pub action: String,
    pub is_active: bool,
    pub description: Option<String>,
    pub legal_basis: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct CreateRetentionPolicyRequest {
    pub entity_type: String,
    pub retention_days: i32,
    pub action: String,
    #[serde(default)]
    pub description: Option<String>,
    #[serde(default)]
    pub legal_basis: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct UpdateRetentionPolicyRequest {
    #[serde(default)]
    pub retention_days: Option<i32>,
    #[serde(default)]
    pub action: Option<String>,
    #[serde(default)]
    pub is_active: Option<bool>,
}

pub async fn list_retention_policies(
    State(state): State<Arc<AppState>>,
    _user: UserContext,
) -> Result<Json<Vec<RetentionPolicyResponse>>, AppError> {
    let policies = ComplianceService::list_retention_policies(&state.db).await?;
    Ok(Json(policies.into_iter().map(|p| RetentionPolicyResponse {
        id: p.id, entity_type: p.entity_type, retention_days: p.retention_days,
        action: p.action, is_active: p.is_active, description: p.description, legal_basis: p.legal_basis,
    }).collect()))
}

pub async fn create_retention_policy(
    State(state): State<Arc<AppState>>,
    _user: UserContext,
    Json(req): Json<CreateRetentionPolicyRequest>,
) -> Result<(StatusCode, Json<RetentionPolicyResponse>), AppError> {
    let p = ComplianceService::create_retention_policy(
        &state.db, req.entity_type, req.retention_days, req.action, req.description, req.legal_basis,
    ).await?;
    Ok((StatusCode::CREATED, Json(RetentionPolicyResponse {
        id: p.id, entity_type: p.entity_type, retention_days: p.retention_days,
        action: p.action, is_active: p.is_active, description: p.description, legal_basis: p.legal_basis,
    })))
}

pub async fn update_retention_policy(
    State(state): State<Arc<AppState>>,
    axum::extract::Path(id): axum::extract::Path<i64>,
    _user: UserContext,
    Json(req): Json<UpdateRetentionPolicyRequest>,
) -> Result<Json<RetentionPolicyResponse>, AppError> {
    let p = ComplianceService::update_retention_policy(&state.db, id, req.retention_days, req.action, req.is_active).await?;
    Ok(Json(RetentionPolicyResponse {
        id: p.id, entity_type: p.entity_type, retention_days: p.retention_days,
        action: p.action, is_active: p.is_active, description: p.description, legal_basis: p.legal_basis,
    }))
}
