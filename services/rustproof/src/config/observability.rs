use serde::{Deserialize, Serialize};
use std::path::PathBuf;
use std::str::FromStr;
use std::string::ParseError;

#[derive(Deserialize, Debug, Serialize, Default)]
pub struct ObservabilityConfig {
    pub logging: LoggingConfig,
    pub tracing: Option<TracingConfig>,
    pub metrics: Option<MetricsConfig>,
}

#[derive(Deserialize, Debug, Serialize, Default)]
#[serde(rename_all = "lowercase")]
pub enum LogLevel {
    Trace,
    Debug,
    #[default]
    Info,
    Warn,
    Error,
}

impl FromStr for LogLevel {
    type Err = ParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            "trace" => Ok(LogLevel::Trace),
            "debug" => Ok(LogLevel::Debug),
            "info" => Ok(LogLevel::Info),
            "warn" => Ok(LogLevel::Warn),
            "error" => Ok(LogLevel::Error),
            _ => Ok(LogLevel::Info),
        }
    }
}

impl LogLevel {
    pub fn as_str(&self) -> &'static str {
        match self {
            LogLevel::Trace => "trace",
            LogLevel::Debug => "debug",
            LogLevel::Info => "info",
            LogLevel::Warn => "warn",
            LogLevel::Error => "error",
        }
    }
}

#[derive(Deserialize, Debug, Serialize, Default)]
#[serde(rename_all = "lowercase")]
pub enum LogFormat {
    #[default]
    Json,
    Plain,
}

#[derive(Deserialize, Debug, Serialize, Default)]
pub struct LoggingConfig {
    pub level: LogLevel,      // LOG_LEVEL, e.g., "debug", "info", "warn", "error"
    pub file: Option<PathBuf>,      // LOG_FILE, e.g., "/var/log/rustproof/auth.log"
    // pub file_format: LogFormat, // LOG_FILE_FORMAT, e.g., "json", "plain"
    pub env_filter_directives: Option<Vec<String>>, // LOG_ENV_FILTER_DIRECTIVES
}

#[derive(Deserialize, Debug, Serialize)]
pub struct TracingConfig {
    pub tracing_enabled: Option<bool>,              // GOTRUE_TRACING_ENABLED, default: false
    pub tracing_exporter: Option<String>,           // GOTRUE_TRACING_EXPORTER, e.g., "opentelemetry"
    pub tracing_service_name: Option<String>,       // OTEL_SERVICE_NAME, e.g., "auth"
    pub tracing_max_level: Option<String>,          // TRACING_MAX_LEVEL, e.g., "debug"
    pub tracing_env_filter_directives: Option<Vec<String>>, // TRACING_ENV_FILTER_DIRECTIVES
}

#[derive(Deserialize, Debug, Serialize)]
pub struct MetricsConfig {
    pub metrics_enabled: Option<bool>,              // GOTRUE_METRICS_ENABLED, default: false
    pub metrics_exporter: Option<String>,           // GOTRUE_METRICS_EXPORTER, e.g., "opentelemetry", "prometheus"
    pub metrics_prometheus_host: Option<String>,    // OTEL_EXPORTER_PROMETHEUS_HOST, default: "0.0.0.0"
    pub metrics_prometheus_port: Option<u16>,       // OTEL_EXPORTER_PROMETHEUS_PORT, default: 9100
}

