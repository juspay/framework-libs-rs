# log_utils

A configurable logging utility crate for building applications.
This crate primarily provides a JSON formatting layer along with a persistence layer,
to format logs in a JSON structure, while also handling propagation of context information to parent spans.

As of now, this crate provides support for the [`tracing`](https://github.com/tokio-rs/tracing) ecosystem only with the `tracing` feature flag,
support for the [`fastrace`](https://github.com/fast/fastrace) ecosystem may be added if required.

## Features

When the `tracing` feature flag is enabled, the following features are available:

- **JSON structured logging** with compact JSON and pretty-printed (human readable) formats.
- **Span data persistence** across nested spans for context propagation.
  This can be useful to propagate values such as resource identifiers (user ID, merchant ID, etc.) to parent spans.
- **Flexible field placement**:
  - A fixed set of top-level key-value pairs may be specified.
  - A set of top-level keys to be always included at top-level may be specified.
    This could include resource identifiers, for example.
  - Any additional keys may be either nested or be logged at top-level itself.
- **File and console logging support**

## Usage and Examples

Refer to the crate documentation in the [`src/lib.rs`](./src/lib.rs) file for examples and usage information.

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

Then a sample log output could look like:

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
  "session_id": "session_789",
  "extra": {
    "endpoint": "/api/users",
    "method": "GET"
  }
}
```

## License

Licensed under [Apache-2.0](../../LICENSE).
