pub fn validate_permission_name(name: &str) -> Result<(), String> {
    if name.len() < 3 || name.len() > 100 {
        return Err("Permission name must be 3-100 characters".into());
    }
    if !name.contains('.') {
        return Err("Permission name must follow module.resource.action format".into());
    }
    Ok(())
}
