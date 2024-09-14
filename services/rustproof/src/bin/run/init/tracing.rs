use crate::{ServiceError, TracingSetGlobalDefaultSnafu};
use clap::builder::TypedValueParser;
use rustproof::config::{LoggingConfig, ObservabilityConfig};
use snafu::ResultExt;
use std::io::stdout;
use std::path::Path;
use tracing::level_filters::LevelFilter;
use tracing_appender::rolling;
use tracing_appender::rolling::RollingFileAppender;
use tracing_subscriber::fmt::format::FmtSpan;
use tracing_subscriber::layer::SubscriberExt;
use tracing_subscriber::{fmt, EnvFilter, Registry};

pub fn init_observability(config: &ObservabilityConfig) -> Result<(), ServiceError> {
    // Initialize Logging
    init_logging(&config.logging)?;

    // // Initialize Tracing
    // if let Some(tracing_config) = &config.tracing {
    //     init_tracing(tracing_config)?;
    // }
    //
    // // Initialize Metrics
    // if let Some(metrics_config) = &config.metrics {
    //     init_metrics(metrics_config)?;
    // }

    Ok(())
}


use tracing_subscriber::Layer;
use rustproof::utils::flatten_json;

#[tracing::instrument(skip(config))]
pub fn init_logging(config: &LoggingConfig) -> Result<(), ServiceError> {
    // Initialize the EnvFilter with the default or provided log level
    let mut env_filter = EnvFilter::from_default_env();
    env_filter = env_filter.add_directive(config.level.as_str().parse().unwrap());


    // If any additional filter directives are provided in the config, add them
    if let Some(directives) = &config.env_filter_directives {
        for directive in directives {
            env_filter = env_filter.add_directive(directive.parse().unwrap());
        }
    }

    // File writer setup
    let file_layer = if let Some(log_path) = &config.file {
        let file_appender = RollingFileAppender::new(
            rolling::Rotation::NEVER,
            log_path.parent().unwrap_or_else(|| Path::new(".")),
            log_path.file_name()
                .and_then(|name| name.to_str())
                .unwrap_or("rustproof.log"),
        );

        Some(
            fmt::layer()
                .json()
                .with_span_events(FmtSpan::CLOSE)
                .with_writer(file_appender)
                .with_filter(LevelFilter::INFO),
        )
    } else {
        None
    };

    // Stdout writer setup
    let stdout_layer = fmt::layer()
        .compact()
        .with_span_events(FmtSpan::CLOSE)
        .with_writer(stdout)
        .with_filter(LevelFilter::DEBUG);

    // Combine layers into a subscriber
    let subscriber = Registry::default()
        .with(stdout_layer)
        .with(env_filter);

    match file_layer {
        Some(file_layer) => { tracing::subscriber::set_global_default(subscriber.with(file_layer)).context(TracingSetGlobalDefaultSnafu)? }
        None => { tracing::subscriber::set_global_default(subscriber).context(TracingSetGlobalDefaultSnafu)? }
    };




    Ok(())
}
//
//
// // pub fn init_tracing(config: &TracingConfig) {
// //     let mut env_filter = EnvFilter::from_default_env();
// //
// //     if let Some(directives) = &config.env_filter_directives {
// //         for directive in directives {
// //             env_filter = env_filter.add_directive(directive.parse().unwrap());
// //         }
// //     }
// //
// //     let max_level = config.max_level
// //         .as_deref()
// //         .unwrap_or("trace")
// //         .parse()
// //         .unwrap_or(Level::TRACE);
// //
// //     tracing_subscriber::fmt()
// //         .with_span_events(FmtSpan::CLOSE)
// //         .with_max_level(max_level)
// //         .with_env_filter(env_filter)
// //         .init();
// // }
// fn init_tracing(config: &TracingConfig) -> Result<(), ServiceError> {
//     if !config.tracing_enabled.unwrap_or(false) {
//         return Ok(());
//     }
//
//     let mut env_filter = EnvFilter::from_default_env();
//
//     if let Some(directives) = &config.tracing_env_filter_directives {
//         for directive in directives {
//             env_filter = env_filter.add_directive(directive.parse().unwrap());
//         }
//     }
//
//     let max_level = config
//         .tracing_max_level
//         .as_deref()
//         .unwrap_or("trace")
//         .parse()
//         .unwrap_or(Level::TRACE);
//
//     let service_name = config.tracing_service_name.clone().unwrap_or_else(|| "rustproof".into());
//
//     let tracer = match config.tracing_exporter.as_deref() {
//         Some("opentelemetry") => {
//             let tracer = opentelemetry_otlp::new_pipeline()
//                 .tracing()
//                 .with_trace_config(sdktrace::config().with_resource(Resource::new(vec![
//                     opentelemetry::KeyValue::new("service.name", service_name),
//                 ])))
//                 .install_batch(opentelemetry::runtime::Tokio)
//                 .map_err(|err| ServiceError::TracingError { source: err })?;
//             tracer
//         }
//         _ => sdktrace::TracerProvider::builder()
//             .build()
//             .versioned_tracer("rustproof", Some("v1.0.0"), None),
//     };
//
//     let telemetry_layer = tracing_opentelemetry::layer().with_tracer(tracer);
//
//     let fmt_layer = tracing_subscriber::fmt::layer()
//         .with_span_events(FmtSpan::CLOSE)
//         .with_max_level(max_level);
//
//     let subscriber = Registry::default()
//         .with(env_filter)
//         .with(fmt_layer)
//         .with(telemetry_layer);
//
//     set_global_default(subscriber).map_err(|err| ServiceError::TracingError { source: err })?;
//     Ok(())
// }
//
// fn init_metrics(config: &MetricsConfig) -> Result<PrometheusExporter, ServiceError> {
//     if !config.metrics_enabled.unwrap_or(false) {
//         return Ok(PrometheusExporter::default());
//     }
//
//     let meter = match config.metrics_exporter.as_deref() {
//         Some("prometheus") => opentelemetry_prometheus::exporter().init(),
//         _ => opentelemetry_sdk::export::metrics::stdout::new_exporter(),
//     };
//
//     let meter_provider = opentelemetry::global::meter_provider();
//
//     let controller = opentelemetry::sdk::metrics::controllers::BasicController::builder()
//         .with_meters(meter_provider)
//         .build(opentelemetry::runtime::Tokio)
//         .unwrap();
//
//     let _ = global::set_meter_provider(controller);
//     Ok(PrometheusExporter::default())
// }
