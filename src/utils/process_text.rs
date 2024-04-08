use unidecode::unidecode;

/// Normalizes all non-ASCII characters in the text. This serves as a temporary solution to the
/// rendering complications for complex characters and emojis.
///
/// * `text` - The text to normalize.
pub fn process_text(text: &str) -> String {
    unidecode(text)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn can_normalize_a_text() {
        let result = process_text("Æneid");
        assert_eq!(result, "AEneid".to_string());
    }
}
