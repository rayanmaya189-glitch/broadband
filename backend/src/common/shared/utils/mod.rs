//! Shared utility functions for the AeroXe Broadband platform.

use chrono::{DateTime, Utc};

/// Returns the current UTC timestamp.
pub fn now_utc() -> DateTime<Utc> {
    Utc::now()
}

/// Generate a unique ticket number with prefix.
pub fn generate_ticket_number(prefix: &str) -> String {
    let timestamp = Utc::now().format("%Y%m%d%H%M%S");
    let random: u16 = rand::random();
    format!("{prefix}-{}-{:04X}", timestamp, random)
}

/// Generate a unique invoice number.
pub fn generate_invoice_number(branch_code: &str) -> String {
    let timestamp = Utc::now().format("%Y%m%d");
    let random: u16 = rand::random();
    format!("INV-{}-{}-{:04X}", branch_code, timestamp, random)
}

/// Generate a unique customer code.
pub fn generate_customer_code(branch_code: &str) -> String {
    let timestamp = Utc::now().format("%Y%m%d");
    let random: u16 = rand::random();
    format!("CUS-{}-{}-{:04X}", branch_code, timestamp, random)
}

/// Truncate a string to a maximum length, appending "..." if truncated.
pub fn truncate_string(s: &str, max_len: usize) -> String {
    if s.len() <= max_len {
        s.to_string()
    } else {
        format!("{}...", &s[..max_len - 3])
    }
}

/// Mask a phone number for privacy (show only last 4 digits).
pub fn mask_phone(phone: &str) -> String {
    if phone.len() <= 4 {
        phone.to_string()
    } else {
        format!("{}****{}", &phone[..phone.len() - 4], &phone[phone.len() - 4..])
    }
}

/// Mask an email for privacy (show first char and domain).
pub fn mask_email(email: &str) -> String {
    if let Some((local, domain)) = email.split_once('@') {
        if local.len() <= 1 {
            format!("*@{domain}")
        } else {
            format!("{}***@{domain}", &local[..1])
        }
    } else {
        "***".to_string()
    }
}
