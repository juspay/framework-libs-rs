# Rust Framework Libraries

This repository provides a collection of Rust libraries and utilities to aid application and service development within Juspay.
However, we aim to keep these libraries generic enough so that they remain useful for the broader Rust community as well.

## Available Crates

- [`log_utils`](crates/log_utils/): A configurable logging infrastructure built on the [`tracing`](https://github.com/tokio-rs/tracing) ecosystem.

## Roadmap

We plan to expand this collection with additional utilities commonly needed in production Rust applications.

- [ ] `log_utils`:
  - [x] Add support for the [`tracing`](https://github.com/tokio-rs/tracing) ecosystem
  - [ ] Add support for the [`fastrace`](https://github.com/fast/fastrace) ecosystem
- [ ] Metrics support:
  - [ ] Support for pushing metrics in OpenTelemetry format with the `opentelemetry` ecosystem
  - [ ] Support for exposing metrics in Prometheus format
- [ ] HTTP client utilities
  - [ ] Optionally, include metrics support
- [ ] HTTP server utilities
  - [ ] Optionally, include metrics support
  - [ ] Middleware for the `axum` server
  - [ ] Middleware for the `actix-web` server
- [ ] Database related utilities
  - [ ] Macros to provide common implementations
  - [ ] Functions / queries for commonly performed operations.
    - [ ] Optionally, include metrics support
  - [ ] Can support PostgreSQL and MySQL
- [ ] Redis related utilities:
  - [ ] An easy / simple interface to work with Redis
  - [ ] Optionally, include metrics support
- [ ] Kafka related utilities:
  - [ ] An easy / simple interface to work with Kafka
  - [ ] Optionally, include metrics support

Preferably, we'd like to keep things in smaller, individual crates rather than including everything in one single crate.
This would allow users to pull specific crates based on their requirements.

Additionally, when supporting multiple underlying libraries / ecosystems or adding optional features (such as observability), we can make use of Cargo features, rather than having everything available by default.

## Contributing

While primarily developed for internal use, we welcome contributions that improve the libraries' quality, performance, or usability.
Please ensure any contributions maintain backward compatibility (as much as possible) and include appropriate tests.

## License

All crates in this repository are licensed under the [Apache-2.0 license](LICENSE).
