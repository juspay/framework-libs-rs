//! Builder pattern implementation for KafkaWriter

use std::time::Duration;

use super::writer::{KafkaWriter, KafkaWriterError};

/// Builder for creating a KafkaWriter with custom configuration
#[derive(Debug, Clone, Default)]
pub struct KafkaWriterBuilder {
    brokers: Option<Vec<String>>,
    topic: Option<String>,
    batch_size: Option<usize>,
    linger_ms: Option<u64>,
    queue_buffering_max_messages: Option<usize>,
    queue_buffering_max_kbytes: Option<usize>,
    reconnect_backoff_min_ms: Option<u64>,
    reconnect_backoff_max_ms: Option<u64>,
}

impl KafkaWriterBuilder {
    /// Creates a new builder with default settings
    pub fn new() -> Self {
        Self::default()
    }

    /// Sets the Kafka brokers to connect to
    pub fn brokers(mut self, brokers: Vec<String>) -> Self {
        self.brokers = Some(brokers);
        self
    }

    /// Sets the Kafka topic to send logs to
    pub fn topic(mut self, topic: impl Into<String>) -> Self {
        self.topic = Some(topic.into());
        self
    }

    /// Sets the batch size for buffering messages before sending
    pub fn batch_size(mut self, size: usize) -> Self {
        self.batch_size = Some(size);
        self
    }

    /// Sets the linger time in milliseconds
    pub fn linger_ms(mut self, ms: u64) -> Self {
        self.linger_ms = Some(ms);
        self
    }

    /// Sets the linger time as a Duration
    pub fn linger(mut self, duration: Duration) -> Self {
        self.linger_ms = Some(duration.as_millis() as u64);
        self
    }

    /// Sets the maximum number of messages to buffer in the producer's queue
    pub fn queue_buffering_max_messages(mut self, size: usize) -> Self {
        self.queue_buffering_max_messages = Some(size);
        self
    }

    /// Sets the maximum size of the producer's queue in kilobytes
    pub fn queue_buffering_max_kbytes(mut self, size: usize) -> Self {
        self.queue_buffering_max_kbytes = Some(size);
        self
    }

    /// Sets the reconnect backoff times
    pub fn reconnect_backoff(mut self, min: Duration, max: Duration) -> Self {
        self.reconnect_backoff_min_ms = Some(min.as_millis() as u64);
        self.reconnect_backoff_max_ms = Some(max.as_millis() as u64);
        self
    }

    /// Builds the KafkaWriter with the configured settings
    pub fn build(self) -> Result<KafkaWriter, KafkaWriterError> {
        let brokers = self.brokers.ok_or_else(|| {
            KafkaWriterError::ProducerCreation(rdkafka::error::KafkaError::ClientCreation(
                "No brokers specified. Use .brokers()".to_string(),
            ))
        })?;

        let topic = self.topic.ok_or_else(|| {
            KafkaWriterError::ProducerCreation(rdkafka::error::KafkaError::ClientCreation(
                "No topic specified. Use .topic()".to_string(),
            ))
        })?;

        KafkaWriter::new(
            brokers,
            topic,
            self.batch_size,
            self.linger_ms,
            self.queue_buffering_max_messages,
            self.queue_buffering_max_kbytes,
            self.reconnect_backoff_min_ms,
            self.reconnect_backoff_max_ms,
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_builder_new() {
        let builder = KafkaWriterBuilder::new();
        assert!(builder.brokers.is_none());
        assert!(builder.topic.is_none());
        assert!(builder.batch_size.is_none());
        assert!(builder.linger_ms.is_none());
    }

    #[test]
    fn test_builder_default() {
        let builder = KafkaWriterBuilder::default();
        assert!(builder.brokers.is_none());
        assert!(builder.topic.is_none());
        assert!(builder.batch_size.is_none());
        assert!(builder.linger_ms.is_none());
    }

    #[test]
    fn test_builder_brokers() {
        let brokers = vec!["localhost:9092".to_string(), "localhost:9093".to_string()];
        let builder = KafkaWriterBuilder::new().brokers(brokers.clone());
        assert_eq!(builder.brokers, Some(brokers));
    }

    #[test]
    fn test_builder_topic() {
        let topic = "test-topic";
        let builder = KafkaWriterBuilder::new().topic(topic);
        assert_eq!(builder.topic, Some(topic.to_string()));
    }

    #[test]
    fn test_builder_batch_size() {
        let batch_size = 5000;
        let builder = KafkaWriterBuilder::new().batch_size(batch_size);
        assert_eq!(builder.batch_size, Some(batch_size));
    }

    #[test]
    fn test_builder_linger_ms() {
        let linger_ms = 500;
        let builder = KafkaWriterBuilder::new().linger_ms(linger_ms);
        assert_eq!(builder.linger_ms, Some(linger_ms));
    }

    #[test]
    fn test_builder_linger_duration() {
        let duration = Duration::from_millis(250);
        let builder = KafkaWriterBuilder::new().linger(duration);
        assert_eq!(builder.linger_ms, Some(250));
    }

    #[test]
    fn test_builder_queue_buffering_max_messages() {
        let max_messages = 20000;
        let builder = KafkaWriterBuilder::new().queue_buffering_max_messages(max_messages);
        assert_eq!(builder.queue_buffering_max_messages, Some(max_messages));
    }

    #[test]
    fn test_builder_queue_buffering_max_kbytes() {
        let max_kbytes = 2048;
        let builder = KafkaWriterBuilder::new().queue_buffering_max_kbytes(max_kbytes);
        assert_eq!(builder.queue_buffering_max_kbytes, Some(max_kbytes));
    }

    #[test]
    fn test_builder_reconnect_backoff() {
        let min = Duration::from_millis(50);
        let max = Duration::from_millis(10000);
        let builder = KafkaWriterBuilder::new().reconnect_backoff(min, max);
        assert_eq!(builder.reconnect_backoff_min_ms, Some(50));
        assert_eq!(builder.reconnect_backoff_max_ms, Some(10000));
    }

    #[test]
    fn test_builder_chaining() {
        let builder = KafkaWriterBuilder::new()
            .brokers(vec!["localhost:9092".to_string()])
            .topic("test-topic")
            .batch_size(1000)
            .linger_ms(100)
            .queue_buffering_max_messages(10000)
            .queue_buffering_max_kbytes(1024)
            .reconnect_backoff(Duration::from_millis(100), Duration::from_millis(30000));

        assert_eq!(builder.brokers, Some(vec!["localhost:9092".to_string()]));
        assert_eq!(builder.topic, Some("test-topic".to_string()));
        assert_eq!(builder.batch_size, Some(1000));
        assert_eq!(builder.linger_ms, Some(100));
        assert_eq!(builder.queue_buffering_max_messages, Some(10000));
        assert_eq!(builder.queue_buffering_max_kbytes, Some(1024));
        assert_eq!(builder.reconnect_backoff_min_ms, Some(100));
        assert_eq!(builder.reconnect_backoff_max_ms, Some(30000));
    }

    #[test]
    fn test_builder_missing_brokers_error() {
        let result = KafkaWriterBuilder::new().topic("test-topic").build();

        assert!(result.is_err());
        match result {
            Err(KafkaWriterError::ProducerCreation(
                rdkafka::error::KafkaError::ClientCreation(msg),
            )) => {
                assert!(msg.contains("No brokers specified"));
            }
            _ => panic!("Expected ProducerCreation error with missing brokers message"),
        }
    }

    #[test]
    fn test_builder_missing_topic_error() {
        let result = KafkaWriterBuilder::new()
            .brokers(vec!["localhost:9092".to_string()])
            .build();

        assert!(result.is_err());
        match result {
            Err(KafkaWriterError::ProducerCreation(
                rdkafka::error::KafkaError::ClientCreation(msg),
            )) => {
                assert!(msg.contains("No topic specified"));
            }
            _ => panic!("Expected ProducerCreation error with missing topic message"),
        }
    }

    #[test]
    fn test_builder_valid_config() {
        let result = KafkaWriterBuilder::new()
            .brokers(vec!["localhost:9092".to_string()])
            .topic("test-topic")
            .build();

        match result {
            Ok(_) => {}
            Err(KafkaWriterError::MetadataFetch(_)) => {}
            Err(KafkaWriterError::ProducerCreation(_)) => {}
        }
    }
}
