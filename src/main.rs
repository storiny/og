use actix_cors::Cors;
use actix_extensible_rate_limit::{backend::SimpleInputFunctionBuilder, RateLimiter};
use actix_http::Method;
use actix_web::{http::header::ContentType, web, App, HttpResponse, HttpServer, Responder};
use dotenv::dotenv;
use redis::aio::ConnectionManager;
use std::{io, sync::Arc, time::Duration};
use storiny_og::{
    config::get_app_config,
    constants::redis_namespaces::RedisNamespace,
    routes,
    telemetry::{get_subscriber, init_subscriber},
};
use tracing::error;
use tracing_actix_web::TracingLogger;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

mod middlewares;

/// The 404 response handler.
async fn not_found() -> impl Responder {
    HttpResponse::NotFound()
        .content_type(ContentType::plaintext())
        .body("Not found")
}

fn main() -> io::Result<()> {
    dotenv().ok();

    match get_app_config() {
        Ok(config) => {
            if config.is_dev {
                let subscriber = get_subscriber("dev".to_string(), "info".to_string(), io::stdout);
                init_subscriber(subscriber);
            } else {
                tracing_subscriber::Registry::default()
                    .with(sentry::integrations::tracing::layer())
                    .init();
            }

            let _guard = sentry::init(sentry::ClientOptions {
                dsn: config.sentry_dsn.parse().ok(),
                release: sentry::release_name!(),
                traces_sample_rate: 0.8,
                before_send: Some(Arc::new(|event| {
                    if let Some(ref message) = event.message {
                        // Do not send client error response.
                        if message.starts_with("ClientError") {
                            return None;
                        }
                    }

                    Some(event)
                })),
                ..Default::default()
            });

            actix_web::rt::System::new().block_on(async {
                let host = config.host.to_string();
                let port = config.port.clone().parse::<u16>().unwrap();
                let redis_connection_string =
                    format!("redis://{}:{}", &config.redis_host, &config.redis_port);

                println!(
                    "Starting open graph service in {} mode at {}:{}",
                    if config.is_dev {
                        "development"
                    } else {
                        "production"
                    },
                    &host,
                    &port
                );

                // Rate-limit
                let redis_client = redis::Client::open(redis_connection_string.clone())
                    .expect("cannot build Redis client");
                let redis_connection_manager = match ConnectionManager::new(redis_client).await {
                    Ok(manager) => {
                        println!("Connected to Redis");
                        manager
                    }
                    Err(error) => {
                        error!("unable to connect to Redis: {error:?}");
                        std::process::exit(1);
                    }
                };

                let rate_limit_backend =
                    middlewares::rate_limiter::RedisBackend::builder(redis_connection_manager)
                        // Add prefix to avoid collisions with other services.
                        .key_prefix(Some(&format!("{}:", RedisNamespace::RateLimit)))
                        .build();

                let web_config = web::Data::new(config.clone());

                HttpServer::new(move || {
                    let input = SimpleInputFunctionBuilder::new(Duration::from_secs(5), 25) // 25 requests / 5s
                        .real_ip_key()
                        .build();

                    App::new()
                        .wrap(
                            RateLimiter::builder(rate_limit_backend.clone(), input)
                                .add_headers()
                                .build(),
                        )
                        .wrap(if config.is_dev {
                            Cors::permissive()
                        } else {
                            Cors::default()
                                .allow_any_origin()
                                .allow_any_header()
                                .allowed_methods(vec![
                                    Method::HEAD,
                                    Method::CONNECT,
                                    Method::OPTIONS,
                                    Method::GET,
                                ])
                                .max_age(3600)
                        })
                        .wrap(TracingLogger::default())
                        .wrap(actix_web::middleware::Compress::default())
                        .wrap(actix_web::middleware::NormalizePath::trim())
                        .app_data(web_config.clone())
                        .configure(routes::init_routes)
                        .default_service(web::route().to(not_found))
                })
                .bind((host, port))?
                .run()
                .await
            })
        }
        Err(error) => {
            eprintln!("environment configuration error: {:#?}", error);
            Ok(())
        }
    }
}
