/// OpenAPI schemas and stub handlers for Admin endpoints.
use serde::Serialize;
use utoipa::ToSchema;

// ── Request / Response types ─────────────────────────────────────────

#[derive(Debug, Serialize, ToSchema)]
pub struct SeedDataResponse {
    /// Status message
    pub message: String,
    /// Number of roles created
    pub roles_created: usize,
    /// Number of permissions created
    pub permissions_created: usize,
}

// ── Stub handler functions (for OpenAPI spec only) ───────────────────

/// Seed default roles and permissions into the database
#[utoipa::path(
    post,
    path = "/api/v1/admin/seed",
    tag = "Admin",
    responses(
        (status = 201, description = "Seed data created successfully", body = SeedDataResponse),
        (status = 401, description = "Unauthorized")
    ),
    security(("bearer_auth" = []))
)]
pub async fn seed_data() -> axum::Json<serde_json::Value> {
    unimplemented!()
}
