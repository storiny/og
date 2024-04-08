use crate::{
    constants::fonts::CABINET_GROTESK_EXTRABOLD,
    error::AppError,
    grpc::defs::open_graph_def::v1::{
        GetTagOpenGraphDataRequest,
        GetTagOpenGraphDataResponse,
    },
    utils::{
        construct_svg_tree,
        get_pixmap,
        get_text_width,
        process_text,
        truncate_text,
    },
    AppState,
    TagTemplate,
};
use abbrev_num::abbrev_num;
use actix_http::StatusCode;
use actix_web::{
    get,
    http::header::{
        CacheControl,
        CacheDirective,
    },
    web,
    HttpResponse,
};
use resvg::render;
use sailfish::TemplateOnce;
use serde::Deserialize;
use tiny_skia::Transform;
use tonic::Code;

#[derive(Deserialize)]
struct Fragments {
    id: String,
}

#[get("/tags/{id}")]
#[tracing::instrument(
    name = "GET /tags/{id}",
    skip_all,
    fields(
        id = %path.id
    ),
    err
)]
async fn get(
    path: web::Path<Fragments>,
    state: web::Data<AppState>,
) -> Result<HttpResponse, AppError> {
    let id = path
        .id
        .parse::<i64>()
        .map_err(|_| AppError::from("Invalid tag ID"))?;

    let response = {
        let grpc_client = &state.grpc_client;
        let mut client = grpc_client.lock().await;

        client
            .get_tag_open_graph_data(tonic::Request::new(GetTagOpenGraphDataRequest {
                id: id.to_string(),
            }))
            .await
            .map_err(|status| {
                if matches!(status.code(), Code::NotFound) {
                    AppError::ClientError(StatusCode::NOT_FOUND, "Tag not found".to_string())
                } else {
                    AppError::TonicError(status)
                }
            })?
            .into_inner()
    };

    let bytes = generate_tag_open_graph_image(response).await?;
    let mut builder = HttpResponse::Ok();

    builder.insert_header(CacheControl(vec![
        CacheDirective::Public,
        CacheDirective::MaxAge(86400_u32), // 24 hours
    ]));

    Ok(builder.content_type(mime::IMAGE_PNG).body(bytes))
}

/// Renders the open graph image for a tag.
///
/// * `response` - The tag open graph response.
async fn generate_tag_open_graph_image(
    response: GetTagOpenGraphDataResponse,
) -> Result<Vec<u8>, AppError> {
    let mut pixmap = get_pixmap()?;

    let story_count = abbrev_num(response.story_count as isize, None).unwrap_or_default();
    let follower_count = abbrev_num(response.follower_count as isize, None).unwrap_or_default();
    let name = truncate_text(&process_text(&response.name), 900, |text| {
        get_text_width(&CABINET_GROTESK_EXTRABOLD, 112.0, &format!("#{text}")) as usize
    });

    let svg = TagTemplate {
        name,
        story_count,
        follower_count,
    }
    .render_once()?;

    let tree = construct_svg_tree(&svg).await?;

    render(&tree, Transform::default(), &mut pixmap.as_mut());

    let bytes = pixmap.encode_png()?;

    Ok(bytes)
}

pub fn init_routes(cfg: &mut web::ServiceConfig) {
    cfg.service(get);
}

#[cfg(test)]
mod tests {
    use super::*;
    use image::EncodableLayout;

    #[tokio::test]
    async fn can_generate_open_graph_image_for_a_tag() {
        let bytes = generate_tag_open_graph_image(GetTagOpenGraphDataResponse {
            id: "0".to_string(),
            name: "The quick brown fox jumped over the lazy dogs".to_string(),
            story_count: 1_344,
            follower_count: 1_520_000,
        })
        .await
        .unwrap();

        let og_image = image::load_from_memory(bytes.as_bytes())
            .expect("unable to load the open graph result")
            .into_rgb8();

        let expected_image = image::open("fixtures/tag.png")
            .expect("could not find the fixture image")
            .into_rgb8();

        let result = image_compare::rgb_hybrid_compare(&og_image, &expected_image)
            .expect("images had different dimensions");

        assert_eq!(result.score, 1.0);
    }
}
