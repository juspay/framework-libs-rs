# metrics_utils

A thin configuration and lifecycle layer over the [OpenTelemetry Rust SDK][opentelemetry-rust-github] for application metrics.

It initializes and manages an [`SdkMeterProvider`][otel-sdk-meter-provider-docs] with optional OTLP and Prometheus exporters.
It also provides macros for declaring meters, instruments, and metric attributes without requiring direct `opentelemetry` references at each call site.

## What this crate provides

- **One-call pipeline initialization** through `init_metrics()`, which configures the [`SdkMeterProvider`][otel-sdk-meter-provider-docs], one [`MetricReader`][otel-metric-reader-docs] per configured exporter, and the corresponding OTLP or Prometheus exporters.
- **Metrics pipeline lifecycle management** through `MetricsHandle`, including global provider registration, Prometheus registry access, and shutdown.
- **Instrumentation macros** (`global_meter!`, `counter_metric!`, `histogram_metric_f64!`, `gauge_metric!`, `up_down_counter_metric!`, `metric_attributes!`, and `metric_attributes_vec!`) for defining meters, instruments, and metric attribute sets.

This crate simplifies pipeline setup without replacing the OpenTelemetry instrumentation API: instruments retain their standard types and use the usual `.add()` and `.record()` methods.

## Feature Flags

- `opentelemetry`: Enables the OpenTelemetry core crate dependency and the macros.
- `opentelemetry-otlp`: Enables the OTLP exporter (implies `opentelemetry`).
  - `opentelemetry-otlp-gzip`: Enables gzip compression for the OTLP gRPC transport when `opentelemetry-otlp` is also enabled.
  - `opentelemetry-otlp-zstd`: Enables zstd compression for the OTLP gRPC transport when `opentelemetry-otlp` is also enabled.
- `opentelemetry-prometheus`: Enables the Prometheus pull exporter and re-exports the [`prometheus`][prometheus-docs] crate (implies `opentelemetry`).
- `opentelemetry-internal-logs`: Enables diagnostic logging for OpenTelemetry crates already activated by other features.
- `opentelemetry-semantic-conventions`: Re-exports the [`opentelemetry-semantic-conventions`][opentelemetry_semantic_conventions-docs] crate for standard OpenTelemetry attribute names.

## Usage and Examples

Refer to the crate documentation in the [`src/lib.rs`][lib-rs] file for examples and usage information.

## Runtime behavior

- `init_metrics()` does not set the global meter provider.
  Call `MetricsHandle::register_as_global()` before first accessing meters or instruments declared through the global instrumentation macros.
  Otherwise, they may be initialized as no-op instruments and recorded measurements will be discarded.
- When OTLP and Prometheus are both configured, their readers share the same `SdkMeterProvider`.
- `MetricsHandle` has a single owner and does not implement `Clone`.
  Wrap it in an `Arc` when shared access is required.
- The Prometheus exporter does not serve an HTTP endpoint.
  Use `MetricsHandle::prometheus_registry()` to access the registry and expose it from the application, typically at `/metrics`.

## License

Licensed under [Apache-2.0][license].

[opentelemetry-rust-github]: https://github.com/open-telemetry/opentelemetry-rust
[otel-sdk-meter-provider-docs]: https://docs.rs/opentelemetry_sdk/latest/opentelemetry_sdk/metrics/struct.SdkMeterProvider.html
[otel-metric-reader-docs]: https://docs.rs/opentelemetry_sdk/latest/opentelemetry_sdk/metrics/reader/trait.MetricReader.html
[prometheus-docs]: https://docs.rs/prometheus/latest/prometheus/
[opentelemetry_semantic_conventions-docs]: https://docs.rs/opentelemetry-semantic-conventions/latest/opentelemetry_semantic_conventions/
[lib-rs]: src/lib.rs
[license]: ../../LICENSE
