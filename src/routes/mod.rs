use actix_web::web;

mod favicon;
mod health;
mod index;
mod robots;

/// Registers the routes.
///
/// * `cfg` - Web service config
pub fn init_routes(cfg: &mut web::ServiceConfig) {
    index::init_routes(cfg);
    health::init_routes(cfg);
    favicon::init_routes(cfg);
    robots::init_routes(cfg);
}
