//! Kafka writer module for sending logs to Kafka
//!
//! This module provides a Kafka writer that integrates with tracing-subscriber
//! to send logs to Kafka topics.

mod builder;
mod writer;

#[cfg(feature = "kafka-metrics")]
mod metrics;

pub use builder::KafkaWriterBuilder;
pub use writer::{KafkaWriter, KafkaWriterError};

#[cfg(feature = "kafka-metrics")]
pub use metrics::*;
