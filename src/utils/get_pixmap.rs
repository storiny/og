use crate::{
    constants::dimensions::{
        IMG_HEIGHT,
        IMG_WIDTH,
    },
    error::AppError,
};
use tiny_skia::Pixmap;

/// Allocates and returns a new [Pixmap] using the default [IMG_WIDTH] and [IMG_HEIGHT] values.
pub fn get_pixmap() -> Result<Pixmap, AppError> {
    Pixmap::new(IMG_WIDTH, IMG_HEIGHT).ok_or(AppError::InternalError(
        "pixmap allocation error".to_string(),
    ))
}
