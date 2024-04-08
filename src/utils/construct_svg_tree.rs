use crate::error::AppError;
use lazy_static::lazy_static;
use reqwest::header::ACCEPT;
use std::{
    ops::Deref,
    sync::Arc,
};
use usvg::{
    fontdb,
    ImageHrefResolver,
    ImageKind,
    Options,
    Tree,
};

lazy_static! {
    static ref REQUEST_CLIENT : reqwest::blocking::Client = reqwest::blocking::Client::new();
    //
    static ref OPTIONS: Options = Options {
        image_href_resolver: ImageHrefResolver {
            resolve_string: Box::new(move |path: &str, _, _| {
                let res = REQUEST_CLIENT.get(path).header(ACCEPT, "image/png, image/jpeg").send().ok()?;
                let content_type = res
                    .headers()
                    .get("content-type")
                    .and_then(|value| value.to_str().ok())?
                    .to_owned();
                let buffer = res.bytes().ok()?.into_iter().collect::<Vec<u8>>();

                match content_type.as_str() {
                    "image/png" => Some(ImageKind::PNG(Arc::new(buffer))),
                    "image/jpeg" => Some(ImageKind::JPEG(Arc::new(buffer))),
                    _ => None,
                }
            }),
            ..Default::default()
        },
        ..Default::default()
    };
    //
    static ref FONT_DB: fontdb::Database = {
        let mut font_db = fontdb::Database::new();
        font_db.load_fonts_dir("fonts");
        font_db
    };
}

/// Constructs a SVG tree using the provided SVG string.
///
/// # Image fetching behaviour
///
/// Only PNG and JPEG images are resolved. Other image formats are ignored.
///
/// * `svg_string` - The SVG string.
#[tracing::instrument(err)]
pub async fn construct_svg_tree(svg_string: &str) -> Result<Tree, AppError> {
    let svg_string = svg_string.to_owned();

    tokio::task::spawn_blocking(move || {
        Tree::from_str(&svg_string, OPTIONS.deref(), FONT_DB.deref())
            .map_err(|err| AppError::InternalError(format!("unable to parse the svg: {err:?}")))
    })
    .await
    .map_err(|err| AppError::InternalError(format!("unable to complete the task: {err:?}")))?
}
