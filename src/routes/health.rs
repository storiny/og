use crate::error::AppError;
use actix_web::{
    get,
    web,
    HttpResponse,
};

#[get("/health")]
#[tracing::instrument(name = "GET /health", skip_all, err)]
async fn get() -> Result<HttpResponse, AppError> {
    Ok(HttpResponse::Ok().body("OK"))
}

pub fn init_routes(cfg: &mut web::ServiceConfig) {
    cfg.service(get);
}
