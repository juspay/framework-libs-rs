//! `log_utils` provides a configurable logging infrastructure.
//!
//! When the `tracing` feature is enabled, this crate provides:
//!
//! - A [`JsonFormattingLayer`] for customizable JSON log output.
//! - A [`SpanStorageLayer`] layer to capture span data.
//! - A central [`build_logging_components`] function to construct logging layers and guards,
//!   based on the specified configuration.
//!
//! This crate aims to provide a generic logging solution that can be easily integrated into
//! various applications, allowing consumers to combine the returned components with their
//! own [`tracing_subscriber::Registry`] and other layers.
//!
//! # Features
//!
//! - `tracing` - Enables `tracing`-based logging infrastructure (disabled by default)
//!
//! # Example
//!
//! ```toml
//! [dependencies]
//! log_utils = { version = "0.1", features = ["tracing"] }
//! ```
//!
//! ```
//! use std::{
//!     collections::{HashMap, HashSet},
//!     num::NonZeroUsize,
//! };
//!
//! use log_utils::{
//!     AdditionalFieldsPlacement, ConsoleLogFormat, ConsoleLoggingConfig, DirectivePrintTarget,
//!     FileLoggingConfig, Level, LoggerConfig, Rotation, build_logging_components,
//! };
//! use serde_json::json;
//! use tracing_subscriber::{Layer, layer::SubscriberExt, util::SubscriberInitExt};
//!
//! // Define keys for static fields
//! const SERVICE: &str = "service";
//! const ENV: &str = "env";
//! const VERSION: &str = "version";
//! const BUILD: &str = "build";
//!
//! // Create static top-level fields
//! let static_fields = HashMap::from([
//!     (SERVICE.to_string(), json!("my_app")),
//!     (ENV.to_string(), json!("development")),
//!     (VERSION.to_string(), json!("0.1.0")),
//!     (BUILD.to_string(), json!("local_dev_build")),
//! ]);
//!
//! let config = LoggerConfig {
//!     static_top_level_fields: static_fields,
//!     top_level_keys: HashSet::new(),
//!     persistent_keys: HashSet::new(),
//!     log_span_lifecycles: false,
//!     additional_fields_placement: AdditionalFieldsPlacement::TopLevel,
//!     file_config: Some(FileLoggingConfig {
//!         directory: std::env::temp_dir().to_string_lossy().to_string(),
//!         file_name_prefix: "my_app_log".to_string(),
//!         file_rotation: Rotation::DAILY,
//!         max_log_files: NonZeroUsize::new(7),
//!         level: Level::INFO,
//!         filtering_directive: Some("my_app=info,warn".to_string()),
//!         print_filtering_directive: DirectivePrintTarget::Stdout,
//!     }),
//!     console_config: Some(ConsoleLoggingConfig {
//!         level: Level::DEBUG,
//!         log_format: ConsoleLogFormat::HumanReadable,
//!         filtering_directive: Some("my_app=debug,info".to_string()),
//!         print_filtering_directive: DirectivePrintTarget::Stdout,
//!     }),
//!     global_filtering_directive: Some("info".to_string()),
//! };
//!
//! match build_logging_components(config) {
//!     Ok(components) => {
//!         let _guards = components.guards; // Keep guards in scope
//!
//!         // Build the subscriber with all components
//!         let mut layers = Vec::new();
//!         layers.push(components.storage_layer.boxed());
//!
//!         if let Some(file_layer) = components.file_log_layer {
//!             layers.push(file_layer);
//!         }
//!         if let Some(console_layer) = components.console_log_layer {
//!             layers.push(console_layer);
//!         }
//!
//!         // Initialize the global subscriber
//!         tracing_subscriber::registry().with(layers).init();
//!
//!         tracing::info!("Logging initialized successfully!");
//!     }
//!     Err(e) => eprintln!("Failed to initialize logging: {e}"),
//! }
//! ```

#![cfg_attr(docsrs, feature(doc_auto_cfg))]
#![doc(test(attr(deny(warnings))))]

#[cfg(feature = "tracing")]
mod tracing;

#[cfg(feature = "tracing")]
pub use self::tracing::{
    AdditionalFieldsPlacement, ConsoleLogFormat, ConsoleLoggingConfig, DirectivePrintTarget,
    FileLoggingConfig, JsonFormattingLayer, JsonFormattingLayerConfig, Level, LoggerConfig,
    LoggerError, LoggingComponents, RecordType, Rotation, SpanStorageLayer,
    build_logging_components,
};
