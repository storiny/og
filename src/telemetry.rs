use actix_web::rt::task::JoinHandle;
use tracing::{
    subscriber::set_global_default,
    Subscriber,
};
use tracing_bunyan_formatter::{
    BunyanFormattingLayer,
    JsonStorageLayer,
};
use tracing_subscriber::{
    fmt::MakeWriter,
    layer::SubscriberExt,
    EnvFilter,
    Registry,
};

/// Composes multiple layers into a tracing's subscriber.
///
/// We are using `impl Subscriber` as return type to avoid having to spell out the actual
/// type of the returned subscriber, which is indeed quite complex.
pub fn get_subscriber<Sink>(
    name: String,
    env_filter: String,
    sink: Sink,
) -> impl Subscriber + Sync + Send
where
    Sink: for<'a> MakeWriter<'a> + Send + Sync + 'static,
{
    let env_filter =
        EnvFilter::try_from_default_env().unwrap_or_else(|_| EnvFilter::new(env_filter));
    let formatting_layer = BunyanFormattingLayer::new(name, sink);
    Registry::default()
        .with(env_filter)
        .with(JsonStorageLayer)
        .with(formatting_layer)
}

/// Registers a subscriber as global default to process span data.
///
/// This should only be called once.
pub fn init_subscriber(subscriber: impl Subscriber + Sync + Send) {
    #[allow(clippy::expect_used)]
    set_global_default(subscriber).expect("failed to set subscriber");
}

pub fn spawn_blocking_with_tracing<F, R>(f: F) -> JoinHandle<R>
where
    F: FnOnce() -> R + Send + 'static,
    R: Send + 'static,
{
    let current_span = tracing::Span::current();
    actix_web::rt::task::spawn_blocking(move || current_span.in_scope(f))
}
