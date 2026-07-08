use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

/// Current user context extracted from a valid JWT in middleware.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UserContext {
    pub user_id: i64,
    pub email: String,
    pub role: String,
    pub branch_id: Option<i64>,
    pub is_company_wide: bool,
    pub permissions: Vec<String>,
}

/// Branch filter injected by the branch-scoping middleware.
#[derive(Debug, Clone)]
pub struct BranchFilter {
    pub branch_ids: Vec<i64>,
    pub is_company_wide: bool,
}

/// Paginated list request.
#[derive(Debug, Deserialize)]
pub struct PaginationParams {
    #[serde(default = "default_page")]
    pub page: u32,
    #[serde(default = "default_limit")]
    pub limit: u32,
    pub sort_by: Option<String>,
    pub sort_order: Option<String>,
    pub search: Option<String>,
}

fn default_page() -> u32 {
    1
}
fn default_limit() -> u32 {
    25
}

impl PaginationParams {
    pub fn offset(&self) -> u32 {
        (self.page.saturating_sub(1)) * self.limit
    }

    pub fn limit_i64(&self) -> i64 {
        self.limit.min(100) as i64
    }
}

/// Paginated list response wrapper.
#[derive(Debug, Serialize)]
pub struct PaginatedResponse<T: Serialize> {
    pub data: Vec<T>,
    pub total: i64,
    pub page: u32,
    pub limit: u32,
    pub total_pages: u32,
}

/// Audit metadata carried through middleware.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AuditMeta {
    pub user_id: Option<i64>,
    pub branch_id: Option<i64>,
    pub ip_address: Option<String>,
    pub user_agent: Option<String>,
    pub action: String,
    pub entity_type: Option<String>,
    pub entity_id: Option<String>,
    pub timestamp: DateTime<Utc>,
}
