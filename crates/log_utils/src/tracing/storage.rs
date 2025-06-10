//! Provides a [`tracing_subscriber::Layer`] ([`SpanStorageLayer`]) for capturing and storing
//! key-value data from tracing spans.

use std::{
    collections::{HashMap, HashSet},
    fmt,
    time::Instant,
};

use tracing::{
    Id, Subscriber,
    field::{Field, Visit},
    span::{Attributes, Record},
};
use tracing_subscriber::{Layer, layer::Context};

/// A [`tracing_subscriber::Layer`] that enables storing key-value data within span extensions.
/// It also handles propagation of "persistent" keys to parent spans and records span duration.
#[derive(Clone, Debug)]
pub struct SpanStorageLayer {
    persistent_keys: HashSet<&'static str>,
}

impl SpanStorageLayer {
    /// Creates a new [`SpanStorageLayer`] layer with the specified persistent keys.
    ///
    /// The values of persistent keys would be propagated to parent spans, if they are set or
    /// updated in the current span.
    pub fn new(persistent_keys: impl IntoIterator<Item = &'static str>) -> Self {
        Self {
            persistent_keys: HashSet::from_iter(persistent_keys),
        }
    }
}

/// Holds key-value data recorded for a span or an event.
///
/// This struct is typically stored in a span's extensions via [`SpanStorageLayer`].
#[derive(Clone, Debug, Default)]
pub(crate) struct Storage<'a> {
    /// The collected key-value pairs for the span.
    values: HashMap<&'a str, serde_json::Value>,

    /// The primary message of an event, if captured.
    message: Option<String>,
}

impl<'a> Storage<'a> {
    /// Records a key-value pair into the storage.
    ///
    /// If the `key` is one of the [`IMPLICIT_KEYS`][crate::keys::IMPLICIT_KEYS],
    /// a warning is logged, and the value is not inserted.
    pub(crate) fn record_value(&mut self, key: &'a str, value: serde_json::Value) {
        if super::keys::IMPLICIT_KEYS.contains(key) {
            tracing::warn!(
                "Attempting to record a reserved key `{key}` (value: {value:?}). Skipping."
            );
        } else {
            self.values.insert(key, value);
        }
    }

    pub(crate) fn values(&self) -> &HashMap<&'a str, serde_json::Value> {
        &self.values
    }

    pub(crate) fn message(&self) -> Option<&str> {
        self.message.as_deref()
    }
}

// Implement `Visit` to capture span or event fields into the `Storage` map.
impl Visit for Storage<'_> {
    fn record_f64(&mut self, field: &Field, value: f64) {
        if field.name() == super::keys::MESSAGE {
            if self.message.is_none() {
                self.message = Some(value.to_string());
            }
        } else {
            self.record_value(field.name(), serde_json::Value::from(value));
        }
    }

    fn record_i64(&mut self, field: &Field, value: i64) {
        if field.name() == super::keys::MESSAGE {
            if self.message.is_none() {
                self.message = Some(value.to_string());
            }
        } else {
            self.record_value(field.name(), serde_json::Value::from(value));
        }
    }

    fn record_u64(&mut self, field: &Field, value: u64) {
        if field.name() == super::keys::MESSAGE {
            if self.message.is_none() {
                self.message = Some(value.to_string());
            }
        } else {
            self.record_value(field.name(), serde_json::Value::from(value));
        }
    }

    fn record_bool(&mut self, field: &Field, value: bool) {
        if field.name() == super::keys::MESSAGE {
            if self.message.is_none() {
                self.message = Some(value.to_string());
            }
        } else {
            self.record_value(field.name(), serde_json::Value::from(value));
        }
    }

    fn record_str(&mut self, field: &Field, value: &str) {
        if field.name() == super::keys::MESSAGE {
            self.message = Some(value.to_string()); // `record_str()` is preferred for `message`
        } else {
            self.record_value(field.name(), serde_json::Value::from(value));
        }
    }

    fn record_debug(&mut self, field: &Field, value: &dyn fmt::Debug) {
        if field.name() == super::keys::MESSAGE {
            if self.message.is_none() {
                // Only use debug if `record_str()` hasn't set it
                self.message = Some(format!("{value:?}"));
            }
        } else {
            match field.name() {
                // Skip fields which are already handled
                name if name.starts_with("log.") => (),
                name if name.starts_with("r#") => {
                    self.record_value(
                        #[allow(clippy::expect_used)]
                        name.get(2..).expect(
                            "field name using raw identifiers must have at least two characters",
                        ),
                        serde_json::Value::from(format!("{value:?}")),
                    );
                }
                name => {
                    self.record_value(name, serde_json::Value::from(format!("{value:?}")));
                }
            };
        }
    }
}

impl<S: Subscriber + for<'a> tracing_subscriber::registry::LookupSpan<'a>> Layer<S>
    for SpanStorageLayer
{
    fn on_new_span(&self, attrs: &Attributes<'_>, id: &Id, ctx: Context<'_, S>) {
        #[allow(clippy::expect_used)]
        let span = ctx
            .span(id)
            .expect("span with specified id does not exist in `on_new_span()`");
        let mut extensions = span.extensions_mut();

        // Inherit storage from parent span if it exists, otherwise create a new span.
        let mut visitor = if let Some(parent_span) = span.parent() {
            parent_span
                .extensions()
                .get::<Storage<'_>>()
                .cloned()
                .unwrap_or_default()
        } else {
            Storage::default()
        };

        attrs.record(&mut visitor);
        extensions.insert(visitor);
    }

    fn on_record(&self, span_id: &Id, values: &Record<'_>, ctx: Context<'_, S>) {
        #[allow(clippy::expect_used)]
        let span = ctx
            .span(span_id)
            .expect("span with specified id does not exist in `on_record()`");
        let mut extensions = span.extensions_mut();

        #[allow(clippy::expect_used)]
        let visitor = extensions
            .get_mut::<Storage<'_>>()
            .expect("span does not have storage in `on_record()`");

        values.record(visitor);
    }

    fn on_enter(&self, span_id: &Id, ctx: Context<'_, S>) {
        #[allow(clippy::expect_used)]
        let span = ctx
            .span(span_id)
            .expect("span with specified id does not exist in `on_enter()`");
        let mut extensions = span.extensions_mut();

        // Store the current time in the span if it doesn't already exist
        if extensions.get_mut::<Instant>().is_none() {
            extensions.insert(Instant::now());
        }
    }

    fn on_close(&self, id: Id, ctx: Context<'_, S>) {
        #[allow(clippy::expect_used)]
        let span = ctx
            .span(&id)
            .expect("span with specified id does not exist in `on_close()`");

        let elapsed_milliseconds = span
            .extensions()
            .get::<Instant>()
            .map(|i| i.elapsed().as_millis())
            .unwrap_or(0);

        // Propagate persistent keys to parent
        if let Some(storage) = span.extensions().get::<Storage<'_>>() {
            storage
                .values
                .iter()
                .filter(|(k, _v)| self.persistent_keys.contains(*k))
                .for_each(|(k, v)| {
                    span.parent().and_then(|parent_span| {
                        parent_span
                            .extensions_mut()
                            .get_mut::<Storage<'_>>()
                            .map(|parent_storage| parent_storage.record_value(k, v.to_owned()))
                    });
                });
        }

        let mut extensions = span.extensions_mut();
        #[allow(clippy::expect_used)]
        let visitor = extensions
            .get_mut::<Storage<'_>>()
            .expect("span does not have storage in `on_close()`");

        // Record elapsed time in the span's storage
        if let Ok(elapsed_time_value) = serde_json::to_value(elapsed_milliseconds) {
            visitor.record_value(super::keys::ELAPSED_MILLISECONDS, elapsed_time_value);
        }
    }
}
