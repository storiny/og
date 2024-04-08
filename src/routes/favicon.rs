use crate::error::AppError;
use actix_web::{
    get,
    web,
    HttpResponse,
};
use tracing::error;

const FAVICON: &[u8] = include_bytes!("../../static/favicon.ico");

#[get("/favicon.ico")]
#[tracing::instrument(name = "GET /favicon.ico", skip_all, err)]
async fn get() -> Result<HttpResponse, AppError> {
    match "image/x-icon".parse::<mime::Mime>() {
        Ok(mime_type) => Ok(HttpResponse::Ok().content_type(mime_type).body(FAVICON)),
        Err(error) => {
            error!("{error:?}");
            Ok(HttpResponse::NotFound().finish())
        }
    }
}

pub fn init_routes(cfg: &mut web::ServiceConfig) {
    cfg.service(get);
}
