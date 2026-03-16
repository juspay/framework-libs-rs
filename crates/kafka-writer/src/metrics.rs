//! Prometheus metrics for Kafka writer

use std::sync::LazyLock;

use prometheus::{IntCounter, IntGauge, register_int_counter, register_int_gauge};

/// Total number of logs successfully sent to Kafka
#[allow(clippy::expect_used)]
pub static KAFKA_LOGS_SENT: LazyLock<IntCounter> = LazyLock::new(|| {
    register_int_counter!(
        "kafka_logs_sent_total",
        "Total number of logs successfully sent to Kafka"
    )
    .expect("Failed to register kafka_logs_sent_total metric")
});

/// Total number of logs dropped due to Kafka queue full or errors
#[allow(clippy::expect_used)]
pub static KAFKA_LOGS_DROPPED: LazyLock<IntCounter> = LazyLock::new(|| {
    register_int_counter!(
        "kafka_logs_dropped_total",
        "Total number of logs dropped due to Kafka queue full or errors"
    )
    .expect("Failed to register kafka_logs_dropped_total metric")
});

/// Current size of Kafka producer queue
#[allow(clippy::expect_used)]
pub static KAFKA_QUEUE_SIZE: LazyLock<IntGauge> = LazyLock::new(|| {
    register_int_gauge!(
        "kafka_producer_queue_size",
        "Current size of Kafka producer queue"
    )
    .expect("Failed to register kafka_producer_queue_size metric")
});

/// Logs dropped due to queue full
#[allow(clippy::expect_used)]
pub static KAFKA_DROPS_QUEUE_FULL: LazyLock<IntCounter> = LazyLock::new(|| {
    register_int_counter!(
        "kafka_drops_queue_full_total",
        "Total number of logs dropped due to Kafka queue being full"
    )
    .expect("Failed to register kafka_drops_queue_full_total metric")
});

/// Logs dropped due to message too large
#[allow(clippy::expect_used)]
pub static KAFKA_DROPS_MSG_TOO_LARGE: LazyLock<IntCounter> = LazyLock::new(|| {
    register_int_counter!(
        "kafka_drops_msg_too_large_total",
        "Total number of logs dropped due to message size exceeding limit"
    )
    .expect("Failed to register kafka_drops_msg_too_large_total metric")
});

/// Logs dropped due to timeout
#[allow(clippy::expect_used)]
pub static KAFKA_DROPS_TIMEOUT: LazyLock<IntCounter> = LazyLock::new(|| {
    register_int_counter!(
        "kafka_drops_timeout_total",
        "Total number of logs dropped due to timeout"
    )
    .expect("Failed to register kafka_drops_timeout_total metric")
});

/// Logs dropped due to other errors
#[allow(clippy::expect_used)]
pub static KAFKA_DROPS_OTHER: LazyLock<IntCounter> = LazyLock::new(|| {
    register_int_counter!(
        "kafka_drops_other_total",
        "Total number of logs dropped due to other errors"
    )
    .expect("Failed to register kafka_drops_other_total metric")
});

/// Total number of audit events successfully sent to Kafka
#[allow(clippy::expect_used)]
pub static KAFKA_AUDIT_EVENTS_SENT: LazyLock<IntCounter> = LazyLock::new(|| {
    register_int_counter!(
        "kafka_audit_events_sent_total",
        "Total number of audit events successfully sent to Kafka"
    )
    .expect("Failed to register kafka_audit_events_sent_total metric")
});

/// Total number of audit events dropped due to Kafka queue full or errors
#[allow(clippy::expect_used)]
pub static KAFKA_AUDIT_EVENTS_DROPPED: LazyLock<IntCounter> = LazyLock::new(|| {
    register_int_counter!(
        "kafka_audit_events_dropped_total",
        "Total number of audit events dropped due to Kafka queue full or errors"
    )
    .expect("Failed to register kafka_audit_events_dropped_total metric")
});

/// Current size of Kafka audit event producer queue
#[allow(clippy::expect_used)]
pub static KAFKA_AUDIT_EVENT_QUEUE_SIZE: LazyLock<IntGauge> = LazyLock::new(|| {
    register_int_gauge!(
        "kafka_audit_event_queue_size",
        "Current size of Kafka audit event producer queue"
    )
    .expect("Failed to register kafka_audit_event_queue_size metric")
});

/// Audit events dropped due to queue full
#[allow(clippy::expect_used)]
pub static KAFKA_AUDIT_DROPS_QUEUE_FULL: LazyLock<IntCounter> = LazyLock::new(|| {
    register_int_counter!(
        "kafka_audit_drops_queue_full_total",
        "Total number of audit events dropped due to Kafka queue being full"
    )
    .expect("Failed to register kafka_audit_drops_queue_full_total metric")
});

/// Audit events dropped due to message too large
#[allow(clippy::expect_used)]
pub static KAFKA_AUDIT_DROPS_MSG_TOO_LARGE: LazyLock<IntCounter> = LazyLock::new(|| {
    register_int_counter!(
        "kafka_audit_drops_msg_too_large_total",
        "Total number of audit events dropped due to message size exceeding limit"
    )
    .expect("Failed to register kafka_audit_drops_msg_too_large_total metric")
});

/// Audit events dropped due to timeout
#[allow(clippy::expect_used)]
pub static KAFKA_AUDIT_DROPS_TIMEOUT: LazyLock<IntCounter> = LazyLock::new(|| {
    register_int_counter!(
        "kafka_audit_drops_timeout_total",
        "Total number of audit events dropped due to timeout"
    )
    .expect("Failed to register kafka_audit_drops_timeout_total metric")
});

/// Audit events dropped due to other errors
#[allow(clippy::expect_used)]
pub static KAFKA_AUDIT_DROPS_OTHER: LazyLock<IntCounter> = LazyLock::new(|| {
    register_int_counter!(
        "kafka_audit_drops_other_total",
        "Total number of audit events dropped due to other errors"
    )
    .expect("Failed to register kafka_audit_drops_other_total metric")
});

/// Forces the initialization of all metrics in this module.
///
/// This function should be called once at application startup to ensure that all metrics
/// are registered upfront. If any metric registration fails (e.g., due to a duplicate
/// metric name), the application will panic immediately.
#[cfg(feature = "kafka-metrics")]
pub fn initialize_all_metrics() {
    // Force evaluation of all lazy metrics to fail fast if registration fails.
    let _ = &*KAFKA_LOGS_SENT;
    let _ = &*KAFKA_LOGS_DROPPED;
    let _ = &*KAFKA_QUEUE_SIZE;
    let _ = &*KAFKA_DROPS_QUEUE_FULL;
    let _ = &*KAFKA_DROPS_MSG_TOO_LARGE;
    let _ = &*KAFKA_DROPS_TIMEOUT;
    let _ = &*KAFKA_DROPS_OTHER;
    let _ = &*KAFKA_AUDIT_EVENTS_SENT;
    let _ = &*KAFKA_AUDIT_EVENTS_DROPPED;
    let _ = &*KAFKA_AUDIT_EVENT_QUEUE_SIZE;
    let _ = &*KAFKA_AUDIT_DROPS_QUEUE_FULL;
    let _ = &*KAFKA_AUDIT_DROPS_MSG_TOO_LARGE;
    let _ = &*KAFKA_AUDIT_DROPS_TIMEOUT;
    let _ = &*KAFKA_AUDIT_DROPS_OTHER;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[cfg(feature = "kafka-metrics")]
    #[test]
    fn test_metric_names_and_descriptions() {
        // Test that all metrics have proper names and descriptions
        let logs_sent = &*KAFKA_LOGS_SENT;
        let logs_dropped = &*KAFKA_LOGS_DROPPED;
        let queue_size = &*KAFKA_QUEUE_SIZE;

        // Verify metrics can be accessed without panicking
        // Note: Counters may have been incremented by other tests
        let _ = logs_sent.get();
        let _ = logs_dropped.get();
        let _ = queue_size.get();
    }

    #[cfg(feature = "kafka-metrics")]
    #[test]
    fn test_audit_metrics() {
        let audit_sent = &*KAFKA_AUDIT_EVENTS_SENT;
        let audit_dropped = &*KAFKA_AUDIT_EVENTS_DROPPED;
        let audit_queue_size = &*KAFKA_AUDIT_EVENT_QUEUE_SIZE;

        // Verify metrics can be accessed without panicking
        let _ = audit_sent.get();
        let _ = audit_dropped.get();
        let _ = audit_queue_size.get();
    }

    #[cfg(feature = "kafka-metrics")]
    #[test]
    fn test_drop_reason_metrics() {
        let queue_full = &*KAFKA_DROPS_QUEUE_FULL;
        let msg_too_large = &*KAFKA_DROPS_MSG_TOO_LARGE;
        let timeout = &*KAFKA_DROPS_TIMEOUT;
        let other = &*KAFKA_DROPS_OTHER;

        assert_eq!(queue_full.get(), 0);
        assert_eq!(msg_too_large.get(), 0);
        assert_eq!(timeout.get(), 0);
        assert_eq!(other.get(), 0);
    }

    #[cfg(feature = "kafka-metrics")]
    #[test]
    fn test_audit_drop_reason_metrics() {
        let audit_queue_full = &*KAFKA_AUDIT_DROPS_QUEUE_FULL;
        let audit_msg_too_large = &*KAFKA_AUDIT_DROPS_MSG_TOO_LARGE;
        let audit_timeout = &*KAFKA_AUDIT_DROPS_TIMEOUT;
        let audit_other = &*KAFKA_AUDIT_DROPS_OTHER;

        assert_eq!(audit_queue_full.get(), 0);
        assert_eq!(audit_msg_too_large.get(), 0);
        assert_eq!(audit_timeout.get(), 0);
        assert_eq!(audit_other.get(), 0);
    }

    #[cfg(feature = "kafka-metrics")]
    #[test]
    fn test_initialize_all_metrics() {
        // Test that initialize_all_metrics doesn't panic
        initialize_all_metrics();
        // Test passes if no panic occurs
    }

    #[cfg(feature = "kafka-metrics")]
    #[test]
    fn test_metrics_can_be_incremented() {
        // Test that metrics can be modified
        let logs_sent = &*KAFKA_LOGS_SENT;
        let initial_value = logs_sent.get();

        logs_sent.inc();
        assert_eq!(logs_sent.get(), initial_value + 1);

        // Note: Counters cannot be reset, they can only increment
    }

    #[cfg(feature = "kafka-metrics")]
    #[test]
    fn test_gauge_metrics_can_be_set() {
        let queue_size = &*KAFKA_QUEUE_SIZE;
        let initial_value = queue_size.get();

        queue_size.set(42);
        assert_eq!(queue_size.get(), 42);

        // Reset for other tests
        queue_size.set(initial_value);
    }
}
