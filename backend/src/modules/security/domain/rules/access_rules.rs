/// Security access control rules and invariants
pub struct AccessRules;

impl AccessRules {
    /// Company-wide roles that bypass branch filtering
    pub const COMPANY_WIDE_ROLES: &[&str] = &["super_admin", "isp_owner", "finance_manager"];

    /// Role hierarchy for permission inheritance
    pub const ROLE_HIERARCHY: &[(&str, Option<&str>)] = &[
        ("super_admin", None),
        ("isp_owner", Some("super_admin")),
        ("network_admin", Some("isp_owner")),
        ("noc_engineer", Some("network_admin")),
        ("finance_manager", Some("isp_owner")),
        ("billing_operator", Some("finance_manager")),
        ("field_technician", None),
        ("customer_support", None),
        ("sales_agent", None),
        ("customer", None),
    ];

    /// Check if a role is company-wide
    pub fn is_company_wide(role: &str) -> bool {
        Self::COMPANY_WIDE_ROLES.contains(&role)
    }

    /// Check if role A has authority over role B
    pub fn has_authority_over(authority_role: &str, target_role: &str) -> bool {
        if Self::is_company_wide(authority_role) {
            return true;
        }
        // Walk the hierarchy
        let mut current = Some(authority_role);
        while let Some(role) = current {
            if role == target_role {
                return true;
            }
            current = Self::ROLE_HIERARCHY
                .iter()
                .find(|(r, _)| *r == role)
                .and_then(|(_, parent)| *parent);
        }
        false
    }

    /// Check if a user can manage another user's role
    pub fn can_assign_role(assigned_by_role: &str, target_role: &str) -> bool {
        match assigned_by_role {
            "super_admin" => true,
            "isp_owner" => !matches!(target_role, "super_admin"),
            _ => false,
        }
    }

    /// Maximum permissions per role
    pub const MAX_PERMISSIONS_PER_ROLE: usize = 500;

    /// Maximum roles per user
    pub const MAX_ROLES_PER_USER: usize = 10;
}
