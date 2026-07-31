use chrono::Utc;

use crate::shared::utils::uuid_v7;

/// Maximum length of a short business number column (VARCHAR(20)).
const MAX_BUSINESS_NUMBER_LEN: usize = 20;

/// Generate a compact, unique business document number (invoice, payment, refund,
/// credit note) that safely fits within a `VARCHAR(20)` column.
///
/// Format: `{PREFIX}-{YYMMDD}-{RANDOM}` e.g. `INV-260731-1a2b3c4d5`.
/// The random suffix is derived from a UUID v7, giving ~36 bits of entropy per
/// day window — collision odds are negligible for a billing system. Prefixes
/// should be 1-3 characters.
pub fn new_business_number(prefix: &str) -> String {
    let uuid = uuid_v7::new_v7_compact();
    let rand = &uuid[uuid.len() - 9..];
    let date = Utc::now().format("%y%m%d");
    let mut number = format!("{}-{}-{}", prefix, date, rand);
    debug_assert!(
        number.len() <= MAX_BUSINESS_NUMBER_LEN,
        "business number exceeds VARCHAR(20) width: {}",
        number
    );
    if number.len() > MAX_BUSINESS_NUMBER_LEN {
        number.truncate(MAX_BUSINESS_NUMBER_LEN);
    }
    number
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new_business_number_format_and_width() {
        let n = new_business_number("INV");
        assert_eq!(n.len(), 20);
        assert!(n.starts_with("INV-"));
        assert_eq!(n.chars().nth(4).unwrap().is_ascii_digit(), true);
    }

    #[test]
    fn test_new_business_number_unique() {
        let a = new_business_number("PAY");
        let b = new_business_number("PAY");
        assert_ne!(a, b);
    }

    #[test]
    fn test_short_prefix_fits() {
        let n = new_business_number("CN");
        assert!(n.len() <= 20);
    }
}
