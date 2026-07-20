use axum::extract::State;
use axum::http::StatusCode;
use axum::Json;
use sea_orm::{ActiveModelTrait, ColumnTrait, EntityTrait, QueryFilter, Set};
use std::sync::Arc;

use crate::shared::app_state::AppState;
use crate::shared::errors::AppError;
use crate::shared::middleware::auth::UserContext;

#[derive(Debug, serde::Serialize)]
pub struct SeedResponse {
    pub message: String,
    pub roles_created: usize,
    pub permissions_created: usize,
}

/// POST /api/v1/admin/seed
pub async fn seed_data(
    State(state): State<Arc<AppState>>,
    _user: UserContext,
) -> Result<(StatusCode, Json<SeedResponse>), AppError> {
    use crate::modules::security::domain::entities::{Permission, PermissionColumn, Role, RoleColumn};

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
        ("subscription", "subscription", "manage", "Manage subscriptions"),
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
        ("bandwidth", "profile", "create", "Create bandwidth profiles"),
        ("bandwidth", "profile", "update", "Update bandwidth profiles"),
        ("bandwidth", "profile", "delete", "Delete bandwidth profiles"),
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
        ("installation", "order", "schedule", "Schedule installations"),
        ("installation", "order", "complete", "Complete installations"),
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
