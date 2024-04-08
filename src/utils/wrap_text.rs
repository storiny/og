use textwrap::wrap;
use unicode_segmentation::UnicodeSegmentation;

/// Wraps the text into multiple lines (upto a maximum of `max_lines` if provided) to fit inside the
/// provided `max_width` value.
///
/// * `text` - The text to wrap.
/// * `max_width` - The maximum width of the text container in pixels.
/// * `max_lines` - The maximum lines to wrap.
/// * `compute_text_width` - The closure to compute width of the text passed as an argument in
///   pixels.
pub fn wrap_text<F>(
    text: &str,
    max_width: u32,
    max_lines: Option<u32>,
    compute_text_width: F,
) -> Vec<String>
where
    F: Fn(&str) -> usize,
{
    let text_width = compute_text_width(text);
    let text_chars = text.graphemes(true).count();
    let char_width = text_width / text_chars.max(1); // Average width of each character in pixels.
    let columns = max_width / char_width as u32;
    let lines = wrap(text, columns as usize);

    lines
        .iter()
        .enumerate()
        .filter_map(|(index, line)| {
            if let Some(max_lines) = max_lines {
                let index = index as u32;
                let max_index = max_lines - 1;

                if index > max_index {
                    return None;
                }

                let is_last_line = index == max_index;
                let has_next_line = lines.get((index + 1) as usize).is_some();

                // Truncate the last line if next line is present.
                if is_last_line && has_next_line {
                    let mut chars = line.chars();
                    chars.next_back(); // Remove the last character for `…`
                    return Some(format!("{}…", chars.as_str()));
                }
            };

            Some(line.to_string())
        })
        .collect::<Vec<_>>()
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{
        constants::fonts::SATOSHI_REGULAR,
        utils::get_text_width,
    };

    #[test]
    fn can_wrap_a_text() {
        let result = wrap_text(
            "The quick brown fox jumps over the lazy dog",
            108,
            None,
            |text| get_text_width(&SATOSHI_REGULAR, 16.0, text) as usize,
        );

        assert_eq!(
            result,
            vec![
                "The quick brown fox".to_string(),
                "jumps over the lazy".to_string(),
                "dog".to_string()
            ]
        );
    }

    #[test]
    fn can_limit_the_number_of_lines() {
        let result = wrap_text(
            "The quick brown fox jumps over the lazy dog",
            108,
            Some(2),
            |text| get_text_width(&SATOSHI_REGULAR, 16.0, text) as usize,
        );

        assert_eq!(
            result,
            vec![
                "The quick brown fox".to_string(),
                "jumps over the laz…".to_string(),
            ]
        );
    }
}
