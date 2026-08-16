use sea_orm::prelude::Decimal;
use serde::{Deserialize, Serialize};

/// Branch ID type
pub type BranchId = i64;

/// User ID type
pub type UserId = i64;

/// Client IP address extracted from request headers (X-Forwarded-For, X-Real-IP)
#[derive(Debug, Clone)]
pub struct ClientIp(pub String);

impl ClientIp {
    pub fn as_str(&self) -> &str {
        &self.0
    }

    /// Extract client IP from request headers.
    ///
    /// SECURITY: X-Forwarded-For / X-Real-IP are only honored when TRUST_PROXY=true,
    /// matching the rate limiter, so a client cannot spoof its own IP in
    /// audit/anomaly signals when the service is exposed directly.
    pub fn from_headers(headers: &axum::http::HeaderMap) -> Self {
        let trust_proxy = std::env::var("TRUST_PROXY")
            .map(|v| v.eq_ignore_ascii_case("true") || v == "1")
            .unwrap_or(false);
        if !trust_proxy {
            return ClientIp("unknown".to_string());
        }
        let ip = headers
            .get("x-forwarded-for")
            .and_then(|v| v.to_str().ok())
            .map(|v| v.split(',').next().unwrap_or(v).trim().to_string())
            .or_else(|| {
                headers
                    .get("x-real-ip")
                    .and_then(|v| v.to_str().ok())
                    .map(|s| s.trim().to_string())
            })
            .unwrap_or_else(|| "unknown".to_string());
        ClientIp(ip)
    }

    /// Best-effort client IP: the real peer socket address is authoritative;
    /// forwarding headers are only consulted when TRUST_PROXY=true.
    pub fn from_peer(peer: Option<std::net::SocketAddr>, headers: &axum::http::HeaderMap) -> Self {
        match peer {
            Some(addr) => ClientIp(addr.ip().to_string()),
            None => Self::from_headers(headers),
        }
    }
}

/// Customer ID type
pub type CustomerId = i64;

/// Subscription ID type
pub type SubscriptionId = i64;

/// Plan ID type
pub type PlanId = i64;

/// Invoice ID type
pub type InvoiceId = i64;

/// Payment ID type
pub type PaymentId = i64;

/// Ticket ID type
pub type TicketId = i64;

/// Device ID type
pub type DeviceId = i64;

/// Generic pagination request
#[derive(Debug, Clone, Deserialize)]
pub struct PaginationParams {
    pub page: Option<u64>,
    pub limit: Option<u64>,
}

impl PaginationParams {
    pub fn offset(&self) -> u64 {
        let page = self.page.unwrap_or(1).max(1);
        let limit = self.limit.unwrap_or(20).min(100);
        (page - 1) * limit
    }

    pub fn limit(&self) -> u64 {
        self.limit.unwrap_or(20).min(100)
    }

    pub fn page(&self) -> u64 {
        self.page.unwrap_or(1).max(1)
    }
}

/// Paginated response wrapper
#[derive(Debug, Serialize)]
pub struct PaginatedResponse<T: Serialize> {
    pub items: Vec<T>,
    pub total: u64,
    pub page: u64,
    pub limit: u64,
    pub total_pages: u64,
}

impl<T: Serialize> PaginatedResponse<T> {
    pub fn new(items: Vec<T>, total: u64, page: u64, limit: u64) -> Self {
        Self {
            items,
            total,
            page,
            limit,
            total_pages: (total as f64 / limit as f64).ceil() as u64,
        }
    }
}

/// Status enum for customer lifecycle
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum CustomerStatus {
    Registered,
    KycPending,
    KycVerified,
    InstallationScheduled,
    InstallationInProgress,
    Active,
    Suspended,
    Terminated,
}

impl std::fmt::Display for CustomerStatus {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let s = match self {
            Self::Registered => "registered",
            Self::KycPending => "kyc_pending",
            Self::KycVerified => "kyc_verified",
            Self::InstallationScheduled => "installation_scheduled",
            Self::InstallationInProgress => "installation_in_progress",
            Self::Active => "active",
            Self::Suspended => "suspended",
            Self::Terminated => "terminated",
        };
        write!(f, "{}", s)
    }
}

/// Status enum for subscription lifecycle
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum SubscriptionStatus {
    Pending,
    Active,
    Suspended,
    Cancelled,
    Expired,
}

impl std::fmt::Display for SubscriptionStatus {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let s = match self {
            Self::Pending => "pending",
            Self::Active => "active",
            Self::Suspended => "suspended",
            Self::Cancelled => "cancelled",
            Self::Expired => "expired",
        };
        write!(f, "{}", s)
    }
}

/// Status enum for invoice lifecycle
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum InvoiceStatus {
    Draft,
    Pending,
    Sent,
    Paid,
    Partial,
    Overdue,
    Void,
    Refunded,
}

/// Pro-rata adjustment calculation
#[derive(Debug, Clone, Serialize)]
pub struct ProRataAdjustment {
    pub old_plan_credit: Decimal,
    pub new_plan_charge: Decimal,
    pub adjustment: Decimal,
}

/// Calculate pro-rata adjustment for plan changes mid-billing cycle
pub fn calculate_pro_rata(
    old_plan_price: Decimal,
    new_plan_price: Decimal,
    billing_period_days: i32,
    days_used: i32,
) -> ProRataAdjustment {
    let remaining_days = (billing_period_days - days_used).max(0) as f64;
    let billing_days = billing_period_days.max(1) as f64;

    let old_daily = old_plan_price / Decimal::from(billing_days as i64);
    let new_daily = new_plan_price / Decimal::from(billing_days as i64);
    let credit = old_daily * Decimal::from(remaining_days as i64);
    let charge = new_daily * Decimal::from(remaining_days as i64);
    let adjustment = charge - credit;

    ProRataAdjustment {
        old_plan_credit: credit,
        new_plan_charge: charge,
        adjustment,
    }
}
