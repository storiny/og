use actix_web::web;

mod health;
mod index;
mod stories;
mod tags;

/// Registers the routes.
///
/// * `cfg` - Web service config
pub fn init_routes(cfg: &mut web::ServiceConfig) {
    index::init_routes(cfg);
    health::init_routes(cfg);
    tags::init_routes(cfg);
    stories::init_routes(cfg);
}
