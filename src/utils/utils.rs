/// generate short id using nanoid
pub fn generate_short_id() -> String {
    nanoid::nanoid!(8)
}

/// validate url format
pub fn is_valid_url(url: &str) -> bool {
    url::Url::parse(url).is_ok()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_generate_short_id() {
        let id1 = generate_short_id();
        let id2 = generate_short_id();
        assert_eq!(id1.len(), 8);
        assert_eq!(id2.len(), 8);
        assert_ne!(id1, id2);
    }

    #[test]
    fn test_is_valid_url() {
        assert!(is_valid_url("https://example.com"));
        assert!(is_valid_url("http://example.com"));
        assert!(is_valid_url("https://example.com/path?query=value"));
        assert!(!is_valid_url("not-a-url"));
        assert!(!is_valid_url(""));
    }
}
