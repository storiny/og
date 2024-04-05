use serde::Serialize;
use strum::Display;

/// Redis cache namespace prefix. Refer to `redis_namespaces.md` at the main project root.
#[derive(Display, Debug, Serialize)]
pub enum RedisNamespace {
    #[strum(serialize = "og:l")]
    RateLimit,
}
