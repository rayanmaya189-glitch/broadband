use crate::common::utils::validators::validate_phone;

pub fn validate_customer_phone(phone: &str) -> Result<(), String> {
    if !validate_phone(phone) {
        return Err("Invalid phone number".into());
    }
    Ok(())
}

pub fn validate_status_transition(from: &str, to: &str) -> Result<(), String> {
    let valid = matches!(
        (from, to),
        ("lead", "prospect") | ("prospect", "active") | ("active", "suspended") |
        ("suspended", "active") | ("active", "deactivated") | ("suspended", "deactivated") |
        ("lead", "blacklist") | ("prospect", "blacklist") | ("active", "blacklist")
    );
    if !valid {
        return Err(format!("Invalid status transition from {from} to {to}"));
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_status_transitions() {
        assert!(validate_status_transition("lead", "prospect").is_ok());
        assert!(validate_status_transition("active", "suspended").is_ok());
        assert!(validate_status_transition("lead", "active").is_err());
    }
}
