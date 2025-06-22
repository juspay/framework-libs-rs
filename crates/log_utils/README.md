# log_utils

A configurable logging utility crate for building applications.
This crate primarily provides a JSON formatting layer along with a persistence layer,
to format logs in a JSON structure, while also handling propagation of context information to parent spans.

As of now, this crate provides support for the [`tracing`][tracing-github] ecosystem only with the `tracing` feature flag,
support for the [`fastrace`][fastrace-github] ecosystem may be added if required.

## Features

When the `tracing` feature flag is enabled, the following features are available:

- **JSON structured logging** with compact JSON and pretty-printed (human readable) formats.
- **Span data persistence** across nested spans for context propagation.
  All fields from parent spans are automatically propagated to child spans, while a specified set of fields can be propagated from child spans to parent spans as well.
- **Flexible field placement**:
  - A fixed set of top-level key-value pairs may be specified.
  - A set of top-level keys to be always included at top-level may be specified.
    This could include resource identifiers, for example.
  - Any additional keys may be either nested or be logged at top-level itself.
- **File and console logging support**

## Comparison with Similar Crates

### [tracing-subscriber][tracing-subscriber]

While [`tracing-subscriber`'s JSON formatter][tracing-subscriber-json-formatter] provides basic JSON formatting, it lacks the following capabilities:

- Context propagation from parent spans to child spans or vice versa.
- Customization of field placement in the log output: it does not allow specifying which fields should be logged at the top-level and which should be nested.
- It does not allow specifying a fixed set of top-level fields that should always be included in the log output.
  This can be useful for including static metadata like service name, version, etc.
- Although this may be a minor concern, logs produced by the JSON formatter do not include the file name and line number information, which can be useful for debugging.

### [tracing-bunyan-formatter][tracing-bunyan-formatter]

The implementation of this crate's JSON formatter is quite similar to the [`tracing-bunyan-formatter`][tracing-bunyan-formatter], but it differs in some aspects.

#### Similarities

- Both implementations make use of a storage layer and a formatting layer.
- Both support context propagation from parent spans to child spans.
- Both have a fixed set of fields that are implicitly included in every log entry, such as `hostname`, `pid`, `level`, `time`, `message` / `msg`, `target`, `line`, and `file`.
- Both support specifying a fixed set of top-level fields that should always be included in the log output.

#### Differences

- This crate does not conform to the [Bunyan][bunyan] log format, so some fields may be named differently or be formatted differently.
  For example, the message field is named `msg` in the Bunyan format, and the `level` and `time` fields are formatted differently.
- Customization of field placement in the log output: this crate allows specifying a set of fields to be logged at the top-level and the rest to be nested under a specific key.
  `tracing-bunyan-formatter` only supports a flat structure where all fields are logged at the top-level.
- This crate allows propagating some fields to parent spans, which is not supported by `tracing-bunyan-formatter`.
  This can be useful for propagating resource identifiers (user ID, merchant ID, etc.) for example.

## Usage and Examples

Refer to the crate documentation in the [`src/lib.rs`][lib-rs] file for examples and usage information.

### Sample Output

Let's say the following configuration is provided:

```rust
LoggerConfig {
    static_top_level_fields: HashMap::from([
        ("service".to_string(), serde_json::json!("my_app")),
        ("version".to_string(), serde_json::json!("1.0.0")),
    ]),

    // Log at top-level if present in span
    top_level_keys: HashSet::from(["user_id", "request_id"]),

    // Propagate to parent spans
    persistent_keys: HashSet::from(["session_id"]),

    additional_fields_placement: AdditionalFieldsPlacement::Nested("extra".to_string()),

    // Other configuration options...
}
```

Then a sample log output could look like (without the formatting):

```json
{
  "message": "Processing user request",
  "hostname": "my-server",
  "pid": 12345,
  "level": "INFO",
  "target": "my_app::handlers",
  "line": 42,
  "file": "src/handlers.rs",
  "fn": "handle_request",
  "full_name": "my_app::handlers::handle_request",
  "time": "2025-06-11T01:04:44.123456Z",
  "service": "my_app",
  "version": "1.0.0",
  "user_id": "user_123",
  "request_id": "request_456",
  "extra": {
    "endpoint": "/api/users",
    "method": "GET",
    "session_id": "session_789"
  }
}
```

## License

Licensed under [Apache-2.0][license].

[tracing-github]: https://github.com/tokio-rs/tracing
[fastrace-github]: https://github.com/fast/fastrace
[tracing-subscriber]: https://crates.io/crates/tracing-subscriber
[tracing-subscriber-json-formatter]: https://docs.rs/tracing-subscriber/latest/tracing_subscriber/fmt/format/struct.Json.html
[tracing-bunyan-formatter]: https://crates.io/crates/tracing-bunyan-formatter
[bunyan]: https://github.com/trentm/node-bunyan
[lib-rs]: src/lib.rs
[license]: ../../LICENSE
