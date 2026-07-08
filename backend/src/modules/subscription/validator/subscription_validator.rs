pub fn validate_billing_period(months: i32) -> Result<(), String> {
    if !(1..=12).contains(&months) {
        return Err("Billing period must be between 1 and 12 months".into());
    }
    Ok(())
}
