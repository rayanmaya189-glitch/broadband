pub fn validate_role_name(name: &str) -> Result<(), String> {
    if name.len() < 2 || name.len() > 50 {
        return Err("Role name must be 2-50 characters".into());
    }
    if !name.chars().all(|c| c.is_alphanumeric() || c == '_') {
        return Err("Role name must be alphanumeric with underscores".into());
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_validate_role_name() {
        assert!(validate_role_name("admin").is_ok());
        assert!(validate_role_name("a").is_err());
        assert!(validate_role_name("super admin!").is_err());
    }
}
