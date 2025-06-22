//! Provides a customizable [`tracing_subscriber::Layer`] ([`JsonFormattingLayer`]) for formatting
//! log events into a JSON structure.

use std::{
    collections::{HashMap, HashSet},
    fmt,
    io::Write,
    sync::Arc,
};

use serde::ser::{SerializeMap, Serializer};
use serde_json::{Value, ser::Formatter};
use time::format_description::well_known::Iso8601;
use tracing::{Event, Metadata, Subscriber, span::Id};
use tracing_subscriber::{
    Layer,
    fmt::MakeWriter,
    layer::Context,
    registry::{LookupSpan, SpanRef},
};

use super::{AdditionalFieldsPlacement, LoggerError, storage::Storage};

/// Configuration for creating a [`JsonFormattingLayer`].
///
/// This struct defines settings that customize the JSON output, such as:
/// - Statically defined top-level fields (e.g., service name, environment).
/// - Keys from event or span data that should be promoted to the top level.
/// - Behavior for logging span lifecycles (entries and exits).
/// - Placement of additional (non-top-level) fields.
#[derive(Clone, Debug)]
pub struct JsonFormattingLayerConfig {
    /// A map of key-value pairs that are statically defined at initialization and included at the
    /// top level of every log entry.
    pub static_top_level_fields: HashMap<String, Value>,

    /// A set of keys which will be promoted to the top level of the JSON output,
    /// if they appear in a log event or span's dynamic data.
    pub top_level_keys: HashSet<&'static str>,

    /// If `true`, logs all span entries and exits.
    /// If `false`, does not log span entries and only logs exits for root spans.
    pub log_span_lifecycles: bool,

    /// Specifies how additional fields (not designated as top-level) are placed in the JSON output.
    pub additional_fields_placement: AdditionalFieldsPlacement,
}

/// Describes the type of a tracing record.
#[derive(Clone, Copy, Debug)]
pub enum RecordType {
    /// Indicates entering a span.
    EnterSpan,

    /// Indicates exiting a span.
    ExitSpan,

    /// Indicates a standalone event.
    Event,
}

impl fmt::Display for RecordType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let repr = match self {
            Self::EnterSpan => "START",
            Self::ExitSpan => "END",
            Self::Event => "EVENT",
        };
        write!(f, "{repr}")
    }
}

/// A [`tracing_subscriber::Layer`] that formats tracing events and span data into a JSON structure.
///
/// This layer is responsible for serializing log records according to the provided
/// [`JsonFormattingLayerConfig`].
/// It includes standard logging metadata (like timestamp, level, target, PID, hostname)
/// and integrates fields from `static_top_level_fields`, `top_level_keys`,
/// and other event/span data based on the configuration.
///
/// It requires a [`MakeWriter`] to determine the output destination and a
/// [`serde_json::ser::Formatter`] to control the JSON output style
/// (e.g., compact or pretty-printed).
#[derive(Debug)]
pub struct JsonFormattingLayer<W, F>
where
    W: for<'a> MakeWriter<'a> + 'static,
    F: Formatter + Clone,
{
    dst_writer: W,
    formatter: F,
    pid: u32,
    hostname: String,
    static_top_level_fields: HashMap<String, Value>,
    top_level_keys: Arc<HashSet<&'static str>>,
    log_span_lifecycles: bool,
    additional_fields_placement: AdditionalFieldsPlacement,
}

impl<W, F> JsonFormattingLayer<W, F>
where
    W: for<'a> MakeWriter<'a> + 'static,
    F: Formatter + Clone,
{
    /// Creates a new [`JsonFormattingLayer`] with the specified configuration, writer and
    /// formatter.
    pub fn new(
        config: JsonFormattingLayerConfig,
        dst_writer: W,
        formatter: F,
    ) -> Result<Self, LoggerError> {
        let pid = std::process::id();
        let hostname = gethostname::gethostname().to_string_lossy().into_owned();

        for key in config.static_top_level_fields.keys() {
            if super::keys::IMPLICIT_KEYS.contains(key.as_str()) {
                return Err(LoggerError::Configuration(format!(
                    "A reserved key `{key}` was included in `static_top_level_fields` in the \
                     log formatting layer"
                )));
            }
        }

        Ok(Self {
            dst_writer,
            formatter,
            pid,
            hostname,
            static_top_level_fields: config.static_top_level_fields,
            top_level_keys: Arc::new(config.top_level_keys),
            log_span_lifecycles: config.log_span_lifecycles,
            additional_fields_placement: config.additional_fields_placement,
        })
    }

    /// Serializes implicit fields.
    fn serialize_implicit_fields(
        &self,
        map_serializer: &mut impl SerializeMap<Error = serde_json::Error>,
        metadata: &Metadata<'_>,
        name: &str,
        message: &str,
    ) -> Result<(), LoggerError> {
        use super::keys;

        map_serializer.serialize_entry(keys::MESSAGE, message)?;
        map_serializer.serialize_entry(keys::HOSTNAME, &self.hostname)?;
        map_serializer.serialize_entry(keys::PID, &self.pid)?;
        map_serializer.serialize_entry(keys::LEVEL, &format_args!("{}", metadata.level()))?;
        map_serializer.serialize_entry(keys::TARGET, metadata.target())?;
        map_serializer.serialize_entry(keys::LINE, &metadata.line())?;
        map_serializer.serialize_entry(keys::FILE, &metadata.file())?;
        map_serializer.serialize_entry(keys::FN, name)?;
        map_serializer.serialize_entry(
            keys::FULL_NAME,
            &format_args!("{}::{}", metadata.target(), name),
        )?;

        if let Ok(time) = time::UtcDateTime::now().format(&Iso8601::DEFAULT) {
            map_serializer.serialize_entry(keys::TIME, &time)?;
        }

        Ok(())
    }

    /// Common serialization implementation used to serialize both event and span fields.
    fn common_serialize<S>(
        &self,
        map_serializer: &mut impl SerializeMap<Error = serde_json::Error>,
        metadata: &Metadata<'_>,
        span: Option<&SpanRef<'_, S>>,
        storage: Option<&Storage<'_>>,
        name: &str,
        message: &str,
    ) -> Result<(), LoggerError>
    where
        S: Subscriber + for<'a> LookupSpan<'a>,
    {
        // Serialize implicit fields
        self.serialize_implicit_fields(map_serializer, metadata, name, message)?;

        // Serialize static top-level fields
        for (key, value) in self.static_top_level_fields.iter() {
            map_serializer.serialize_entry(key, value)?;
        }

        let mut explicit_entries_set: HashSet<&str> = HashSet::default();
        let mut fields_to_nest: Option<HashMap<String, Value>> = None;

        // Initialize the map if nesting is enabled
        if self.additional_fields_placement.is_nested() {
            fields_to_nest = Some(HashMap::new());
        }

        if let Some(storage) = storage {
            // Serialize event fields
            for (key, value) in storage.values() {
                if super::keys::IMPLICIT_KEYS.contains(*key) {
                    tracing::warn!(
                        "Attempting to log a reserved key `{key}` (value: `{value:?}`) via event. \
                         Skipping."
                    );
                } else if self.top_level_keys.contains(*key) {
                    map_serializer.serialize_entry(key, value)?;
                    explicit_entries_set.insert(*key);
                } else {
                    if self.additional_fields_placement.is_nested() {
                        if let Some(map) = fields_to_nest.as_mut() {
                            map.insert(key.to_string(), value.clone());
                        }
                    } else {
                        map_serializer.serialize_entry(key, value)?;
                    }
                    explicit_entries_set.insert(key);
                }
            }
        }

        // Serialize span fields
        if let Some(span_ref) = &span {
            let extensions = span_ref.extensions();
            if let Some(visitor) = extensions.get::<Storage<'_>>() {
                for (key, value) in visitor
                    .values()
                    .iter()
                    .filter(|(k, _v)| !explicit_entries_set.contains(*k))
                {
                    if super::keys::IMPLICIT_KEYS.contains(*key) {
                        tracing::warn!(
                            "Attempting to log a reserved key `{key}` (value: `{value:?}`) via span. \
                             Skipping."
                        );
                    } else if self.top_level_keys.contains(*key) {
                        map_serializer.serialize_entry(key, value)?;
                    } else if self.additional_fields_placement.is_nested() {
                        if let Some(map) = fields_to_nest.as_mut() {
                            map.insert(key.to_string(), value.clone());
                        }
                    } else {
                        map_serializer.serialize_entry(key, value)?;
                    }
                }
            }
        }

        // Serialize the collected fields_to_nest if nesting is enabled and if map is not empty
        if let AdditionalFieldsPlacement::Nested(field_name) = &self.additional_fields_placement {
            if let Some(map) = fields_to_nest {
                if !map.is_empty() {
                    map_serializer.serialize_entry(field_name.as_str(), &map)?;
                }
            }
        }

        Ok(())
    }

    /// Flush memory buffer into an output stream with a trailing newline.
    ///
    /// Should be done by a single `write_all` call to avoid fragmentation of log because of
    /// multithreading.
    fn flush(&self, mut buffer: Vec<u8>) -> Result<(), std::io::Error> {
        buffer.write_all(b"\n")?;
        self.dst_writer.make_writer().write_all(&buffer)
    }

    /// Serialize entries of a span.
    fn span_serialize<S>(
        &self,
        span: &SpanRef<'_, S>,
        ty: RecordType,
    ) -> Result<Vec<u8>, LoggerError>
    where
        S: Subscriber + for<'a> LookupSpan<'a>,
    {
        let mut buffer = Vec::new();
        let mut serializer =
            serde_json::Serializer::with_formatter(&mut buffer, self.formatter.clone());
        let mut map_serializer = serializer.serialize_map(None)?;

        let message = Self::span_message(span, ty);

        self.common_serialize(
            &mut map_serializer,
            span.metadata(),
            Some(span),
            None,
            span.name(),
            &message,
        )?;

        map_serializer.end()?;
        Ok(buffer)
    }

    /// Serialize entries from an event and its parent span.
    fn event_serialize<S>(
        &self,
        span: Option<&SpanRef<'_, S>>,
        event: &Event<'_>,
    ) -> Result<Vec<u8>, LoggerError>
    where
        S: Subscriber + for<'a> LookupSpan<'a>,
    {
        let mut buffer = Vec::new();
        let mut serializer =
            serde_json::Serializer::with_formatter(&mut buffer, self.formatter.clone());
        let mut map_serializer = serializer.serialize_map(None)?;

        let mut storage = Storage::default();
        event.record(&mut storage);

        let name = span.map_or("?", SpanRef::name);
        let message = Self::event_message(span, event, &storage);

        self.common_serialize(
            &mut map_serializer,
            event.metadata(),
            span,
            Some(&storage),
            name,
            &message,
        )?;

        map_serializer.end()?;
        Ok(buffer)
    }

    /// Format the message for a span.
    ///
    /// Example: "[FN_WITHOUT_COLON - START]"
    fn span_message<S>(span: &SpanRef<'_, S>, ty: RecordType) -> String
    where
        S: Subscriber + for<'a> LookupSpan<'a>,
    {
        format!("[{} - {}]", span.metadata().name().to_uppercase(), ty)
    }

    /// Format the message for an event.
    ///
    /// Examples: "[FN_WITHOUT_COLON - EVENT] Message"
    fn event_message<S>(
        span: Option<&SpanRef<'_, S>>,
        event: &Event<'_>,
        storage: &Storage<'_>,
    ) -> String
    where
        S: Subscriber + for<'a> LookupSpan<'a>,
    {
        // Obtain the value of the `message` field if set, or the `target` from metadata otherwise.
        let message = storage
            .message()
            .unwrap_or_else(|| event.metadata().target())
            .to_string();

        // Prepend the span name to the message if span exists.
        if let Some(span) = span {
            format!(
                "{} {}",
                Self::span_message(span, RecordType::Event),
                message
            )
        } else {
            message
        }
    }
}

impl<S, W, F> Layer<S> for JsonFormattingLayer<W, F>
where
    S: Subscriber + for<'a> LookupSpan<'a>,
    W: for<'a> MakeWriter<'a> + 'static,
    F: Formatter + Clone + 'static,
{
    fn on_event(&self, event: &Event<'_>, ctx: Context<'_, S>) {
        // Obtain the parent span for the event
        let span = ctx.lookup_current();

        let result = self.event_serialize(span.as_ref(), event);
        if let Ok(serialized) = result {
            let _ = self.flush(serialized);
        }
    }

    fn on_enter(&self, id: &Id, ctx: Context<'_, S>) {
        if self.log_span_lifecycles {
            #[allow(clippy::expect_used)]
            let span = ctx
                .span(id)
                .expect("span with specified id does not exist in `on_enter()`");

            if let Ok(serialized) = self.span_serialize(&span, RecordType::EnterSpan) {
                let _ = self.flush(serialized);
            }
        }
    }

    fn on_close(&self, id: Id, ctx: Context<'_, S>) {
        #[allow(clippy::expect_used)]
        let span = ctx
            .span(&id)
            .expect("span with specified id does not exist in `on_close()`");

        let should_log_exit = if self.log_span_lifecycles {
            true // Log all exits if full lifecycle is enabled
        } else {
            span.parent().is_none() // Only log root span exits otherwise
        };

        if should_log_exit {
            if let Ok(serialized) = self.span_serialize(&span, RecordType::ExitSpan) {
                let _ = self.flush(serialized);
            }
        }
    }
}
