use crate::modules::security::domain::entities::{
    Permission, Role, RoleColumn, RolePermission, RolePermissionColumn, UserRole,
    UserRoleActiveModel, UserRoleColumn,
};
use crate::shared::errors::AppError;
use sea_orm::{ActiveModelTrait, ColumnTrait, DatabaseConnection, EntityTrait, QueryFilter, Set};

pub struct SecurityService;

impl SecurityService {
    pub async fn resolve_permissions(
        db: &DatabaseConnection,
        user_id: i64,
    ) -> Result<Vec<String>, AppError> {
        let user_roles = UserRole::find()
            .filter(UserRoleColumn::UserId.eq(user_id))
            .filter(UserRoleColumn::IsActive.eq(true))
            .all(db)
            .await?;

        let mut permission_names = Vec::new();
        for ur in user_roles {
            let role_perms = RolePermission::find()
                .filter(RolePermissionColumn::RoleId.eq(ur.role_id))
                .all(db)
                .await?;
            for rp in role_perms {
                if let Some(perm) = Permission::find_by_id(rp.permission_id).one(db).await? {
                    permission_names.push(perm.name);
                }
            }
            if let Some(role) = Role::find_by_id(ur.role_id).one(db).await? {
                if let Some(parent_id) = role.parent_role_id {
                    let parent_perms = RolePermission::find()
                        .filter(RolePermissionColumn::RoleId.eq(parent_id))
                        .all(db)
                        .await?;
                    for rp in parent_perms {
                        if let Some(perm) = Permission::find_by_id(rp.permission_id).one(db).await?
                        {
                            if !permission_names.contains(&perm.name) {
                                permission_names.push(perm.name);
                            }
                        }
                    }
                }
            }
        }
        Ok(permission_names)
    }

    pub async fn has_permission(
        db: &DatabaseConnection,
        user_id: i64,
        permission: &str,
    ) -> Result<bool, AppError> {
        let permissions = Self::resolve_permissions(db, user_id).await?;
        Ok(permissions.contains(&permission.to_string()))
    }

    pub async fn list_roles(
        db: &DatabaseConnection,
    ) -> Result<Vec<crate::modules::security::domain::entities::role::Model>, AppError> {
        let roles = Role::find()
            .filter(RoleColumn::IsActive.eq(true))
            .all(db)
            .await?;
        Ok(roles)
    }

    pub async fn list_permissions(
        db: &DatabaseConnection,
    ) -> Result<Vec<crate::modules::security::domain::entities::permission::Model>, AppError> {
        let perms = Permission::find().all(db).await?;
        Ok(perms)
    }

    pub async fn assign_role(
        db: &DatabaseConnection,
        redis: &mut redis::aio::ConnectionManager,
        user_id: i64,
        role_id: i64,
        assigned_by: Option<i64>,
    ) -> Result<(), AppError> {
        let existing = UserRole::find()
            .filter(UserRoleColumn::UserId.eq(user_id))
            .filter(UserRoleColumn::RoleId.eq(role_id))
            .one(db)
            .await?;
        if let Some(ur) = existing {
            if ur.is_active {
                return Ok(());
            }
            let mut active: UserRoleActiveModel = ur.into();
            active.is_active = Set(true);
            active.assigned_by = Set(assigned_by);
            active.update(db).await?;
        } else {
            let new_ur = UserRoleActiveModel {
                user_id: Set(user_id),
                role_id: Set(role_id),
                assigned_by: Set(assigned_by),
                is_active: Set(true),
                created_at: Set(chrono::Utc::now()),
                ..Default::default()
            };
            new_ur.insert(db).await?;
        }
        // Invalidate cached permissions in Redis so the user gets fresh permissions on next request
        crate::modules::identity::application::services::IdentityService::invalidate_permissions(
            redis, user_id,
        )
        .await?;
        Ok(())
    }

    pub async fn revoke_role(
        db: &DatabaseConnection,
        redis: &mut redis::aio::ConnectionManager,
        user_id: i64,
        role_id: i64,
    ) -> Result<(), AppError> {
        if let Some(ur) = UserRole::find()
            .filter(UserRoleColumn::UserId.eq(user_id))
            .filter(UserRoleColumn::RoleId.eq(role_id))
            .one(db)
            .await?
        {
            let mut active: UserRoleActiveModel = ur.into();
            active.is_active = Set(false);
            active.update(db).await?;
        }
        // Invalidate cached permissions in Redis so the user gets fresh permissions on next request
        crate::modules::identity::application::services::IdentityService::invalidate_permissions(
            redis, user_id,
        )
        .await?;
        Ok(())
    }
}

