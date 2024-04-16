use unicode_segmentation::UnicodeSegmentation;

/// Truncates the given text to fit inside the provided `max_width` value. The `compute_text_width`
/// should return the width of the text in pixels.
///
/// * `text` - The text to truncate.
/// * `max_width` - The maximum width in pixels.
/// * `compute_text_width` - The closure to compute width of the text passed as an argument in
///   pixels.
pub fn truncate_text<F>(text: &str, max_width: u32, compute_text_width: F) -> String
where
    F: Fn(&str) -> usize,
{
    let mut text = text.to_owned();
    let text_width = compute_text_width(&text);
    let text_chars = text.graphemes(true).count().max(1); // Total number of characters.
    let char_width = (text_width / text_chars).max(1); // Average width of each character in pixels.
    // Maximum number of characters that can fit inside the given `max_width` value.
    let mut fitting_chars = ((max_width / char_width as u32) as f32).floor() as usize;

    // Remove the remaining characters.
    if text_chars > fitting_chars {
        // For `…` at the end
        fitting_chars = fitting_chars.saturating_sub(1);
        text.truncate(fitting_chars);
        text = format!("{text}…");
    }

    text
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{
        constants::fonts::SATOSHI_REGULAR,
        utils::get_text_width,
    };

    #[test]
    fn can_truncate_a_text() {
        let result = truncate_text(
            "a sufficiently long string to perform truncation",
            128,
            |text| get_text_width(&SATOSHI_REGULAR, 16.0, text) as usize,
        );

        assert_eq!(result, "a sufficiently long stri…".to_string());
    }
}
