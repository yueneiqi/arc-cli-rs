pub fn sanitize_url(url: &str) -> String {
    // if url does not start with http or https, prepend https
    if !url.starts_with("http://") && !url.starts_with("https://") {
        format!("https://{}", url)
    } else {
        url.to_string()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sanitize_url_with_https() {
        assert_eq!(sanitize_url("https://example.com"), "https://example.com");
    }

    #[test]
    fn test_sanitize_url_with_http() {
        assert_eq!(sanitize_url("http://example.com"), "http://example.com");
    }

    #[test]
    fn test_sanitize_url_without_protocol() {
        assert_eq!(sanitize_url("example.com"), "https://example.com");
    }
}
