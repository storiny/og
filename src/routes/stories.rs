use crate::{
    config::Config,
    constants::{
        fonts::{
            CABINET_GROTESK_EXTRABOLD,
            SATOSHI_MEDIUM,
            SATOSHI_REGULAR,
        },
        icons::DEFAULT_AVATAR,
    },
    error::AppError,
    grpc::defs::open_graph_def::v1::{
        GetStoryOpenGraphDataRequest,
        GetStoryOpenGraphDataResponse,
    },
    utils::{
        construct_svg_tree,
        escape_string,
        get_pixmap,
        get_text_width,
        get_thumbnail_url,
        process_text,
        truncate_text,
        wrap_text,
    },
    AppState,
    StoryTemplate,
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

#[get("/stories/{id}")]
#[tracing::instrument(
    name = "GET /stories/{id}",
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
        .map_err(|_| AppError::from("Invalid story ID"))?;

    let response = {
        let grpc_client = &state.grpc_client;
        let mut client = grpc_client.lock().await;

        client
            .get_story_open_graph_data(tonic::Request::new(GetStoryOpenGraphDataRequest {
                id: id.to_string(),
            }))
            .await
            .map_err(|status| {
                if matches!(status.code(), Code::NotFound) {
                    AppError::ClientError(StatusCode::NOT_FOUND, "Story not found".to_string())
                } else {
                    AppError::TonicError(status)
                }
            })?
            .into_inner()
    };

    if response.is_private {
        return Ok(HttpResponse::Forbidden().body("This story is private"));
    }

    let bytes = generate_story_open_graph_image(&state.config, response).await?;
    let mut builder = HttpResponse::Ok();

    builder.insert_header(CacheControl(vec![
        CacheDirective::Public,
        CacheDirective::MaxAge(86400_u32), // 24 hours
    ]));

    Ok(builder.content_type(mime::IMAGE_PNG).body(bytes))
}

/// Renders the open graph image for a story.
///
/// * `config` - The environment configuration.
/// * `response` - The story open graph response.
async fn generate_story_open_graph_image(
    config: &Config,
    response: GetStoryOpenGraphDataResponse,
) -> Result<Vec<u8>, AppError> {
    let mut pixmap = get_pixmap()?;

    let splash_url = response
        .splash_id
        .map(|id| get_thumbnail_url(&config.cdn_server_url, id.as_str()));
    let user_avatar_url = response
        .user_avatar_id
        .map(|id| get_thumbnail_url(&config.cdn_server_url, id.as_str()))
        .unwrap_or(DEFAULT_AVATAR.to_string());

    let read_count = abbrev_num(response.read_count as isize, None).unwrap_or_default();
    let like_count = abbrev_num(response.like_count as isize, None).unwrap_or_default();
    let comment_count = abbrev_num(response.comment_count as isize, None).unwrap_or_default();

    let mut title_offset = "0";
    let mut max_description_lines = 3;

    let title = {
        let title_lines = wrap_text(&process_text(&response.title), 920, Some(2), |text| {
            get_text_width(&CABINET_GROTESK_EXTRABOLD, 46.0, text) as usize
        });

        if title_lines.len() == 1 {
            title_offset = "-60";
            max_description_lines = 4;
        }

        title_lines
            .iter()
            .enumerate()
            .map(|(index, line)| {
                format!(
                    r#"<tspan x="40" {}>{}</tspan>"#,
                    if index == 0 {
                        r#"y="81.57""#
                    } else {
                        r#"dy="1.25em""#
                    },
                    escape_string(line)
                )
            })
            .collect::<Vec<_>>()
            .join("")
    };

    let description = response.description.map(|value| {
        wrap_text(
            &process_text(value.as_str()),
            if splash_url.is_some() { 430 } else { 760 },
            Some(max_description_lines),
            |text| get_text_width(&SATOSHI_REGULAR, 30.0, text) as usize,
        )
        .iter()
        .enumerate()
        .map(|(index, line)| {
            format!(
                r#"<tspan x="40" {}>{}</tspan>"#,
                if index == 0 {
                    r#"y="308.133""#
                } else {
                    r#"dy="1.3em""#
                },
                escape_string(line)
            )
        })
        .collect::<Vec<_>>()
        .join("")
    });

    let user_name = truncate_text(
        &process_text(&response.user_name),
        if splash_url.is_some() { 360 } else { 640 },
        |text| get_text_width(&SATOSHI_MEDIUM, 32.0, text) as usize,
    );

    let svg = StoryTemplate {
        title,
        description,
        splash_url,
        read_count,
        like_count,
        comment_count,
        user_name,
        user_avatar_url,
        title_offset: title_offset.to_string(),
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
    use crate::config::get_app_config;
    use image::EncodableLayout;

    #[tokio::test]
    async fn can_generate_open_graph_image_for_a_story() {
        let config = get_app_config().unwrap();
        let bytes = generate_story_open_graph_image(
            &config,
            GetStoryOpenGraphDataResponse {
                id: "0".to_string(),
                title: "The quick brown fox jumps over the lazy dog's back, vaulting effortlessly, while the hazy sky blankets the quaint village, igniting whispered conversations amongst jovial townsfolk.".to_string(),
                description: Some("Jumbled thoughts quickly zapped by fox, quirky vixen whispers joyfully; amid fancy, exploring grand, hazy jungles, kickboxing lions menace, nervously observing playful quetzals, rambunctious sloths, tigers, ungulates, vexing wolverines, xylophones yielding zany antics.".to_string()),
                splash_id: None,
                read_count: 9_600_000,
                like_count: 120_000,
                comment_count: 1_890,
                is_private: false,
                user_name: "Ian Weaver".to_string(),
                user_avatar_id: None,
            },
        )
            .await
            .unwrap();

        let og_image = image::load_from_memory(bytes.as_bytes())
            .expect("unable to load the open graph result")
            .into_rgb8();

        let expected_image = image::open("fixtures/story.png")
            .expect("could not find the fixture image")
            .into_rgb8();

        let result = image_compare::rgb_hybrid_compare(&og_image, &expected_image)
            .expect("images had different dimensions");

        assert_eq!(result.score, 1.0);
    }

    #[tokio::test]
    async fn can_generate_open_graph_image_for_a_story_with_external_images() {
        let config = get_app_config().unwrap();
        let bytes = generate_story_open_graph_image(
            &config,
            GetStoryOpenGraphDataResponse {
                id: "0".to_string(),
                title: "The quick brown fox jumps over the lazy dog's back, vaulting effortlessly, while the hazy sky blankets the quaint village, igniting whispered conversations amongst jovial townsfolk.".to_string(),
                description: Some("Jumbled thoughts quickly zapped by fox, quirky vixen whispers joyfully; amid fancy, exploring grand, hazy jungles, kickboxing lions menace, nervously observing playful quetzals, rambunctious sloths, tigers, ungulates, vexing wolverines, xylophones yielding zany antics.".to_string()),
                splash_id: Some(
                    "https://gist.github.com/assets/77036902/03350aca-511c-4007-9eff-1bddcd7b5c33"
                        .to_string(),
                ),
                read_count: 9_600_000,
                like_count: 120_000,
                comment_count: 1_890,
                is_private: false,
                user_name: "Ian Weaver".to_string(),
                user_avatar_id: Some(
                    "https://gist.github.com/assets/77036902/23e75ee6-25d6-4844-b005-b17a099cebd6"
                        .to_string(),
                ),
            },
        )
            .await
            .unwrap();

        let og_image = image::load_from_memory(bytes.as_bytes())
            .expect("unable to load the open graph result")
            .into_rgb8();

        let expected_image = image::open("fixtures/story_with_external_images.png")
            .expect("could not find the fixture image")
            .into_rgb8();

        let result = image_compare::rgb_hybrid_compare(&og_image, &expected_image)
            .expect("images had different dimensions");

        assert_eq!(result.score, 1.0);
    }
}
