# kafka-writer

A generic Kafka writer for `tracing-subscriber` that allows you to send structured logs to Kafka.

## Features

- Seamless integration with `tracing-subscriber`
- Non-blocking writes
- Optional Prometheus metrics for monitoring
- Builder pattern for easy configuration
- Graceful shutdown with message flushing

## Usage

### Basic Usage

```rust
use kafka_writer::KafkaWriter;
use tracing_subscriber::prelude::*;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Create a Kafka writer
    let kafka_writer = KafkaWriter::builder()
        .brokers(vec!["localhost:9092".to_string()])
        .topic("application-logs")
        .build()?;

    // Use it with tracing-subscriber
    tracing_subscriber::fmt()
        .with_writer(kafka_writer)
        .json()
        .init();

    // Your application code here
    tracing::info!("Application started");
    
    Ok(())
}
```

### With Configuration

```rust
use kafka_writer::KafkaWriter;
use std::time::Duration;

let kafka_writer = KafkaWriter::builder()
    .brokers(vec!["broker1:9092".to_string(), "broker2:9092".to_string()])
    .topic("app-logs")
    .batch_size(1000)
    .linger(Duration::from_millis(100))
    .queue_buffering_max_messages(200_000)
    .reconnect_backoff(
        Duration::from_millis(100),
        Duration::from_secs(10)
    )
    .build()?;
```


### With Metrics

Enable the `metrics` feature in your `Cargo.toml`:

```toml
[dependencies]
kafka-writer = { version = "0.1", features = ["metrics"] }
```

This will expose the following Prometheus metrics:
- `kafka_logs_sent_total` - Total number of logs successfully sent
- `kafka_logs_dropped_total` - Total number of logs dropped
- `kafka_producer_queue_size` - Current size of the producer queue
- `kafka_drops_queue_full_total` - Logs dropped due to queue full
- `kafka_drops_msg_too_large_total` - Logs dropped due to message size
- `kafka_drops_timeout_total` - Logs dropped due to timeout
- `kafka_drops_other_total` - Logs dropped due to other errors

## Configuration Options

- **brokers**: List of Kafka broker addresses (required)
- **topic**: Kafka topic to send logs to (required)
- **batch_size**: Number of messages to batch before sending
- **linger_ms**: Time to wait before sending a batch (Default: 0ms)
- **queue_buffering_max_messages**: Maximum number of messages in the producer queue
- **queue_buffering_max_kbytes**: Maximum size of the producer queue in KB
- **reconnect_backoff**: Min and max backoff times for reconnection

## Error Handling

The writer is designed to be non-blocking. If Kafka is unavailable or the queue is full, logs will be dropped rather than blocking your application. This ensures your application remains responsive even if the logging infrastructure has issues.

## License

This project is licensed under the same terms as the parent workspace.
