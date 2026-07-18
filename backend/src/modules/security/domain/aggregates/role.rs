use crate::modules::security::domain::value_objects::{Permission, RoleId, RoleStatus};

/// Role aggregate root - represents a user role in the RBAC system
#[derive(Debug, Clone)]
pub struct Role {
    pub id: RoleId,
    pub name: String,
    pub slug: String,
    pub description: Option<String>,
    pub parent_role_id: Option<i64>,
    pub is_system: bool,
    pub is_active: bool,
    pub status: RoleStatus,
    pub permissions: Vec<Permission>,
}

/// Domain errors for Role aggregate
#[derive(Debug, Clone, PartialEq)]
pub enum RoleDomainError {
    InvalidSlug,
    RoleNotFound(i64),
    CannotDeleteSystemRole,
    CannotModifySystemRole,
    CircularHierarchy,
    RoleAlreadyHasPermission(String),
    RoleMissingPermission(String),
}

impl std::fmt::Display for RoleDomainError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::InvalidSlug => write!(
                f,
                "Role slug must be lowercase alphanumeric with underscores"
            ),
            Self::RoleNotFound(id) => write!(f, "Role {} not found", id),
            Self::CannotDeleteSystemRole => write!(f, "Cannot delete a system role"),
            Self::CannotModifySystemRole => write!(f, "Cannot modify a system role"),
            Self::CircularHierarchy => write!(f, "Circular role hierarchy detected"),
            Self::RoleAlreadyHasPermission(ref p) => {
                write!(f, "Role already has permission '{}'", p)
            }
            Self::RoleMissingPermission(ref p) => {
                write!(f, "Role does not have permission '{}'", p)
            }
        }
    }
}

impl std::error::Error for RoleDomainError {}

impl Role {
    pub fn new(
        name: String,
        slug: String,
        description: Option<String>,
    ) -> Result<Self, RoleDomainError> {
        if !Self::is_valid_slug(&slug) {
            return Err(RoleDomainError::InvalidSlug);
        }
        Ok(Self {
            id: RoleId::new(0),
            name,
            slug,
            description,
            parent_role_id: None,
            is_system: false,
            is_active: true,
            status: RoleStatus::Active,
            permissions: Vec::new(),
        })
    }

    fn is_valid_slug(slug: &str) -> bool {
        !slug.is_empty()
            && slug.len() <= 100
            && slug
                .chars()
                .all(|c| c.is_ascii_lowercase() || c.is_ascii_digit() || c == '_')
    }

    pub fn add_permission(&mut self, permission: Permission) -> Result<(), RoleDomainError> {
        if self.permissions.iter().any(|p| p.name == permission.name) {
            return Err(RoleDomainError::RoleAlreadyHasPermission(permission.name));
        }
        self.permissions.push(permission);
        Ok(())
    }

    pub fn remove_permission(&mut self, permission_name: &str) -> Result<(), RoleDomainError> {
        let before = self.permissions.len();
        self.permissions.retain(|p| p.name != permission_name);
        if self.permissions.len() == before {
            return Err(RoleDomainError::RoleMissingPermission(
                permission_name.to_string(),
            ));
        }
        Ok(())
    }

    pub fn has_permission(&self, permission_name: &str) -> bool {
        self.permissions
            .iter()
            .any(|p| p.name == permission_name || p.matches_wildcard(permission_name))
    }

    pub fn can_be_deleted(&self) -> Result<(), RoleDomainError> {
        if self.is_system {
            return Err(RoleDomainError::CannotDeleteSystemRole);
        }
        Ok(())
    }

    pub fn resolve_all_permissions(&self, all_roles: &[Role]) -> Vec<String> {
        let mut perms: Vec<String> = self.permissions.iter().map(|p| p.name.clone()).collect();
        if let Some(parent_id) = self.parent_role_id {
            if let Some(parent) = all_roles.iter().find(|r| r.id.value() == parent_id) {
                perms.extend(parent.resolve_all_permissions(all_roles));
            }
        }
        perms.sort();
        perms.dedup();
        perms
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new_role() {
        let role = Role::new(
            "Network Admin".to_string(),
            "network_admin".to_string(),
            Some("Network infrastructure management".to_string()),
        );
        assert!(role.is_ok());
        let role = role.unwrap();
        assert_eq!(role.status, RoleStatus::Active);
        assert!(role.permissions.is_empty());
    }

    #[test]
    fn test_add_permission() {
        let mut role = Role::new("Admin".to_string(), "admin".to_string(), None).unwrap();
        let perm = Permission::new(
            "device.view".to_string(),
            "device".to_string(),
            "view".to_string(),
        );
        assert!(role.add_permission(perm).is_ok());
        assert!(role.has_permission("device.view"));
        // Duplicate should fail
        let perm2 = Permission::new(
            "device.view".to_string(),
            "device".to_string(),
            "view".to_string(),
        );
        assert_eq!(
            role.add_permission(perm2),
            Err(RoleDomainError::RoleAlreadyHasPermission(
                "device.view".to_string()
            ))
        );
    }

    #[test]
    fn test_cannot_delete_system_role() {
        let mut role =
            Role::new("Super Admin".to_string(), "super_admin".to_string(), None).unwrap();
        role.is_system = true;
        assert_eq!(
            role.can_be_deleted(),
            Err(RoleDomainError::CannotDeleteSystemRole)
        );
    }
}
