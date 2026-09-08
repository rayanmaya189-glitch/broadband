use axum::extract::State;
use axum::http::StatusCode;
use axum::Json;
use sea_orm::{
    ActiveModelTrait, ColumnTrait, ConnectionTrait, DatabaseBackend, EntityTrait, QueryFilter, Set,
};
use std::sync::Arc;

use crate::shared::app_state::AppState;
use crate::shared::errors::AppError;
use crate::shared::middleware::auth::{require_permission, UserContext};

#[derive(Debug, serde::Serialize)]
pub struct SeedResponse {
    pub message: String,
    pub roles_created: usize,
    pub permissions_created: usize,
}

/// POST /api/v1/admin/seed
/// SECURITY: Requires admin.data.manage permission. Only super_admin should call this.
pub async fn seed_data(
    State(state): State<Arc<AppState>>,
    user: UserContext,
) -> Result<(StatusCode, Json<SeedResponse>), AppError> {
    // Require explicit permission — prevents accidental or malicious data seeding
    require_permission(&user, "admin.data.manage").map_err(|(_, msg)| AppError::Forbidden(msg))?;
    use crate::modules::security::domain::entities::{
        Permission, PermissionColumn, Role, RoleColumn,
    };

    let default_roles = vec![
        ("Super Administrator", "super_admin", "Full system access"),
        ("Administrator", "admin", "Administrative access"),
        ("Manager", "manager", "Branch manager access"),
        ("Operator", "operator", "Basic operator access"),
        ("Technician", "technician", "Field technician access"),
        ("Finance Manager", "finance_manager", "Financial operations"),
        ("Billing Operator", "billing_operator", "Billing operations"),
        ("Support Agent", "support_agent", "Customer support"),
    ];

    let mut roles_created = 0;
    for (name, slug, description) in default_roles {
        let exists = Role::find()
            .filter(RoleColumn::Slug.eq(slug))
            .one(&state.db)
            .await?;
        if exists.is_none() {
            let now = chrono::Utc::now();
            let role = crate::modules::security::domain::entities::role::ActiveModel {
                name: Set(name.to_string()),
                slug: Set(slug.to_string()),
                description: Set(Some(description.to_string())),
                is_system: Set(true),
                is_active: Set(true),
                is_company_wide: Set(true),
                parent_role_id: Set(None),
                created_at: Set(now),
                updated_at: Set(now),
                ..Default::default()
            };
            if role.insert(&state.db).await.is_ok() {
                roles_created += 1;
            }
        }
    }

    let default_permissions = vec![
        ("customer", "account", "view", "View customers"),
        ("customer", "account", "create", "Create customers"),
        ("customer", "account", "update", "Update customers"),
        ("customer", "account", "delete", "Delete customers"),
        ("customer", "address", "create", "Create addresses"),
        ("billing", "invoice", "view", "View invoices"),
        ("billing", "invoice", "create", "Create invoices"),
        ("billing", "invoice", "send", "Send invoices"),
        ("billing", "invoice", "void", "Void invoices"),
        ("billing", "invoice", "refund", "Process refunds"),
        ("billing", "discount", "create", "Create discounts"),
        ("billing", "payment", "record", "Record payments"),
        ("branch", "branch", "view", "View branches"),
        ("branch", "branch", "create", "Create branches"),
        ("branch", "branch", "update", "Update branches"),
        ("branch", "branch", "delete", "Delete branches"),
        ("subscription", "subscription", "view", "View subscriptions"),
        (
            "subscription",
            "subscription",
            "manage",
            "Manage subscriptions",
        ),
        ("device", "device", "view", "View devices"),
        ("device", "device", "manage", "Manage devices"),
        ("network", "vlan", "view", "View VLANs"),
        ("network", "vlan", "create", "Create VLANs"),
        ("network", "vlan", "delete", "Delete VLANs"),
        ("network", "ippool", "view", "View IP pools"),
        ("network", "ippool", "create", "Create IP pools"),
        ("network", "pppoe", "view", "View PPPoE sessions"),
        ("network", "pppoe", "create", "Create PPPoE sessions"),
        ("network", "pppoe", "terminate", "Terminate PPPoE sessions"),
        ("network", "mac_binding", "view", "View MAC bindings"),
        ("network", "mac_binding", "create", "Create MAC bindings"),
        (
            "bandwidth",
            "profile",
            "create",
            "Create bandwidth profiles",
        ),
        (
            "bandwidth",
            "profile",
            "update",
            "Update bandwidth profiles",
        ),
        (
            "bandwidth",
            "profile",
            "delete",
            "Delete bandwidth profiles",
        ),
        ("ticket", "ticket", "view", "View tickets"),
        ("ticket", "ticket", "create", "Create tickets"),
        ("ticket", "ticket", "manage", "Manage tickets"),
        ("plan", "plan", "view", "View plans"),
        ("plan", "plan", "manage", "Manage plans"),
        ("accounting", "accounts", "create", "Create accounts"),
        ("accounting", "accounts", "update", "Update accounts"),
        ("accounting", "journal", "create", "Create journal entries"),
        ("accounting", "journal", "post", "Post journal entries"),
        ("accounting", "journal", "void", "Void journal entries"),
        ("document", "document", "upload", "Upload documents"),
        ("document", "document", "view", "View documents"),
        ("document", "document", "delete", "Delete documents"),
        ("installation", "order", "view", "View installations"),
        ("installation", "order", "create", "Create installations"),
        (
            "installation",
            "order",
            "schedule",
            "Schedule installations",
        ),
        (
            "installation",
            "order",
            "complete",
            "Complete installations",
        ),
        ("installation", "order", "cancel", "Cancel installations"),
        ("installation", "order", "update", "Update installations"),
        ("monitoring", "alert", "create", "Create alerts"),
        ("lead", "lead", "view", "View leads"),
        ("lead", "lead", "create", "Create leads"),
        ("referral", "referral", "view", "View referrals"),
        ("referral", "referral", "manage", "Manage referrals"),
        ("payment", "link", "create", "Create payment links"),
        ("payment", "manual", "record", "Record manual payments"),
        ("payment", "retry", "retry", "Retry payments"),
        ("payment", "gateway", "view", "View gateways"),
        ("rbac", "role", "manage", "Manage roles"),
        ("rbac", "permission", "manage", "Manage permissions"),
    ];

    let mut permissions_created = 0;
    for (module, resource, action, description) in default_permissions {
        let perm_name = format!("{}.{}.{}", module, resource, action);
        let exists = Permission::find()
            .filter(PermissionColumn::Module.eq(module))
            .filter(PermissionColumn::Resource.eq(resource))
            .filter(PermissionColumn::Action.eq(action))
            .one(&state.db)
            .await?;
        if exists.is_none() {
            let now = chrono::Utc::now();
            let perm = crate::modules::security::domain::entities::permission::ActiveModel {
                name: Set(perm_name),
                module: Set(module.to_string()),
                resource: Set(resource.to_string()),
                action: Set(action.to_string()),
                description: Set(Some(description.to_string())),
                created_at: Set(now),
                ..Default::default()
            };
            if perm.insert(&state.db).await.is_ok() {
                permissions_created += 1;
            }
        }
    }

    Ok((
        StatusCode::CREATED,
        Json(SeedResponse {
            message: "Seed data created successfully".to_string(),
            roles_created,
            permissions_created,
        }),
    ))
}

/// GET /api/v1/admin/dashboard/summary
///
/// Single-query dashboard counters for the admin overview. Returns COUNT(*)
/// aggregates instead of full row lists so the dashboard stays cheap at any
/// data volume. Branch-scoped for non company-wide users, mirroring the
/// per-module list endpoints.
#[derive(Debug, serde::Serialize)]
pub struct DashboardSummaryResponse {
    pub total_customers: i64,
    pub active_subscriptions: i64,
    pub monthly_revenue: f64,
    pub overdue_invoices: i64,
    pub open_tickets: i64,
    pub open_leads: i64,
    pub devices_online: i64,
    pub devices_total: i64,
    pub active_alerts: i64,
}

pub async fn dashboard_summary(
    State(state): State<Arc<AppState>>,
    user: UserContext,
) -> Result<Json<DashboardSummaryResponse>, AppError> {
    require_permission(&user, "customer.account.view").map_err(|e| AppError::Forbidden(e.1))?;

    // Company-wide users see everything; branch-scoped users only see their
    // own branch's rows (same convention as the list endpoints).
    let branch_filter = if user.is_company_wide {
        String::new()
    } else {
        match user.branch_id {
            Some(bid) => format!(" AND t.branch_id = {}", bid),
            None => String::new(),
        }
    };

    let db = &state.db;
    let backend = sea_orm::DatabaseConnection::get_database_backend(db);

    // Each scalar is a separate cheap COUNT query; they are independent so a
    // failure in one non-critical counter (e.g. monitoring) must not blank
    // the whole dashboard. Critical counts use the same tolerant pattern.
    async fn scalar(
        db: &sea_orm::DatabaseConnection,
        backend: DatabaseBackend,
        sql: String,
    ) -> i64 {
        match db
            .query_one(sea_orm::Statement::from_string(backend, sql))
            .await
        {
            Ok(Some(row)) => row.try_get::<i64>("", "c").unwrap_or(0),
            _ => 0,
        }
    }

    let total_customers = scalar(
        db,
        backend,
        format!(
            "SELECT COUNT(*) AS c FROM customer.customers WHERE deleted_at IS NULL{branch}",
            branch = branch_filter
        ),
    )
    .await;

    let active_subscriptions = scalar(
        db,
        backend,
        format!(
            "SELECT COUNT(*) AS c FROM subscription.subscriptions t WHERE t.status = 'active'{branch}",
            branch = branch_filter
        ),
    )
    .await;

    // Monthly revenue: payments received this calendar month.
    let monthly_revenue = match db
        .query_one(sea_orm::Statement::from_string(
            backend,
            format!(
                "SELECT COALESCE(SUM(t.amount), 0) AS c FROM billing.payments t \
                 WHERE t.status = 'completed' \
                 AND t.paid_at >= date_trunc('month', NOW()){branch}",
                branch = branch_filter
            ),
        ))
        .await
    {
        Ok(Some(row)) => row
            .try_get::<sea_orm::prelude::Decimal>("", "c")
            .map(|d| d.try_into().unwrap_or(0.0_f64))
            .unwrap_or(0.0),
        _ => 0.0,
    };

    // Overdue: same definition as BillingService::list_overdue_invoices —
    // unpaid invoices past their due date (pending/sent/overdue).
    let overdue_invoices = scalar(
        db,
        backend,
        format!(
            "SELECT COUNT(*) AS c FROM billing.invoices t WHERE t.status IN ('pending', 'sent', 'overdue') \
             AND t.due_date < CURRENT_DATE{branch}",
            branch = branch_filter
        ),
    )
    .await;

    let open_tickets = scalar(
        db,
        backend,
        format!(
            "SELECT COUNT(*) AS c FROM ticket.tickets t WHERE t.status NOT IN ('closed', 'resolved'){branch}",
            branch = branch_filter
        ),
    )
    .await;

    let open_leads = scalar(
        db,
        backend,
        format!(
            "SELECT COUNT(*) AS c FROM lead.leads t WHERE t.status NOT IN ('converted', 'lost'){branch}",
            branch = branch_filter
        ),
    )
    .await;

    let devices_total = scalar(
        db,
        backend,
        format!(
            "SELECT COUNT(*) AS c FROM device.network_devices t WHERE TRUE{branch}",
            branch = branch_filter
        ),
    )
    .await;

    let devices_online = scalar(
        db,
        backend,
        format!(
            "SELECT COUNT(*) AS c FROM device.network_devices t WHERE t.status = 'online'{branch}",
            branch = branch_filter
        ),
    )
    .await;

    let active_alerts = scalar(
        db,
        backend,
        format!(
            "SELECT COUNT(*) AS c FROM monitoring.monitoring_alerts t WHERE t.status = 'firing'{branch}",
            branch = branch_filter
        ),
    )
    .await;

    Ok(Json(DashboardSummaryResponse {
        total_customers,
        active_subscriptions,
        monthly_revenue,
        overdue_invoices,
        open_tickets,
        open_leads,
        devices_online,
        devices_total,
        active_alerts,
    }))
}
