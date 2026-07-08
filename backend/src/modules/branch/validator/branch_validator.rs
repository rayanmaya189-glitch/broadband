pub fn validate_branch_code(code: &str) -> Result<(), String> {
    if code.len() < 2 || code.len() > 50 {
        return Err("Branch code must be 2-50 characters".into());
    }
    if !code.chars().all(|c| c.is_alphanumeric()) {
        return Err("Branch code must be alphanumeric".into());
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_validate_branch_code() {
        assert!(validate_branch_code("JLG").is_ok());
        assert!(validate_branch_code("A").is_err());
        assert!(validate_branch_code("JL-G").is_err());
    }
}
