/// OpenAPI schemas and stub handlers for Referral endpoints.
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

// ── Request / Response types ─────────────────────────────────────────

#[derive(Debug, Deserialize, ToSchema)]
pub struct CreateReferralRequest {
    /// Referral program ID
    pub program_id: i64,
    /// Referee phone number
    pub referee_phone: String,
    /// Referral code to attribute
    pub referral_code: String,
}

#[derive(Debug, Serialize, ToSchema)]
pub struct ReferralResponse {
    /// Referral ID
    pub id: i64,
    /// Referral code used
    pub referral_code: String,
    /// Referral status
    pub status: String,
    /// Referee phone number
    pub referee_phone: String,
}

#[derive(Debug, Serialize, ToSchema)]
pub struct WalletResponse {
    /// Wallet ID
    pub id: i64,
    /// Customer ID
    pub customer_id: i64,
    /// Current balance
    pub balance: String,
    /// Total amount earned
    pub total_earned: String,
}

#[derive(Debug, Serialize, ToSchema)]
pub struct ReferralStatsResponse {
    /// User's referral code
    pub referral_code: String,
    /// Total times shared
    pub total_shared: i64,
    /// Total registered via referral
    pub total_registered: i64,
    /// Total currently active
    pub total_active: i64,
    /// Total rewarded
    pub total_rewarded: i64,
    /// Total reward amount earned
    pub total_reward_amount: String,
    /// Current wallet balance
    pub wallet_balance: String,
    /// Total wallet earned
    pub wallet_total_earned: String,
}

#[derive(Debug, Deserialize, ToSchema)]
pub struct CreateProgramRequest {
    /// Program name
    pub name: String,
    /// Reward type (e.g. "fixed", "percentage")
    pub reward_type: String,
    /// Reward value
    pub reward_value: String,
    /// Max referrals per user (optional)
    #[serde(default)]
    pub max_referrals_per_user: Option<i32>,
    /// Valid from date (YYYY-MM-DD)
    pub valid_from: String,
    /// Valid until date (YYYY-MM-DD)
    pub valid_until: String,
}

#[derive(Debug, Deserialize, ToSchema)]
pub struct UpdateProgramRequest {
    /// Program name
    #[serde(default)]
    pub name: Option<String>,
    /// Reward type
    #[serde(default)]
    pub reward_type: Option<String>,
    /// Reward value
    #[serde(default)]
    pub reward_value: Option<String>,
    /// Max referrals per user
    #[serde(default)]
    pub max_referrals_per_user: Option<Option<i32>>,
    /// Valid from date
    #[serde(default)]
    pub valid_from: Option<String>,
    /// Valid until date
    #[serde(default)]
    pub valid_until: Option<String>,
}

#[derive(Debug, Serialize, ToSchema)]
pub struct ReferralProgramResponse {
    /// Program ID
    pub id: i64,
    /// Program name
    pub name: String,
    /// Reward type
    pub reward_type: String,
    /// Reward value
    pub reward_value: String,
    /// Max referrals per user
    pub max_referrals_per_user: Option<i32>,
    /// Valid from date
    pub valid_from: String,
    /// Valid until date
    pub valid_until: String,
    /// Whether program is active
    pub is_active: bool,
}

#[derive(Debug, Deserialize, ToSchema)]
pub struct ShareReferralRequest {
    /// Phone number of the person being referred
    pub referred_phone: String,
    /// Channel used to share (e.g. "sms", "whatsapp")
    pub channel: String,
}

#[derive(Debug, Deserialize, ToSchema)]
pub struct AdjustWalletRequest {
    /// Amount to adjust (positive = credit, negative = debit)
    pub amount: String,
    /// Reason for adjustment
    pub reason: String,
}

#[derive(Debug, Serialize, ToSchema)]
pub struct WalletTransactionResponse {
    /// Transaction ID
    pub id: i64,
    /// Wallet ID
    pub wallet_id: i64,
    /// Transaction type
    pub transaction_type: String,
    /// Amount
    pub amount: String,
    /// Description
    pub description: String,
    /// Created at timestamp
    pub created_at: String,
}

#[derive(Debug, Serialize, ToSchema)]
pub struct ReferralAnalyticsResponse {
    /// Total referrals
    pub total_referrals: i64,
    /// Total shared
    pub total_shared: i64,
    /// Total active
    pub total_active: i64,
    /// Total rewarded
    pub total_rewarded: i64,
    /// Conversion rate
    pub conversion_rate: String,
    /// Total rewards paid
    pub total_rewards_paid: String,
}

// ── Stub handler functions (for OpenAPI spec only) ───────────────────

/// List all referrals (admin)
#[utoipa::path(
    get,
    path = "/api/v1/referrals",
    tag = "Referrals",
    params(("page" = Option<u64>, Query, description = "Page number"),
           ("limit" = Option<u64>, Query, description = "Items per page")),
    responses(
        (status = 200, description = "Paginated list of referrals"),
        (status = 403, description = "Forbidden")
    ),
    security(("bearer_auth" = []))
)]
pub async fn list_referrals() -> axum::Json<serde_json::Value> {
    unimplemented!()
}

/// Create a referral (admin)
#[utoipa::path(
    post,
    path = "/api/v1/referrals",
    tag = "Referrals",
    request_body = CreateReferralRequest,
    responses(
        (status = 201, description = "Referral created", body = ReferralResponse),
        (status = 403, description = "Forbidden"),
        (status = 422, description = "Validation error")
    ),
    security(("bearer_auth" = []))
)]
pub async fn create_referral() -> axum::Json<serde_json::Value> {
    unimplemented!()
}

/// Get authenticated user's wallet
#[utoipa::path(
    get,
    path = "/api/v1/referrals/wallet",
    tag = "Referrals",
    responses(
        (status = 200, description = "Wallet details", body = WalletResponse)
    ),
    security(("bearer_auth" = []))
)]
pub async fn get_wallet() -> axum::Json<serde_json::Value> {
    unimplemented!()
}

/// Get authenticated user's referral code and summary
#[utoipa::path(
    get,
    path = "/api/v1/referrals/my-code",
    tag = "Referrals",
    responses(
        (status = 200, description = "Referral code and summary")
    ),
    security(("bearer_auth" = []))
)]
pub async fn get_my_referral_code() -> axum::Json<serde_json::Value> {
    unimplemented!()
}

/// List referrals made by the authenticated user
#[utoipa::path(
    get,
    path = "/api/v1/referrals/my-referrals",
    tag = "Referrals",
    responses(
        (status = 200, description = "List of user's referrals")
    ),
    security(("bearer_auth" = []))
)]
pub async fn list_my_referrals() -> axum::Json<serde_json::Value> {
    unimplemented!()
}

/// Share a referral with another person
#[utoipa::path(
    post,
    path = "/api/v1/referrals/share",
    tag = "Referrals",
    request_body = ShareReferralRequest,
    responses(
        (status = 201, description = "Referral shared"),
        (status = 403, description = "Forbidden")
    ),
    security(("bearer_auth" = []))
)]
pub async fn share_referral() -> axum::Json<serde_json::Value> {
    unimplemented!()
}

/// Get referral statistics for the authenticated user
#[utoipa::path(
    get,
    path = "/api/v1/referrals/stats",
    tag = "Referrals",
    responses(
        (status = 200, description = "Referral statistics", body = ReferralStatsResponse)
    ),
    security(("bearer_auth" = []))
)]
pub async fn get_referral_stats() -> axum::Json<serde_json::Value> {
    unimplemented!()
}

/// List referral programs (admin)
#[utoipa::path(
    get,
    path = "/api/v1/referrals/programs",
    tag = "Referrals",
    params(("page" = Option<u64>, Query, description = "Page number"),
           ("limit" = Option<u64>, Query, description = "Items per page")),
    responses(
        (status = 200, description = "Paginated list of programs"),
        (status = 403, description = "Forbidden")
    ),
    security(("bearer_auth" = []))
)]
pub async fn list_programs() -> axum::Json<serde_json::Value> {
    unimplemented!()
}

/// Create a referral program (admin)
#[utoipa::path(
    post,
    path = "/api/v1/referrals/programs",
    tag = "Referrals",
    request_body = CreateProgramRequest,
    responses(
        (status = 201, description = "Program created", body = ReferralProgramResponse),
        (status = 403, description = "Forbidden"),
        (status = 422, description = "Validation error")
    ),
    security(("bearer_auth" = []))
)]
pub async fn create_program() -> axum::Json<serde_json::Value> {
    unimplemented!()
}

/// Update a referral program (admin)
#[utoipa::path(
    put,
    path = "/api/v1/referrals/programs/{id}",
    tag = "Referrals",
    params(("id" = i64, Path, description = "Program ID")),
    request_body = UpdateProgramRequest,
    responses(
        (status = 200, description = "Program updated", body = ReferralProgramResponse),
        (status = 403, description = "Forbidden"),
        (status = 404, description = "Program not found")
    ),
    security(("bearer_auth" = []))
)]
pub async fn update_program() -> axum::Json<serde_json::Value> {
    unimplemented!()
}

/// Delete a referral program (admin)
#[utoipa::path(
    delete,
    path = "/api/v1/referrals/programs/{id}",
    tag = "Referrals",
    params(("id" = i64, Path, description = "Program ID")),
    responses(
        (status = 204, description = "Program deleted"),
        (status = 403, description = "Forbidden"),
        (status = 404, description = "Program not found")
    ),
    security(("bearer_auth" = []))
)]
pub async fn delete_program() -> axum::Json<serde_json::Value> {
    unimplemented!()
}

/// Get referral analytics (admin)
#[utoipa::path(
    get,
    path = "/api/v1/referrals/analytics",
    tag = "Referrals",
    responses(
        (status = 200, description = "Referral analytics", body = ReferralAnalyticsResponse),
        (status = 403, description = "Forbidden")
    ),
    security(("bearer_auth" = []))
)]
pub async fn get_analytics() -> axum::Json<serde_json::Value> {
    unimplemented!()
}

/// List all wallets (admin)
#[utoipa::path(
    get,
    path = "/api/v1/referrals/wallets",
    tag = "Referrals",
    params(("page" = Option<u64>, Query, description = "Page number"),
           ("limit" = Option<u64>, Query, description = "Items per page")),
    responses(
        (status = 200, description = "Paginated list of wallets"),
        (status = 403, description = "Forbidden")
    ),
    security(("bearer_auth" = []))
)]
pub async fn list_wallets() -> axum::Json<serde_json::Value> {
    unimplemented!()
}

/// Adjust a wallet balance (admin)
#[utoipa::path(
    post,
    path = "/api/v1/referrals/wallets/{id}/adjust",
    tag = "Referrals",
    params(("id" = i64, Path, description = "Wallet ID")),
    request_body = AdjustWalletRequest,
    responses(
        (status = 201, description = "Wallet adjusted", body = WalletTransactionResponse),
        (status = 403, description = "Forbidden"),
        (status = 404, description = "Wallet not found")
    ),
    security(("bearer_auth" = []))
)]
pub async fn adjust_wallet() -> axum::Json<serde_json::Value> {
    unimplemented!()
}
