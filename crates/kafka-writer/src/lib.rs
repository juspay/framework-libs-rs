//! Kafka writer module for sending logs to Kafka
//!
//! This module provides a Kafka writer that integrates with tracing-subscriber
//! to send logs to Kafka topics.
//!
//! # Examples
//!
//! ## Using KafkaWriter directly
//! ```no_run
//! use kafka_writer::KafkaWriter;
//! use rdkafka::message::OwnedHeaders;
//!
//! let writer = KafkaWriter::new(
//!     vec!["localhost:9092".to_string()],
//!     "default-topic".to_string(),
//!     None,
//!     None,
//!     None,
//!     None,
//!     None,
//!     None,
//! )
//! .expect("Failed to create KafkaWriter");
//!
//! let headers = OwnedHeaders::new().insert(rdkafka::message::Header {
//!     key: "my-header",
//!     value: Some("my-value"),
//! });
//!
//! let result = writer.publish_event(
//!     "custom-events",
//!     Some("event-key"),
//!     b"event-payload",
//!     Some(headers),
//! );
//!
//! if let Err(e) = result {
//!     eprintln!("Failed to publish event: {}", e);
//! }
//! ```
//!
//! ## Using KafkaLayer with tracing (requires "layer" feature)
//! ```no_run
//! # #[cfg(feature = "layer")]
//! # {
//! use kafka_writer::KafkaLayer;
//! use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};
//!
//! let kafka_layer = KafkaLayer::builder()
//!     .brokers(&["localhost:9092"])
//!     .topic("application-logs")
//!     .build()
//!     .expect("Failed to create Kafka layer");
//!
//! tracing_subscriber::registry().with(kafka_layer).init();
//! # }
//! ```

pub mod builder;
mod writer;

#[cfg(feature = "layer")]
mod layer;

pub use builder::KafkaWriterBuilder;
#[cfg(feature = "layer")]
pub use layer::{KafkaLayer, KafkaLayerBuilder, KafkaLayerError};
pub use writer::{KafkaWriter, KafkaWriterError};

#[cfg(feature = "kafka-metrics")]
mod metrics;

/// Initializes the metrics for the kafka writer.
/// This function should be called once at application startup.
#[cfg(feature = "kafka-metrics")]
pub fn init() {
    metrics::initialize_all_metrics();
}

#[cfg(not(feature = "kafka-metrics"))]
pub fn init() {
    tracing::warn!("Kafka metrics feature is not enabled. Metrics will not be collected.");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_kafka_writer_builder_accessible() {
        let _builder = KafkaWriterBuilder::new();
        // Test passes if no panic occurs
    }

    #[test]
    fn test_init_function_exists() {
        init();
        // Test passes if no panic occurs
    }

    #[cfg(feature = "kafka-metrics")]
    #[test]
    fn test_init_with_metrics() {
        init();
        // Test passes if no panic occurs
    }

    #[test]
    fn test_public_api_exports() {
        let _builder: KafkaWriterBuilder = KafkaWriterBuilder::new();

        let _error = KafkaWriterError::ProducerCreation(
            rdkafka::error::KafkaError::ClientCreation("test".to_string()),
        );

        // Test passes if no panic occurs
    }
}
