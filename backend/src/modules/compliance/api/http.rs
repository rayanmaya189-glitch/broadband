use axum::extract::{Query, State};
use axum::http::StatusCode;
use axum::Json;
use serde::{Deserialize, Serialize};
use std::sync::Arc;

use crate::modules::compliance::application::services::ComplianceService;
use crate::shared::app_state::AppState;
use crate::shared::errors::AppError;
use crate::shared::middleware::auth::{require_permission, UserContext};
use crate::shared::primitives::PaginationParams;

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
    user: UserContext,
    Query(_p): Query<PaginationParams>,
) -> Result<Json<Vec<KycResponse>>, AppError> {
    require_permission(&user, "compliance.kyc.view").map_err(|e| AppError::Forbidden(e.1))?;
    let kycs = ComplianceService::list_kyc_verifications(&state.db).await?;
    Ok(Json(
        kycs.into_iter()
            .map(|k| KycResponse {
                id: k.id,
                customer_id: k.customer_id,
                document_type: k.document_type,
                status: k.status,
                provider: k.provider,
                verified_at: k.verified_at.map(|v| v.to_rfc3339()),
            })
            .collect(),
    ))
}

pub async fn create_kyc_verification(
    State(state): State<Arc<AppState>>,
    user: UserContext,
    Json(req): Json<CreateKycRequest>,
) -> Result<(StatusCode, Json<KycResponse>), AppError> {
    require_permission(&user, "compliance.kyc.create").map_err(|e| AppError::Forbidden(e.1))?;
    let kyc = ComplianceService::create_kyc_verification(
        &state.db,
        req.customer_id,
        req.document_type,
        req.document_number_hash,
        req.provider,
    )
    .await?;
    if let Err(e) = crate::infrastructure::messaging::outbox::insert_outbox_event(
        &state.db,
        "compliance.kyc.created",
        "kyc_verification",
        kyc.id,
        serde_json::json!({"kyc_id": kyc.id, "customer_id": kyc.customer_id, "status": kyc.status}),
        None,
        Some(user.user_id),
        user.branch_id,
    )
    .await
    {
        tracing::error!(error = %e, "Failed to publish compliance.kyc.created event");
    }
    Ok((
        StatusCode::CREATED,
        Json(KycResponse {
            id: kyc.id,
            customer_id: kyc.customer_id,
            document_type: kyc.document_type,
            status: kyc.status,
            provider: kyc.provider,
            verified_at: None,
        }),
    ))
}

pub async fn update_kyc_status(
    State(state): State<Arc<AppState>>,
    axum::extract::Path(id): axum::extract::Path<i64>,
    user: UserContext,
    Json(req): Json<UpdateKycStatusRequest>,
) -> Result<Json<KycResponse>, AppError> {
    require_permission(&user, "compliance.kyc.update").map_err(|e| AppError::Forbidden(e.1))?;
    let kyc = ComplianceService::update_kyc_status(
        &state.db,
        id,
        req.status,
        req.rejection_reason,
        req.provider_reference,
    )
    .await?;
    if let Err(e) = crate::infrastructure::messaging::outbox::insert_outbox_event(
        &state.db,
        "compliance.kyc.status.updated",
        "kyc_verification",
        kyc.id,
        serde_json::json!({"kyc_id": kyc.id, "status": kyc.status}),
        None,
        Some(user.user_id),
        user.branch_id,
    )
    .await
    {
        tracing::error!(error = %e, "Failed to publish compliance.kyc.status.updated event");
    }
    Ok(Json(KycResponse {
        id: kyc.id,
        customer_id: kyc.customer_id,
        document_type: kyc.document_type,
        status: kyc.status,
        provider: kyc.provider,
        verified_at: kyc.verified_at.map(|v| v.to_rfc3339()),
    }))
}

pub async fn get_customer_kyc(
    State(state): State<Arc<AppState>>,
    axum::extract::Path(customer_id): axum::extract::Path<i64>,
    _user: UserContext,
) -> Result<Json<Vec<KycResponse>>, AppError> {
    let kycs = ComplianceService::get_kyc_by_customer(&state.db, customer_id).await?;
    Ok(Json(
        kycs.into_iter()
            .map(|k| KycResponse {
                id: k.id,
                customer_id: k.customer_id,
                document_type: k.document_type,
                status: k.status,
                provider: k.provider,
                verified_at: k.verified_at.map(|v| v.to_rfc3339()),
            })
            .collect(),
    ))
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
    Ok(Json(
        consents
            .into_iter()
            .map(|c| ConsentResponse {
                id: c.id,
                customer_id: c.customer_id,
                consent_type: c.consent_type,
                granted: c.granted,
                collection_channel: c.collection_channel,
                revoked_at: c.revoked_at.map(|r| r.to_rfc3339()),
            })
            .collect(),
    ))
}

pub async fn grant_consent(
    State(state): State<Arc<AppState>>,
    user: UserContext,
    Json(req): Json<GrantConsentRequest>,
) -> Result<(StatusCode, Json<ConsentResponse>), AppError> {
    require_permission(&user, "compliance.consent.grant").map_err(|e| AppError::Forbidden(e.1))?;
    let consent_type = req.consent_type.clone();
    let c = ComplianceService::grant_consent(
        &state.db,
        req.customer_id,
        req.consent_type,
        req.collection_channel,
        req.ip_address,
        req.user_agent,
    )
    .await?;
    if let Err(e) = crate::infrastructure::messaging::outbox::insert_outbox_event(
        &state.db, "compliance.consent.granted", "consent", c.id,
        serde_json::json!({"consent_id": c.id, "customer_id": c.customer_id, "consent_type": consent_type}), None,
        Some(user.user_id), user.branch_id,
    ).await {
        tracing::error!(error = %e, "Failed to publish compliance.consent.granted event");
    }
    Ok((
        StatusCode::CREATED,
        Json(ConsentResponse {
            id: c.id,
            customer_id: c.customer_id,
            consent_type: c.consent_type,
            granted: c.granted,
            collection_channel: c.collection_channel,
            revoked_at: None,
        }),
    ))
}

pub async fn revoke_consent(
    State(state): State<Arc<AppState>>,
    user: UserContext,
    Json(req): Json<RevokeConsentRequest>,
) -> Result<StatusCode, AppError> {
    require_permission(&user, "compliance.consent.revoke").map_err(|e| AppError::Forbidden(e.1))?;
    let consent_type = req.consent_type.clone();
    ComplianceService::revoke_consent(&state.db, req.customer_id, req.consent_type).await?;
    if let Err(e) = crate::infrastructure::messaging::outbox::insert_outbox_event(
        &state.db,
        "compliance.consent.revoked",
        "consent",
        req.customer_id,
        serde_json::json!({"customer_id": req.customer_id, "consent_type": consent_type}),
        None,
        Some(user.user_id),
        user.branch_id,
    )
    .await
    {
        tracing::error!(error = %e, "Failed to publish compliance.consent.revoked event");
    }
    Ok(StatusCode::OK)
}

pub async fn check_consent(
    State(state): State<Arc<AppState>>,
    axum::extract::Path((customer_id, consent_type)): axum::extract::Path<(i64, String)>,
    _user: UserContext,
) -> Result<Json<serde_json::Value>, AppError> {
    let has = ComplianceService::has_consent(&state.db, customer_id, &consent_type).await?;
    Ok(Json(
        serde_json::json!({ "customer_id": customer_id, "consent_type": consent_type, "granted": has }),
    ))
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
    user: UserContext,
) -> Result<Json<Vec<RetentionPolicyResponse>>, AppError> {
    require_permission(&user, "compliance.retention.view").map_err(|e| AppError::Forbidden(e.1))?;
    let policies = ComplianceService::list_retention_policies(&state.db).await?;
    Ok(Json(
        policies
            .into_iter()
            .map(|p| RetentionPolicyResponse {
                id: p.id,
                entity_type: p.entity_type,
                retention_days: p.retention_days,
                action: p.action,
                is_active: p.is_active,
                description: p.description,
                legal_basis: p.legal_basis,
            })
            .collect(),
    ))
}

pub async fn create_retention_policy(
    State(state): State<Arc<AppState>>,
    user: UserContext,
    Json(req): Json<CreateRetentionPolicyRequest>,
) -> Result<(StatusCode, Json<RetentionPolicyResponse>), AppError> {
    require_permission(&user, "compliance.retention.create")
        .map_err(|e| AppError::Forbidden(e.1))?;
    let p = ComplianceService::create_retention_policy(
        &state.db,
        req.entity_type,
        req.retention_days,
        req.action,
        req.description,
        req.legal_basis,
    )
    .await?;
    if let Err(e) = crate::infrastructure::messaging::outbox::insert_outbox_event(
        &state.db,
        "compliance.retention.created",
        "retention_policy",
        p.id,
        serde_json::json!({"policy_id": p.id, "entity_type": p.entity_type}),
        None,
        Some(user.user_id),
        user.branch_id,
    )
    .await
    {
        tracing::error!(error = %e, "Failed to publish compliance.retention.created event");
    }
    Ok((
        StatusCode::CREATED,
        Json(RetentionPolicyResponse {
            id: p.id,
            entity_type: p.entity_type,
            retention_days: p.retention_days,
            action: p.action,
            is_active: p.is_active,
            description: p.description,
            legal_basis: p.legal_basis,
        }),
    ))
}

pub async fn update_retention_policy(
    State(state): State<Arc<AppState>>,
    axum::extract::Path(id): axum::extract::Path<i64>,
    user: UserContext,
    Json(req): Json<UpdateRetentionPolicyRequest>,
) -> Result<Json<RetentionPolicyResponse>, AppError> {
    require_permission(&user, "compliance.retention.update")
        .map_err(|e| AppError::Forbidden(e.1))?;
    let p = ComplianceService::update_retention_policy(
        &state.db,
        id,
        req.retention_days,
        req.action,
        req.is_active,
    )
    .await?;
    if let Err(e) = crate::infrastructure::messaging::outbox::insert_outbox_event(
        &state.db,
        "compliance.retention.updated",
        "retention_policy",
        p.id,
        serde_json::json!({"policy_id": p.id, "entity_type": p.entity_type}),
        None,
        Some(user.user_id),
        user.branch_id,
    )
    .await
    {
        tracing::error!(error = %e, "Failed to publish compliance.retention.updated event");
    }
    Ok(Json(RetentionPolicyResponse {
        id: p.id,
        entity_type: p.entity_type,
        retention_days: p.retention_days,
        action: p.action,
        is_active: p.is_active,
        description: p.description,
        legal_basis: p.legal_basis,
    }))
}
