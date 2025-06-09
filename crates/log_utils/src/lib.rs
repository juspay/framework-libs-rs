//! `log_utils` provides a configurable logging infrastructure based on the [`tracing`] ecosystem.
//!
//! It offers:
//! - A [`JsonFormattingLayer`] for customizable JSON log output.
//! - A [`SpanStorageLayer`] layer to capture span data.
//! - A central [`build_logging_components`] function to construct logging layers and guards,
//!   based on the specified configuration.
//!
//! This crate aims to provide a generic logging solution that can be easily integrated into
//! various applications, allowing consumers to combine the returned components with their
//! own [`tracing_subscriber::Registry`] and other layers.

mod formatter;
mod storage;

use std::collections::{HashMap, HashSet};

use serde_json::Value;
pub use tracing::Level;
pub use tracing_appender::rolling::Rotation;
use tracing_subscriber::{EnvFilter, Layer};

pub use self::{
    formatter::{JsonFormattingLayer, JsonFormattingLayerConfig, RecordType},
    storage::SpanStorageLayer,
};

mod keys {
    use std::sync::LazyLock;

    use rustc_hash::FxHashSet;

    pub(crate) const MESSAGE: &str = "message";
    pub(crate) const LEVEL: &str = "level";
    pub(crate) const TARGET: &str = "target";
    pub(crate) const LINE: &str = "line";
    pub(crate) const FILE: &str = "file";
    pub(crate) const TIME: &str = "time";
    pub(crate) const HOSTNAME: &str = "hostname";
    pub(crate) const PID: &str = "pid";
    pub(crate) const FN: &str = "fn";
    pub(crate) const FULL_NAME: &str = "full_name";
    pub(crate) const ELAPSED_MILLISECONDS: &str = "elapsed_milliseconds";

    pub(crate) static IMPLICIT_KEYS: LazyLock<FxHashSet<&'static str>> = LazyLock::new(|| {
        [
            MESSAGE, LEVEL, TARGET, LINE, FILE, TIME, HOSTNAME, PID, FN, FULL_NAME,
        ]
        .iter()
        .copied()
        .collect()
    });
}

/// Comprehensive configuration for the entire logging system.
#[derive(Debug, Clone)]
pub struct LoggerConfig {
    /// A map of key-value pairs that are statically defined at initialization and included
    /// at the top level of every log entry.
    pub static_top_level_fields: HashMap<String, Value>,

    /// A set of keys which will be promoted to the top level of the JSON output,
    /// if they appear in a log event or span's dynamic data.
    pub top_level_keys: HashSet<&'static str>,

    /// Keys whose values from spans should be propagated to parent spans,
    /// if they set in the current span.
    /// This is useful for capturing context that should be visible in parent spans,
    /// such as merchant IDs, user IDs, etc.
    pub persistent_keys: HashSet<&'static str>,

    /// If `true`, logs all span entries and exits.
    /// If `false`, does not log span entries and only logs exits for root spans.
    pub log_span_lifecycles: bool,

    /// Specifies how additional fields (not designated as top-level) are placed in the JSON output.
    pub additional_fields_placement: AdditionalFieldsPlacement,

    /// Configuration for file logging. If `None`, file logging is disabled.
    pub file_config: Option<FileLoggingConfig>,

    /// Configuration for console logging. If `None`, console logging is disabled.
    pub console_config: Option<ConsoleLoggingConfig>,

    /// A global [`EnvFilter`] directive (e.g., `"info,my_crate=debug"`) for filtering log events.
    /// This directive may be overridden by specific directives in
    /// [`FileLoggingConfig`] or [`ConsoleLoggingConfig`].
    /// This allows using a less verbose log level for third-party crates,
    /// while using a more verbose level for first-party crates, for example.
    pub global_filtering_directive: Option<String>,
}

/// Configuration for file logging.
#[derive(Debug, Clone)]
pub struct FileLoggingConfig {
    /// Directory where log files will be stored.
    pub directory: String,

    /// Prefix for log file names.
    pub file_name_prefix: String,

    /// Rotation strategy for log files.
    pub file_rotation: Rotation,

    /// Maximum number of log files to keep. If `None`, all files are kept.
    pub max_log_files: Option<std::num::NonZeroUsize>,

    /// Minimum log level for file logs.
    pub level: Level,

    /// [`EnvFilter`] directive specific to file logs, overriding the global filtering directive.
    /// If `None`, the global filtering directive is used.
    pub filtering_directive: Option<String>,

    /// Specifies where to print the effective filtering directive for file logs.
    pub print_filtering_directive: DirectivePrintTarget,
}

/// Configuration for console logging.
#[derive(Debug, Clone)]
pub struct ConsoleLoggingConfig {
    /// Minimum log level for console logs.
    pub level: Level,

    /// Output format for console logs.
    pub log_format: ConsoleLogFormat,

    /// [`EnvFilter`] directive specific to console logs, overriding the global filtering directive.
    /// If `None`, the global filtering directive is used.
    pub filtering_directive: Option<String>,

    /// Specifies where to print the effective filtering directive for console logs.
    pub print_filtering_directive: DirectivePrintTarget,
}

/// Specifies where (if at all) to print the effective filtering directive during logger setup.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DirectivePrintTarget {
    /// Print to standard output.
    Stdout,

    /// Print to standard error.
    Stderr,

    /// Do not print the directive.
    None,
}

/// Defines the output format for console logging.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ConsoleLogFormat {
    /// Pretty-printed, human-readable, multi-line format.
    HumanReadable,

    /// Compact, single-line JSON format.
    CompactJson,

    /// Pretty-printed, multi-line JSON format.
    PrettyJson,
}

/// Defines how additional (non-top-level, non-implicit) fields are placed in the JSON log output.
#[derive(Debug, Clone)]
pub enum AdditionalFieldsPlacement {
    /// Log all additional fields at the top level of the JSON object.
    TopLevel,

    /// Nest all additional fields under the specified key.
    Nested(String),
}

impl AdditionalFieldsPlacement {
    pub(crate) fn is_nested(&self) -> bool {
        matches!(self, Self::Nested(_))
    }
}

/// Holds the constructed logging layers and their associated worker guards.
/// These components can be combined with other layers and a [`tracing_subscriber::Registry`]
/// before initializing the global subscriber.
#[allow(missing_debug_implementations)] // File and console layers are `dyn Trait` objects
pub struct LoggingComponents {
    /// The layer responsible for storing span data.
    pub storage_layer: SpanStorageLayer,

    /// The file logging layer, if enabled and configured.
    pub file_log_layer:
        Option<Box<dyn Layer<tracing_subscriber::Registry> + Send + Sync + 'static>>,

    /// The console logging layer, if enabled and configured.
    pub console_log_layer:
        Option<Box<dyn Layer<tracing_subscriber::Registry> + Send + Sync + 'static>>,

    /// Worker guards for file and console logging layers.
    /// Logs would be written as long as these guards are in scope.
    pub guards: Vec<tracing_appender::non_blocking::WorkerGuard>,
}

/// Errors that can occur within the logger.
#[derive(Debug, thiserror::Error)]
pub enum LoggerError {
    /// Represents an error in configuration.
    #[error("Configuration error: {0}")]
    Configuration(String),

    /// Represents an error during JSON serialization.
    #[error("JSON serialization error: {0}")]
    JsonSerialization(#[from] serde_json::Error),

    /// Represents an error during initialization of the rolling file appender.
    #[error("Failed to initialize rolling file appender: {0}")]
    FileAppenderInitialization(#[from] tracing_appender::rolling::InitError),

    /// Represents an error due to an invalid filtering directive.
    #[error("Failed to parse filtering directive: {0}")]
    InvalidFilteringDirective(#[from] tracing_subscriber::filter::ParseError),
}

/// Constructs logging components based on the provided [`LoggerConfig`].
///
/// This function prepares a [`SpanStorageLayer`], and optionally file and console logging layers,
/// along with any necessary worker guards for the file and console logging layers.
/// These components are returned in a [`LoggingComponents`] struct, allowing the caller
/// to integrate them with a [`tracing_subscriber::Registry`] and other custom layers before
/// initializing the global `tracing` subscriber.
///
/// # Example
///
/// ```
/// use std::{
///     collections::{HashMap, HashSet},
///     num::NonZeroUsize,
/// };
///
/// use log_utils::{
///     AdditionalFieldsPlacement, ConsoleLogFormat, ConsoleLoggingConfig, DirectivePrintTarget,
///     FileLoggingConfig, Level, LoggerConfig, Rotation, build_logging_components,
/// };
/// use serde_json::json;
/// use tracing_subscriber::{Layer, layer::SubscriberExt, util::SubscriberInitExt};
///
/// // Define keys for static fields
/// const SERVICE: &str = "service";
/// const ENV: &str = "env";
/// const VERSION: &str = "version";
/// const BUILD: &str = "build";
///
/// // Create static top-level fields
/// let static_fields = HashMap::from([
///     (SERVICE.to_string(), json!("my_app")),
///     (ENV.to_string(), json!("development")),
///     (VERSION.to_string(), json!("0.1.0")),
///     (BUILD.to_string(), json!("local_dev_build")),
/// ]);
///
/// let config = LoggerConfig {
///     static_top_level_fields: static_fields,
///     top_level_keys: HashSet::new(),
///     persistent_keys: HashSet::new(),
///     log_span_lifecycles: false,
///     additional_fields_placement: AdditionalFieldsPlacement::TopLevel,
///     file_config: Some(FileLoggingConfig {
///         directory: "logs".to_string(),
///         file_name_prefix: "my_app_log".to_string(),
///         file_rotation: Rotation::DAILY,
///         max_log_files: NonZeroUsize::new(7),
///         level: Level::INFO,
///         filtering_directive: Some("my_app=info,warn".to_string()),
///         print_filtering_directive: DirectivePrintTarget::Stdout,
///     }),
///     console_config: Some(ConsoleLoggingConfig {
///         level: Level::DEBUG,
///         log_format: ConsoleLogFormat::HumanReadable,
///         filtering_directive: Some("my_app=debug,info".to_string()),
///         print_filtering_directive: DirectivePrintTarget::Stdout,
///     }),
///     global_filtering_directive: Some("info".to_string()),
/// };
///
/// match build_logging_components(config) {
///     Ok(components) => {
///         let _guards = components.guards; // Keep guards in scope
///
///         // Build the subscriber with all components
///         let mut layers = Vec::new();
///         layers.push(components.storage_layer.boxed());
///
///         if let Some(file_layer) = components.file_log_layer {
///             layers.push(file_layer);
///         }
///         if let Some(console_layer) = components.console_log_layer {
///             layers.push(console_layer);
///         }
///
///         // Initialize the global subscriber
///         tracing_subscriber::registry().with(layers).init();
///
///         tracing::info!("Logging initialized successfully!");
///     }
///     Err(e) => eprintln!("Failed to initialize logging: {e}"),
/// }
/// ```
///
/// # Errors
///
/// Returns [`LoggerError`] if any part of the component building fails
/// (e.g., due to invalid configuration, invalid filter directives, etc.).
pub fn build_logging_components(config: LoggerConfig) -> Result<LoggingComponents, LoggerError> {
    let mut guards = Vec::new();

    let storage_layer = SpanStorageLayer::new(config.persistent_keys);

    let json_formatting_config = JsonFormattingLayerConfig {
        static_top_level_fields: config.static_top_level_fields,
        top_level_keys: config.top_level_keys,
        log_span_lifecycles: config.log_span_lifecycles,
        additional_fields_placement: config.additional_fields_placement,
    };

    // File logging
    let file_log_layer: Option<
        Box<dyn Layer<tracing_subscriber::Registry> + Send + Sync + 'static>,
    > = if let Some(file_logging_config) = config.file_config {
        let mut file_appender_builder = tracing_appender::rolling::RollingFileAppender::builder()
            .rotation(file_logging_config.file_rotation)
            .filename_prefix(file_logging_config.file_name_prefix);

        if let Some(max_log_files) = file_logging_config.max_log_files {
            file_appender_builder = file_appender_builder.max_log_files(usize::from(max_log_files));
        }

        let file_appender = file_appender_builder.build(&file_logging_config.directory)?;
        let (non_blocking_appender, guard) = tracing_appender::non_blocking(file_appender);
        guards.push(guard);

        let file_filter_directive = file_logging_config
            .filtering_directive
            .as_deref()
            .or(config.global_filtering_directive.as_deref())
            .unwrap_or_default(); // Using an empty string causes it to use the default directive

        match file_logging_config.print_filtering_directive {
            #[allow(clippy::print_stdout)]
            DirectivePrintTarget::Stdout => {
                println!(
                    "[INFO] {}: Using file filtering directive: {file_filter_directive}",
                    env!("CARGO_PKG_NAME")
                );
            }
            #[allow(clippy::print_stderr)]
            DirectivePrintTarget::Stderr => {
                eprintln!(
                    "[INFO] {}: Using file filtering directive: {file_filter_directive}",
                    env!("CARGO_PKG_NAME")
                );
            }
            DirectivePrintTarget::None => (), // Do nothing
        }

        let filter = EnvFilter::builder()
            .with_default_directive(file_logging_config.level.into())
            .parse(file_filter_directive)?;

        let layer = JsonFormattingLayer::new(
            json_formatting_config.clone(),
            non_blocking_appender,
            serde_json::ser::CompactFormatter,
        )?
        .with_filter(filter)
        .boxed();

        Some(layer)
    } else {
        None
    };

    // Console logging
    let console_log_layer: Option<
        Box<dyn Layer<tracing_subscriber::Registry> + Send + Sync + 'static>,
    > = if let Some(console_logging_config) = config.console_config {
        let (non_blocking_stdout, guard) = tracing_appender::non_blocking(std::io::stdout());
        guards.push(guard);

        let console_filter_directive = console_logging_config
            .filtering_directive
            .as_deref()
            .or(config.global_filtering_directive.as_deref())
            .unwrap_or_default(); // Using an empty string causes it to use the default directive

        match console_logging_config.print_filtering_directive {
            #[allow(clippy::print_stdout)]
            DirectivePrintTarget::Stdout => {
                println!(
                    "[INFO] {}: Using console filtering directive: {console_filter_directive}",
                    env!("CARGO_PKG_NAME")
                );
            }
            #[allow(clippy::print_stderr)]
            DirectivePrintTarget::Stderr => {
                eprintln!(
                    "[INFO] {}: Using console filtering directive: {console_filter_directive}",
                    env!("CARGO_PKG_NAME")
                );
            }
            DirectivePrintTarget::None => (), // Do nothing
        }

        let filter = EnvFilter::builder()
            .with_default_directive(console_logging_config.level.into())
            .parse(console_filter_directive)?;

        match console_logging_config.log_format {
            ConsoleLogFormat::HumanReadable => {
                let human_readable_layer = tracing_subscriber::fmt::layer()
                    .with_timer(tracing_subscriber::fmt::time::time())
                    .pretty()
                    .with_writer(non_blocking_stdout)
                    .with_filter(filter)
                    .boxed();
                Some(human_readable_layer)
            }
            ConsoleLogFormat::CompactJson => {
                let json_layer = JsonFormattingLayer::new(
                    json_formatting_config,
                    non_blocking_stdout,
                    serde_json::ser::CompactFormatter,
                )?
                .with_filter(filter)
                .boxed();
                Some(json_layer)
            }
            ConsoleLogFormat::PrettyJson => {
                let pretty_json_layer = JsonFormattingLayer::new(
                    json_formatting_config,
                    non_blocking_stdout,
                    serde_json::ser::PrettyFormatter::new(),
                )?
                .with_filter(filter)
                .boxed();
                Some(pretty_json_layer)
            }
        }
    } else {
        None
    };

    Ok(LoggingComponents {
        storage_layer,
        file_log_layer,
        console_log_layer,
        guards,
    })
}
