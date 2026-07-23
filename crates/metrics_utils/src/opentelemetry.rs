//! OpenTelemetry-backed metrics infrastructure.
//!
//! This module is only available when the `opentelemetry` feature is enabled.

mod macros;

// Re-export the OpenTelemetry types referenced by the macros in the `macros` module.
#[doc(hidden)]
pub use ::opentelemetry::{
    KeyValue, Value, global,
    metrics::{Counter, Gauge, Histogram, Meter, UpDownCounter},
};

/// Configuration for the metrics system.
#[derive(Debug, Clone)]
pub struct MetricsConfig {
    /// The service name to report as the `service.name` [`Resource`][opentelemetry_sdk::Resource]
    /// attribute.
    pub service_name: String,

    /// Additional [`Resource`][opentelemetry_sdk::Resource] attributes,
    /// provided as `(key, value)` pairs.
    ///
    /// These are merged with the environment-detected resource attributes and the
    /// telemetry SDK-provided attributes.
    /// Keys should follow the
    /// [OpenTelemetry semantic conventions](https://github.com/open-telemetry/semantic-conventions/blob/main/docs/resource/README.md)
    /// (e.g. `service.version`, `deployment.environment.name`).
    pub resource_attributes: Vec<(String, String)>,

    /// Configuration for the OTLP push metrics exporter.
    #[cfg(feature = "opentelemetry-otlp")]
    pub otlp_config: Option<OtlpConfig>,

    /// Whether to enable the Prometheus pull exporter.
    #[cfg(feature = "opentelemetry-prometheus")]
    pub enable_prometheus: bool,
}

/// Configuration for the OTLP push metrics exporter.
#[cfg(feature = "opentelemetry-otlp")]
#[derive(Debug, Clone)]
pub struct OtlpConfig {
    /// The OTLP collector endpoint (e.g. `http://localhost:4317`).
    pub endpoint: String,

    /// Timeout applied to each OTLP metrics export.
    ///
    /// If not specified, the [`MetricExporter`][opentelemetry_otlp::MetricExporter]'s
    /// default timeout is used.
    pub endpoint_timeout: Option<std::time::Duration>,

    /// Time between consecutive metrics exports.
    ///
    /// If not specified, the [`PeriodicReader`][opentelemetry_sdk::metrics::PeriodicReader]'s
    /// default interval is used.
    pub metrics_export_interval: Option<std::time::Duration>,

    /// Compression algorithm for gRPC requests.
    #[cfg(any(
        feature = "opentelemetry-otlp-gzip",
        feature = "opentelemetry-otlp-zstd"
    ))]
    pub compression: Option<OtlpCompression>,
}

/// Compression algorithm for the OTLP gRPC exporter.
/// Each variant requires the corresponding feature flag.
#[cfg(all(
    feature = "opentelemetry-otlp",
    any(
        feature = "opentelemetry-otlp-gzip",
        feature = "opentelemetry-otlp-zstd"
    )
))]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum OtlpCompression {
    /// Compress data using gzip.
    #[cfg(feature = "opentelemetry-otlp-gzip")]
    Gzip,

    /// Compress data using zstd.
    #[cfg(feature = "opentelemetry-otlp-zstd")]
    Zstd,
}

#[cfg(all(
    feature = "opentelemetry-otlp",
    any(
        feature = "opentelemetry-otlp-gzip",
        feature = "opentelemetry-otlp-zstd"
    )
))]
impl OtlpCompression {
    /// Converts to the corresponding [`opentelemetry_otlp::Compression`] variant.
    fn as_otel(self) -> opentelemetry_otlp::Compression {
        match self {
            #[cfg(feature = "opentelemetry-otlp-gzip")]
            Self::Gzip => opentelemetry_otlp::Compression::Gzip,

            #[cfg(feature = "opentelemetry-otlp-zstd")]
            Self::Zstd => opentelemetry_otlp::Compression::Zstd,
        }
    }
}

impl MetricsConfig {
    /// Validates the configuration, returning an error if any values are invalid.
    ///
    /// # Errors
    ///
    /// Returns [`MetricsError::InvalidConfig`] if any configuration value is invalid.
    pub fn validate(&self) -> Result<(), MetricsError> {
        if self.service_name.is_empty() {
            return Err(MetricsError::InvalidConfig(
                "service name must not be empty",
            ));
        }

        #[cfg(feature = "opentelemetry-otlp")]
        if let Some(otlp) = &self.otlp_config {
            if otlp.endpoint.is_empty() {
                return Err(MetricsError::InvalidConfig(
                    "OTLP endpoint must not be empty",
                ));
            }

            if let Some(timeout) = otlp.endpoint_timeout
                && timeout.is_zero()
            {
                return Err(MetricsError::InvalidConfig(
                    "OTLP endpoint timeout must be non-zero",
                ));
            }

            if let Some(interval) = otlp.metrics_export_interval
                && interval.is_zero()
            {
                return Err(MetricsError::InvalidConfig(
                    "OTLP export interval must be non-zero",
                ));
            }
        }

        Ok(())
    }
}

/// Initializes the OpenTelemetry metrics pipeline based on the provided [`MetricsConfig`].
///
/// Builds an [`SdkMeterProvider`](opentelemetry_sdk::metrics::SdkMeterProvider) with a single
/// [`Resource`](opentelemetry_sdk::Resource) and one
/// [`MetricReader`](opentelemetry_sdk::metrics::reader::MetricReader) per configured exporter.
/// When both OTLP and Prometheus exports are enabled, both readers are attached to the same
/// provider.
/// Returns a [`MetricsHandle`] that can be used to register the provider globally,
/// access the Prometheus registry, and shut the provider down.
///
/// This function does NOT set the global meter provider.
/// Per the OpenTelemetry docs, setting the global meter provider is the application's
/// responsibility.
/// Use [`MetricsHandle::register_as_global`] as a convenience method for that, or call
/// [`opentelemetry::global::set_meter_provider`] directly.
///
/// # Errors
///
/// This function returns:
///
/// - [`MetricsError::InvalidConfig`] if the configuration is invalid.
/// - [`MetricsError::NoExportersConfigured`] if no readers were attached (OTLP config is `None`
///   and `enable_prometheus` is `false`).
/// - Exporter-specific errors if an exporter cannot be built.
///
/// # Example
///
/// ```
/// use std::time::Duration;
///
/// use metrics_utils::{MetricsConfig, OtlpConfig, init_metrics};
///
/// # #[tokio::main(flavor = "current_thread")]
/// # async fn main() -> Result<(), Box<dyn std::error::Error>> {
/// // Configure metrics
/// let config = MetricsConfig {
///     service_name: String::from("my_service"),
///     resource_attributes: vec![
///         ("service.version".into(), "1.0.0".into()),
///         ("deployment.environment.name".into(), "production".into()),
///     ],
///     # #[cfg(feature = "opentelemetry-otlp")]
///     otlp_config: Some(OtlpConfig {
///         endpoint: "http://localhost:4317".into(),
///         endpoint_timeout: Some(Duration::from_secs(5)),
///         metrics_export_interval: Some(Duration::from_secs(10)),
///         # #[cfg(any(feature = "opentelemetry-otlp-gzip", feature = "opentelemetry-otlp-zstd"))]
///         compression: None,
///     }),
///     # #[cfg(feature = "opentelemetry-prometheus")]
///     enable_prometheus: true,
/// };
///
/// // Initialize metrics and register global meter provider
/// let handle = init_metrics(&config)?;
/// handle.register_as_global();
///
/// # #[cfg(feature = "opentelemetry-prometheus")]
/// if let Some(_registry) = handle.prometheus_registry() {
///     // Serve registry at `/metrics` endpoint.
/// }
///
/// // Shutdown provider when shutting down your application
/// handle.shutdown()?;
/// # Ok(())
/// # }
/// ```
#[cfg(any(feature = "opentelemetry-otlp", feature = "opentelemetry-prometheus"))]
pub fn init_metrics(config: &MetricsConfig) -> Result<MetricsHandle, MetricsError> {
    config.validate()?;

    let resource = build_resource(config);
    let mut provider_builder =
        opentelemetry_sdk::metrics::SdkMeterProvider::builder().with_resource(resource);

    let mut has_reader = false;

    #[cfg(feature = "opentelemetry-otlp")]
    if let Some(otlp_config) = &config.otlp_config {
        provider_builder = provider_builder.with_reader(build_otlp_reader(otlp_config)?);
        has_reader = true;
    }

    #[cfg(feature = "opentelemetry-prometheus")]
    let prometheus_registry = if config.enable_prometheus {
        let (registry, exporter) = build_prometheus_exporter()?;
        provider_builder = provider_builder.with_reader(exporter);
        has_reader = true;
        Some(registry)
    } else {
        None
    };

    if !has_reader {
        return Err(MetricsError::NoExportersConfigured);
    }

    let provider = provider_builder.build();

    Ok(MetricsHandle {
        provider,
        #[cfg(feature = "opentelemetry-prometheus")]
        prometheus_registry,
    })
}

/// Build the [`Resource`][opentelemetry_sdk::Resource] from config, layering `service.name` and
/// extra attributes on top of the environment-detected resource attributes and
/// telemetry SDK-provided attributes.
#[cfg(any(feature = "opentelemetry-otlp", feature = "opentelemetry-prometheus"))]
fn build_resource(config: &MetricsConfig) -> opentelemetry_sdk::Resource {
    opentelemetry_sdk::Resource::builder()
        .with_service_name(config.service_name.clone())
        .with_attributes(
            config
                .resource_attributes
                .iter()
                .map(|(key, value)| KeyValue::new(key.clone(), value.clone())),
        )
        .build()
}

/// Build a [`PeriodicReader`][opentelemetry_sdk::metrics::PeriodicReader] wrapping a
/// [`MetricExporter`][opentelemetry_otlp::MetricExporter].
#[cfg(feature = "opentelemetry-otlp")]
fn build_otlp_reader(
    config: &OtlpConfig,
) -> Result<
    opentelemetry_sdk::metrics::PeriodicReader<opentelemetry_otlp::MetricExporter>,
    MetricsError,
> {
    use opentelemetry_otlp::WithExportConfig;
    #[cfg(any(
        feature = "opentelemetry-otlp-gzip",
        feature = "opentelemetry-otlp-zstd"
    ))]
    use opentelemetry_otlp::WithTonicConfig;

    let mut exporter_builder = opentelemetry_otlp::MetricExporter::builder()
        .with_tonic()
        .with_endpoint(&config.endpoint);

    #[cfg(any(
        feature = "opentelemetry-otlp-gzip",
        feature = "opentelemetry-otlp-zstd"
    ))]
    if let Some(compression) = config.compression {
        exporter_builder = exporter_builder.with_compression(compression.as_otel());
    }

    if let Some(timeout) = config.endpoint_timeout {
        exporter_builder = exporter_builder.with_timeout(timeout);
    }

    let exporter = exporter_builder
        .build()
        .map_err(|error| MetricsError::OtlpExporterBuild {
            source: Box::new(error),
        })?;

    let mut reader_builder = opentelemetry_sdk::metrics::PeriodicReader::builder(exporter);

    if let Some(interval) = config.metrics_export_interval {
        reader_builder = reader_builder.with_interval(interval);
    }

    Ok(reader_builder.build())
}

/// Build a Prometheus exporter and return the [`Registry`][prometheus::Registry] and
/// [`PrometheusExporter`][opentelemetry_prometheus::PrometheusExporter].
#[cfg(feature = "opentelemetry-prometheus")]
fn build_prometheus_exporter() -> Result<
    (
        prometheus::Registry,
        opentelemetry_prometheus::PrometheusExporter,
    ),
    MetricsError,
> {
    let registry = prometheus::Registry::new();
    let exporter = opentelemetry_prometheus::exporter()
        .with_registry(registry.clone())
        .build()
        .map_err(|error| MetricsError::PrometheusExporterBuild {
            source: Box::new(error),
        })?;
    Ok((registry, exporter))
}

/// A handle to the initialized metrics pipeline.
///
/// Can be used to register the provider as the global meter provider, access the Prometheus
/// registry (when configured), and shut the provider down.
///
/// # Global provider registration
///
/// The built provider is NOT automatically set as the global meter provider by [`init_metrics`].
/// Call [`register_as_global`][Self::register_as_global] to do so, or call
/// [`opentelemetry::global::set_meter_provider`] directly.
/// Until the global provider is set, meters and instruments created via the
/// [`global_meter!`][crate::global_meter!] macro and other instrumentation macros are no-ops,
/// and recorded measurements are discarded.
///
/// # Shutdown
///
/// The provider is shut down when [`shutdown`][Self::shutdown] is called or when the handle is
/// dropped.
/// Calling [`shutdown`][Self::shutdown] after the provider has already been shut down returns
/// an error.
/// Dropping the handle after an explicit shutdown is a no-op.
#[cfg(any(feature = "opentelemetry-otlp", feature = "opentelemetry-prometheus"))]
#[derive(Debug)]
pub struct MetricsHandle {
    /// The SDK meter provider managing the pipeline.
    provider: opentelemetry_sdk::metrics::SdkMeterProvider,

    /// The Prometheus registry.
    #[cfg(feature = "opentelemetry-prometheus")]
    prometheus_registry: Option<prometheus::Registry>,
}

#[cfg(any(feature = "opentelemetry-otlp", feature = "opentelemetry-prometheus"))]
impl MetricsHandle {
    /// Registers the underlying meter provider as the global meter provider.
    ///
    /// This is a convenience wrapper around [`opentelemetry::global::set_meter_provider`].
    /// Until this is called (or the application sets the global provider itself), meters and
    /// instruments created via the [`global_meter!`](crate::global_meter!) macro and other
    /// instrumentation macros are no-ops, and recorded measurements are discarded.
    pub fn register_as_global(&self) {
        global::set_meter_provider(self.provider.clone());
    }

    /// Returns a reference to the Prometheus registry, if available.
    ///
    /// The consumer is responsible for serving the registry over HTTP (e.g. at `/metrics`).
    #[cfg(feature = "opentelemetry-prometheus")]
    pub fn prometheus_registry(&self) -> Option<&prometheus::Registry> {
        self.prometheus_registry.as_ref()
    }

    /// Shuts down the meter provider, flushing any buffered metrics and releasing resources.
    ///
    /// Calling [`shutdown`][Self::shutdown] after the provider has already been shut down returns
    /// an error.
    /// Dropping the handle after an explicit shutdown is a no-op.
    ///
    /// # Errors
    ///
    /// Returns [`MetricsError::Shutdown`] if the provider could not be shut down (e.g. a flush or
    /// release step failed), or if shutdown was already invoked.
    pub fn shutdown(&self) -> Result<(), MetricsError> {
        self.provider
            .shutdown()
            .map_err(|error| MetricsError::Shutdown {
                source: Box::new(error),
            })
    }
}

#[cfg(any(feature = "opentelemetry-otlp", feature = "opentelemetry-prometheus"))]
impl Drop for MetricsHandle {
    fn drop(&mut self) {
        if let Err(error) = self.provider.shutdown() {
            tracing::error!("failed to shutdown metric provider during drop: {error}");
        }
    }
}

/// Errors that can occur during metrics initialization and shutdown.
#[cfg_attr(
    not(any(feature = "opentelemetry-otlp", feature = "opentelemetry-prometheus")),
    expect(
        missing_copy_implementations,
        reason = "No need for this error type to implement `Copy`"
    )
)]
#[derive(Debug, thiserror::Error)]
pub enum MetricsError {
    /// The configuration is invalid.
    #[error("invalid metrics configuration: {0}")]
    InvalidConfig(&'static str),

    /// No exporters were enabled or configured.
    #[error("no exporters configured: enable at least one of OTLP or Prometheus exporter")]
    NoExportersConfigured,

    /// The OTLP exporter could not be built.
    #[cfg(feature = "opentelemetry-otlp")]
    #[error("failed to build OTLP metric exporter")]
    OtlpExporterBuild {
        /// The underlying error.
        #[source]
        source: Box<dyn std::error::Error + Send + Sync>,
    },

    /// The Prometheus exporter could not be built.
    #[cfg(feature = "opentelemetry-prometheus")]
    #[error("failed to build Prometheus metric exporter")]
    PrometheusExporterBuild {
        /// The underlying error.
        #[source]
        source: Box<dyn std::error::Error + Send + Sync>,
    },

    /// The meter provider could not be shut down. This is returned by [`MetricsHandle::shutdown`].
    #[cfg(any(feature = "opentelemetry-otlp", feature = "opentelemetry-prometheus"))]
    #[error("failed to shutdown meter provider")]
    Shutdown {
        /// The underlying error.
        #[source]
        source: Box<dyn std::error::Error + Send + Sync>,
    },
}

#[cfg(all(
    test,
    feature = "opentelemetry-otlp",
    feature = "opentelemetry-prometheus"
))]
mod tests {
    use std::time::Duration;

    use super::{MetricsConfig, MetricsError, OtlpConfig, init_metrics};

    #[test]
    fn test_config_validation() {
        // Enable OTLP exporter only
        let config = MetricsConfig {
            service_name: String::from("test_service"),
            resource_attributes: Vec::new(),
            otlp_config: Some(OtlpConfig {
                endpoint: "http://localhost:4317".into(),
                endpoint_timeout: None,
                metrics_export_interval: None,
                #[cfg(any(
                    feature = "opentelemetry-otlp-gzip",
                    feature = "opentelemetry-otlp-zstd"
                ))]
                compression: None,
            }),
            enable_prometheus: false,
        };
        assert!(config.validate().is_ok());

        // Enable both OTLP and Prometheus exporters
        let config = MetricsConfig {
            service_name: String::from("test_service"),
            resource_attributes: vec![("env".into(), "test".into())],
            otlp_config: Some(OtlpConfig {
                endpoint: "http://localhost:4317".into(),
                endpoint_timeout: Some(Duration::from_secs(5)),
                metrics_export_interval: Some(Duration::from_secs(10)),
                #[cfg(any(
                    feature = "opentelemetry-otlp-gzip",
                    feature = "opentelemetry-otlp-zstd"
                ))]
                compression: None,
            }),
            enable_prometheus: true,
        };
        assert!(config.validate().is_ok());

        // Empty service name
        let config = MetricsConfig {
            service_name: String::new(),
            resource_attributes: Vec::new(),
            otlp_config: None,
            enable_prometheus: false,
        };
        assert!(matches!(
            config.validate(),
            Err(MetricsError::InvalidConfig(_))
        ));

        // Empty OTLP endpoint
        let config = MetricsConfig {
            service_name: String::from("test"),
            resource_attributes: Vec::new(),
            otlp_config: Some(OtlpConfig {
                endpoint: String::new(),
                endpoint_timeout: None,
                metrics_export_interval: None,
                #[cfg(any(
                    feature = "opentelemetry-otlp-gzip",
                    feature = "opentelemetry-otlp-zstd"
                ))]
                compression: None,
            }),
            enable_prometheus: false,
        };
        assert!(matches!(
            config.validate(),
            Err(MetricsError::InvalidConfig(_))
        ));

        // Zero OTLP timeout
        let config = MetricsConfig {
            service_name: String::from("test"),
            resource_attributes: Vec::new(),
            otlp_config: Some(OtlpConfig {
                endpoint: "http://localhost:4317".into(),
                endpoint_timeout: Some(Duration::ZERO),
                metrics_export_interval: None,
                #[cfg(any(
                    feature = "opentelemetry-otlp-gzip",
                    feature = "opentelemetry-otlp-zstd"
                ))]
                compression: None,
            }),
            enable_prometheus: false,
        };
        assert!(matches!(
            config.validate(),
            Err(MetricsError::InvalidConfig(_))
        ));

        // Zero OTLP export interval
        let config = MetricsConfig {
            service_name: String::from("test"),
            resource_attributes: Vec::new(),
            otlp_config: Some(OtlpConfig {
                endpoint: "http://localhost:4317".into(),
                endpoint_timeout: None,
                metrics_export_interval: Some(Duration::ZERO),
                #[cfg(any(
                    feature = "opentelemetry-otlp-gzip",
                    feature = "opentelemetry-otlp-zstd"
                ))]
                compression: None,
            }),
            enable_prometheus: false,
        };
        assert!(matches!(
            config.validate(),
            Err(MetricsError::InvalidConfig(_))
        ));
    }

    #[test]
    fn init_with_no_exporters_returns_error() {
        let config = MetricsConfig {
            service_name: String::from("test_service"),
            resource_attributes: Vec::new(),
            otlp_config: None,
            enable_prometheus: false,
        };

        let error = init_metrics(&config).unwrap_err();
        assert!(matches!(error, MetricsError::NoExportersConfigured));
    }
}
