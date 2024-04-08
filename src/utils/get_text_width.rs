use rusttype::{
    point,
    Font,
    Scale,
};

/// Computes the width (in pixels) of a given text using the provided font size and family.
///
/// See https://stackoverflow.com/a/68154492/22683234
///
/// * `font` - The font instance to use for measuring the width.
/// * `font_size` - The font size value.
/// * `text` - The target text.
pub fn get_text_width(font: &Font, font_size: f32, text: &str) -> f32 {
    font.layout(text, Scale::uniform(font_size), point(0.0, 0.0))
        .last()
        .map(|glyph| glyph.position().x + glyph.unpositioned().h_metrics().advance_width)
        .unwrap_or(0.0)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::constants::fonts::SATOSHI_REGULAR;

    #[test]
    fn can_compute_text_width() {
        let text_width = get_text_width(&SATOSHI_REGULAR, 16.0, "Hello world");
        assert_eq!(format!("{:.1}", text_width), "63.7".to_string());
    }
}
