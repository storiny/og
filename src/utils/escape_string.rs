use sailfish::runtime::escape::escape_to_string;

/// Replaces the characters `&"'<>` in the provided text with the equivalent HTML.
///
/// * `text` - The text to escape.
pub fn escape_string(text: &str) -> String {
    let mut buffer = String::new();
    escape_to_string(text, &mut buffer);
    buffer
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn can_escape_a_string() {
        let result = escape_string("<h1>Hello, world!</ h1>");
        assert_eq!(result, "&lt;h1&gt;Hello, world!&lt;/ h1&gt;");
    }
}
