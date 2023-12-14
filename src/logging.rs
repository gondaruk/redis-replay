use crate::cli;

use tracing_subscriber::prelude::*;

pub fn init() {
    let default_filter = if cli::is_verbose() {
        "debug"
    } else if cli::is_silent() {
        "error"
    } else {
        "info"
    };

    tracing_log::LogTracer::init().expect("failed to init tracing_log");
    let env_filter = tracing_subscriber::EnvFilter::try_from_env("ATLAS_LOG")
        .unwrap_or_else(|_| tracing_subscriber::EnvFilter::new(default_filter));
    let fmt_layer = tracing_subscriber::fmt::layer()
        .with_thread_ids(true)
        .with_target(true)
        .with_line_number(true)
        .with_span_events(
            tracing_subscriber::fmt::format::FmtSpan::NEW
                | tracing_subscriber::fmt::format::FmtSpan::CLOSE,
        );
    let subscriber = tracing_subscriber::Registry::default()
        .with(env_filter)
        .with(fmt_layer);
    tracing::subscriber::set_global_default(subscriber).expect("failed to subscribe to traces");
}
