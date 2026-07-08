use ulid::Ulid;

/// Generate a new ULID (sortable, unique identifier).
pub fn new_ulid() -> String {
    Ulid::new().to_string()
}

/// Generate a friendly ID string (e.g., AX-XXXXXXXXXXXX).
pub fn friendly_id() -> String {
    let ulid = Ulid::new();
    let s = ulid.to_string();
    format!("AX-{}", &s[..12])
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_ulid_unique() {
        let id1 = new_ulid();
        let id2 = new_ulid();
        assert_ne!(id1, id2);
    }

    #[test]
    fn test_friendly_id_format() {
        let id = friendly_id();
        assert!(id.starts_with("AX-"));
        assert_eq!(id.len(), 15);
    }
}
