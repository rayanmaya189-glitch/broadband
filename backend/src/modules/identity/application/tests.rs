#[cfg(test)]
mod tests {
    use crate::shared::middleware::auth::{has_permission, require_permission, UserContext};

    // ── Permission Tests ──

    fn make_user(permissions: Vec<&str>, is_company_wide: bool) -> UserContext {
        UserContext {
            user_id: 1,
            email: "test@aeroxe.com".to_string(),
            role: "test_role".to_string(),
            branch_id: Some(1),
            is_company_wide,
            permissions: permissions.into_iter().map(String::from).collect(),
        }
    }

    #[test]
    fn test_company_wide_user_has_all_permissions() {
        let user = make_user(vec![], true);
        assert!(has_permission(&user, "customer.account.view"));
        assert!(has_permission(&user, "billing.invoice.create"));
        assert!(has_permission(&user, "device.router.restart"));
    }

    #[test]
    fn test_exact_permission_match() {
        let user = make_user(vec!["customer.account.view", "billing.invoice.create"], false);
        assert!(has_permission(&user, "customer.account.view"));
        assert!(has_permission(&user, "billing.invoice.create"));
        assert!(!has_permission(&user, "device.router.restart"));
    }

    #[test]
    fn test_wildcard_permission_match() {
        let user = make_user(vec!["customer.account.*", "billing.invoice.view"], false);
        assert!(has_permission(&user, "customer.account.view"));
        assert!(has_permission(&user, "customer.account.create"));
        assert!(has_permission(&user, "customer.account.update"));
        assert!(!has_permission(&user, "billing.invoice.create"));
    }

    #[test]
    fn test_require_permission_success() {
        let user = make_user(vec!["customer.account.view"], false);
        assert!(require_permission(&user, "customer.account.view").is_ok());
    }

    #[test]
    fn test_require_permission_failure() {
        let user = make_user(vec!["customer.account.view"], false);
        let result = require_permission(&user, "device.router.restart");
        assert!(result.is_err());
        assert!(result.unwrap_err().1.contains("device.router.restart"));
    }

    #[test]
    fn test_no_permissions_denies_access() {
        let user = make_user(vec![], false);
        assert!(!has_permission(&user, "customer.account.view"));
    }

    #[test]
    fn test_partial_wildcard_does_not_match_deeper() {
        let user = make_user(vec!["device.*"], false);
        // "device.*" matches exactly 2-part permissions under device
        assert!(has_permission(&user, "device.router"));
        assert!(has_permission(&user, "device.view"));
        // But NOT 3-part or deeper permissions
        assert!(!has_permission(&user, "device.router.view"));
        assert!(!has_permission(&user, "device.router.view.extra"));
    }
}
