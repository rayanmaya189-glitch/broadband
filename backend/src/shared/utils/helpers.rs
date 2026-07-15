use chrono::Utc;
use sha2::{Digest, Sha256};

/// Hash Aadhaar number with personal salt
pub fn hash_aadhaar(aadhaar: &str) -> String {
    let salt = format!("aeroxe:{}", aadhaar);
    let mut hasher = Sha256::new();
    hasher.update(salt.as_bytes());
    hex::encode(hasher.finalize())
}

/// Hash PAN number with personal salt
pub fn hash_pan(pan: &str) -> String {
    let salt = format!("aeroxe:{}", pan);
    let mut hasher = Sha256::new();
    hasher.update(salt.as_bytes());
    hex::encode(hasher.finalize())
}

/// Mask phone number: +919876543210 → +91******3210
pub fn mask_phone(phone: &str) -> String {
    if phone.len() > 6 {
        let prefix_len = phone.len().min(4);
        let suffix_len = 4.min(phone.len() - prefix_len);
        let masked_len = phone.len() - prefix_len - suffix_len;
        format!(
            "{}{}{}",
            &phone[..prefix_len],
            "*".repeat(masked_len),
            &phone[phone.len() - suffix_len..]
        )
    } else {
        phone.to_string()
    }
}

/// Mask email: rahul@example.com → r****l@example.com
pub fn mask_email(email: &str) -> String {
    let parts: Vec<&str> = email.split('@').collect();
    if parts.len() == 2 && parts[0].len() > 2 {
        format!(
            "{}****{}@{}",
            &parts[0][..1],
            &parts[0][parts[0].len() - 1..],
            parts[1]
        )
    } else {
        email.to_string()
    }
}

/// Generate customer code: AX-{BRANCH_CODE}-{YYYYMM}-{SEQUENCE}
pub fn generate_customer_code(branch_code: &str, sequence: i64) -> String {
    let month = Utc::now().format("%Y%m").to_string();
    format!("AX-{}-{}-{:04}", branch_code, month, sequence)
}

/// Generate invoice number: INV-{YYYY}-{MM}-{SEQUENCE}
pub fn generate_invoice_number(sequence: i64) -> String {
    let now = Utc::now();
    format!(
        "INV-{}-{:02}-{:04}",
        now.format("%Y"),
        now.format("%m"),
        sequence
    )
}

/// Generate ticket number: TKT-{YYYY}-{MM}-{SEQUENCE}
pub fn generate_ticket_number(sequence: i64) -> String {
    let now = Utc::now();
    format!(
        "TKT-{}-{:02}-{:04}",
        now.format("%Y"),
        now.format("%m"),
        sequence
    )
}

/// Generate payment number: PAY-{YYYY}-{MM}-{SEQUENCE}
pub fn generate_payment_number(sequence: i64) -> String {
    let now = Utc::now();
    format!(
        "PAY-{}-{:02}-{:04}",
        now.format("%Y"),
        now.format("%m"),
        sequence
    )
}

/// Generate refund number: RFD-{YYYY}-{MM}-{SEQUENCE}
pub fn generate_refund_number(sequence: i64) -> String {
    let now = Utc::now();
    format!(
        "RFD-{}-{:02}-{:04}",
        now.format("%Y"),
        now.format("%m"),
        sequence
    )
}

/// Generate journal entry number: JE-{YYYY}-{MM}-{SEQUENCE}
pub fn generate_journal_entry_number(sequence: i64) -> String {
    let now = Utc::now();
    format!(
        "JE-{}-{:02}-{:06}",
        now.format("%Y"),
        now.format("%m"),
        sequence
    )
}

/// Generate a referral code from customer name + random suffix
pub fn generate_referral_code(customer_name: &str) -> String {
    use rand::Rng;
    let prefix: String = customer_name
        .chars()
        .filter(|c| c.is_alphabetic())
        .take(4)
        .collect::<String>()
        .to_uppercase();
    let suffix: u32 = rand::thread_rng().gen_range(1000..9999);
    format!("{}{}", prefix, suffix)
}

/// Calculate retry delay with exponential backoff
pub fn calculate_retry_delay(retry_count: u32) -> std::time::Duration {
    let base_delay = 10;
    let multiplier = 3u64.pow(retry_count);
    std::time::Duration::from_secs(base_delay * multiplier)
}
