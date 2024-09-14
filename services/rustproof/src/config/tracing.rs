// use crate::config::TracingConfig;
// use tracing::Level;
// use tracing_subscriber::fmt::format::FmtSpan;
// use tracing_subscriber::EnvFilter;
//
// pub fn init_tracing(config: &TracingConfig) {
//     let mut env_filter = EnvFilter::from_default_env();
//
//     if let Some(directives) = &config.env_filter_directives {
//         for directive in directives {
//             env_filter = env_filter.add_directive(directive.parse().unwrap());
//         }
//     }
//
//     let max_level = config.max_level
//         .as_deref()
//         .unwrap_or("trace")
//         .parse()
//         .unwrap_or(Level::TRACE);
//
//     tracing_subscriber::fmt()
//         .with_span_events(FmtSpan::CLOSE)
//         .with_max_level(max_level)
//         .with_env_filter(env_filter)
//         .init();
// }