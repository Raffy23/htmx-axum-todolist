use tracing_subscriber::fmt::format::FmtSpan;
use tracing_subscriber::layer::SubscriberExt;
use tracing_subscriber::prelude::*;
use tracing_subscriber::{filter::EnvFilter, fmt};

pub fn init_tracing() {
    let filter_layer = EnvFilter::try_from_default_env()
        .or_else(|_| EnvFilter::try_new("trace"))
        .unwrap();

    let stdout_layer = fmt::layer()
        .with_span_events(FmtSpan::CLOSE)
        .with_target(true);
        //.with_ansi(false);

    tracing_subscriber::registry()
        .with(filter_layer)
        .with(stdout_layer)
        .init();
}
