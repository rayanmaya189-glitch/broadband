/// Generate a new UUID v7 (time-ordered, sortable) using the current system time.
///
/// UUID v7 encodes a Unix timestamp in milliseconds as the most significant bits,
/// making it ideal for database primary keys (B-tree friendly, no hotspot).
pub fn new_v7() -> uuid::Uuid {
    uuid::Uuid::now_v7()
}

/// Generate a new UUID v7 as a lowercase hyphenated string.
pub fn new_v7_string() -> String {
    new_v7().to_string()
}

/// Generate a new UUID v7 as a compact string (no hyphens).
pub fn new_v7_compact() -> String {
    new_v7().simple().to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new_v7_is_time_ordered() {
        let a = new_v7();
        let b = new_v7();
        assert!(b.as_u128() >= a.as_u128());
    }

    #[test]
    fn test_new_v7_string_format() {
        let id = new_v7_string();
        assert_eq!(id.len(), 36);
        assert!(id.contains('-'));
    }

    #[test]
    fn test_new_v7_compact_format() {
        let id = new_v7_compact();
        assert_eq!(id.len(), 32);
        assert!(!id.contains('-'));
    }
}
