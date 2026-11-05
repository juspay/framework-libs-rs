//! `metrics_utils` provides utilities for instrumenting Rust applications with OpenTelemetry
//! metrics.
//!
//! When the `opentelemetry-otlp` or `opentelemetry-prometheus` feature is enabled, this crate
//! provides:
//!
//! - A [`MetricsConfig`] struct for configuring the metrics system.
//! - An [`init_metrics()`] function to construct the metrics pipeline from configuration.
//! - A [`MetricsHandle`] for registering the global provider, accessing the Prometheus registry,
//!   and shutting down the pipeline.
//! - Declarative macros ([`global_meter!`], [`counter_metric!`], [`up_down_counter_metric!`],
//!   [`gauge_metric!`], [`histogram_metric_f64!`], [`metric_attributes!`],
//!   [`metric_attributes_vec!`]) for convenient instrument creation.
//!
//! # Features
//!
//! - `opentelemetry` - Enables the `opentelemetry` core crate dependency and the macros.
//! - `opentelemetry-otlp` - Enables OTLP exporter support (implies `opentelemetry`).
//!   - `opentelemetry-otlp-gzip` - Enables gzip compression for the OTLP gRPC transport.
//!     Only effective when `opentelemetry-otlp` is also enabled.
//!   - `opentelemetry-otlp-zstd` - Enables zstd compression for the OTLP gRPC transport.
//!     Only effective when `opentelemetry-otlp` is also enabled.
//! - `opentelemetry-prometheus` - Enables Prometheus exporter support and re-exports the
//!   [`prometheus`] crate (implies `opentelemetry`).
//! - `opentelemetry-internal-logs` - Enables internal diagnostic logging from the OpenTelemetry
//!   crates. This only affects crates already activated by other features.
//! - `opentelemetry-semantic-conventions` - Re-exports the [`opentelemetry_semantic_conventions`]
//!   crate for standard OpenTelemetry attribute names.
//!
//! # Example
//!
//! ```toml
//! [dependencies]
//! metrics_utils = { version = "0.1", features = ["opentelemetry-otlp"] }
//! ```
//!
//! ```
//! # #[cfg(feature = "opentelemetry-otlp")]
//! # #[tokio::main(flavor = "current_thread")]
//! # async fn main() -> Result<(), Box<dyn std::error::Error>> {
//! use metrics_utils::{
//!     MetricsConfig, OtlpConfig, Temporality, counter_metric, global_meter, histogram_metric_f64,
//!     init_metrics, metric_attributes,
//! };
//!
//! // Configure the metrics pipeline
//! let config = MetricsConfig {
//!     service_name: String::from("my_service"),
//!     resource_attributes: vec![
//!         ("service.version".into(), "1.0.0".into()),
//!         ("deployment.environment.name".into(), "production".into()),
//!     ],
//!     # #[cfg(feature = "opentelemetry-otlp")]
//!     otlp_config: Some(OtlpConfig {
//!         endpoint: "http://localhost:4317".into(),
//!         endpoint_timeout: Some(std::time::Duration::from_secs(5)),
//!         metrics_export_interval: Some(std::time::Duration::from_secs(10)),
//!         # #[cfg(any(feature = "opentelemetry-otlp-gzip", feature = "opentelemetry-otlp-zstd"))]
//!         compression: None,
//!         temporality: Some(Temporality::Cumulative),
//!     }),
//!     # #[cfg(feature = "opentelemetry-prometheus")]
//!     enable_prometheus: false,
//! };
//!
//! // Initialize the metrics pipeline and obtain a handle to it
//! let handle = init_metrics(&config)?;
//!
//! // Register the provider as the global meter provider
//! handle.register_as_global();
//!
//! // Create a meter for this service
//! global_meter!(pub MY_METER, "my_service");
//!
//! // Create instruments
//! counter_metric!(
//!     pub REQUEST_COUNT, MY_METER,
//!     name: "http.server.requests",
//!     description: "Count of HTTP server requests processed",
//! );
//! histogram_metric_f64!(
//!     pub REQUEST_DURATION, MY_METER,
//!     name: "http.server.request.duration",
//!     description: "Duration of HTTP server requests in seconds",
//!     unit: "s",
//! );
//!
//! // In your request handler:
//!
//! // Record the measurements
//! REQUEST_COUNT.add(1, metric_attributes!(("http.route", "/health")));
//! REQUEST_DURATION.record(0.123, metric_attributes!(("http.route", "/health")));
//!
//! // Shut down the provider to flush any pending exports, when shutting down your application
//! if let Err(e) = handle.shutdown() {
//!     eprintln!("Metrics provider shutdown failed: {e}");
//! }
//!
//! # Ok(())
//! # }
//! # #[cfg(not(feature = "opentelemetry-otlp"))]
//! # fn main() {}
//! ```

#![cfg_attr(docsrs, feature(doc_cfg))]
#![doc(test(attr(deny(warnings))))]

// Public so that declarative macros can reference re-exported OpenTelemetry types across crate
// boundaries.
// Doc-hidden because consumers should import items via re-exports below.
#[cfg(feature = "opentelemetry")]
#[doc(hidden)]
pub mod opentelemetry;
mod utils;

#[cfg(feature = "opentelemetry-semantic-conventions")]
pub use opentelemetry_semantic_conventions;
#[cfg(feature = "opentelemetry-prometheus")]
pub use prometheus;

#[cfg(all(
    feature = "opentelemetry-otlp",
    any(
        feature = "opentelemetry-otlp-gzip",
        feature = "opentelemetry-otlp-zstd"
    )
))]
pub use self::opentelemetry::OtlpCompression;
#[cfg(feature = "opentelemetry")]
pub use self::opentelemetry::{MetricsConfig, MetricsError};
#[cfg(any(feature = "opentelemetry-otlp", feature = "opentelemetry-prometheus"))]
pub use self::opentelemetry::{MetricsHandle, init_metrics};
#[cfg(feature = "opentelemetry-otlp")]
pub use self::opentelemetry::{OtlpConfig, Temporality};
pub use self::utils::f64_histogram_buckets;
