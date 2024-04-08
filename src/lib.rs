#![forbid(unsafe_code)]
#![allow(clippy::module_inception)]
#![deny(clippy::expect_used, clippy::unwrap_used)]

use crate::grpc::defs::grpc_service::v1::api_service_client::ApiServiceClient;
use sailfish::TemplateOnce;
use tokio::sync::Mutex;
use tonic::{
    codegen::InterceptedService,
    transport::Channel,
    Request,
    Status,
};

pub mod config;
pub mod constants;
pub mod error;
pub mod grpc;
pub mod routes;
pub mod telemetry;
pub mod utils;

pub type AuthInterceptor = Box<dyn Fn(Request<()>) -> Result<Request<()>, Status> + Send>;
pub type GrpcClient = ApiServiceClient<InterceptedService<Channel, AuthInterceptor>>;

/// The application state.
pub struct AppState {
    /// The environment configuration.
    pub config: config::Config,
    /// The gRPC service client.
    pub grpc_client: Mutex<GrpcClient>,
}

// Story open graph image template.
#[derive(TemplateOnce)]
#[template(path = "story.stpl")]
pub struct StoryTemplate {
    /// The title of the story.
    title: String,
    /// The optional description of story.
    description: Option<String>,
    /// The optional splash image URL for the story.
    splash_url: Option<String>,
    /// The name of author of the story.
    user_name: String,
    /// The avatar image URL of author of the story.
    user_avatar_url: String,
    /// The read count value for the story.
    read_count: String,
    /// The like count value for the story.
    like_count: String,
    /// The comment count value for the story.
    comment_count: String,
    /// The vertical translation parameter for the persona and description. It depends on whether
    /// the title is a single line or multiline.
    title_offset: String,
}

// Tag open graph image template.
#[derive(TemplateOnce)]
#[template(path = "tag.stpl")]
pub struct TagTemplate {
    /// The name of the tag without `#` prefix.
    name: String,
    /// The follower count value for the tag.
    follower_count: String,
    /// The story count value for the tag.
    story_count: String,
}
