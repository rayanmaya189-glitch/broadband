pub fn validate_plan_code(code: &str) -> Result<(), String> {
    if code.len() < 2 || code.len() > 50 {
        return Err("Plan code must be 2-50 characters".into());
    }
    if !code.chars().all(|c| c.is_alphanumeric() || c == '-') {
        return Err("Plan code must be alphanumeric with hyphens".into());
    }
    Ok(())
}

pub fn validate_speed(down: i32, up: i32) -> Result<(), String> {
    if down < 1 || up < 1 { return Err("Speed must be at least 1 Mbps".into()); }
    if down > 10000 || up > 10000 { return Err("Speed cannot exceed 10000 Mbps".into()); }
    Ok(())
}
