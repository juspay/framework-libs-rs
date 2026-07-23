/// Create a global [`Meter`][opentelemetry::metrics::Meter] with the specified name.
///
/// The meter is obtained from the global meter provider.
/// Until the application sets the global provider
/// (via [`MetricsHandle::register_as_global`][crate::MetricsHandle::register_as_global]
/// or [`opentelemetry::global::set_meter_provider`]), the meter and any instruments created from
/// it are no-ops, and recorded measurements are discarded.
///
/// # Example
///
/// ```
/// use metrics_utils::global_meter;
///
/// global_meter!(pub FOO_METER, "foo");
/// global_meter!(pub BAR_METER);
/// ```
#[macro_export]
macro_rules! global_meter {
    // Explicit meter name.
    ($vis:vis $meter:ident, $name:literal) => {
        $vis static $meter: ::std::sync::LazyLock<
            $crate::opentelemetry::Meter
        > = ::std::sync::LazyLock::new(|| {
            $crate::opentelemetry::global::meter($name)
        });
    };

    // Meter name derived from the identifier.
    ($vis:vis $meter:ident) => {
        $vis static $meter: ::std::sync::LazyLock<
            $crate::opentelemetry::Meter
        > = ::std::sync::LazyLock::new(|| {
            $crate::opentelemetry::global::meter(
                ::std::stringify!($meter),
            )
        });
    };
}

/// Create a [`Counter<u64>`][opentelemetry::metrics::Counter] with the given name.
///
/// # Example
///
/// ```
/// use metrics_utils::{counter_metric, global_meter};
///
/// global_meter!(pub MY_METER, "my_service");
///
/// counter_metric!(
///     pub REQUEST_COUNT, MY_METER,
///     name: "request.count",
///     description: "Number of incoming requests",
/// );
/// ```
#[macro_export]
macro_rules! counter_metric {
    // Entry arm: capture all keyword arguments and start the builder chain.
    // The metric name defaults to the Rust identifier (stringified).
    // Remaining tokens (`$rest`) are passed to the recursive builder which processes
    // `name:` (must appear first if provided), then `description:`, `unit:` in any order,
    // each transforming the builder expression, until no tokens remain and the final `static`
    // binding is emitted.
    ($vis:vis $name:ident, $meter:ident $($rest:tt)*) => {
        $crate::__build_counter_metric! { $vis $name, $meter [$($rest)*]
            $meter.u64_counter(::std::stringify!($name)) }
    };
}

/// Internal build helper for [`counter_metric!`].
///
/// Walks the keyword token stream (`name:`, `description:`, `unit:`, commas) one token at a time,
/// threading the instrument builder expression through recursion, and emits the final `static`
/// binding in the base case.
#[doc(hidden)]
#[macro_export]
macro_rules! __build_counter_metric {
    // `name:` overrides the default metric name derived from the identifier.
    ($vis:vis $name:ident, $meter:ident
        [name: $metric_name:literal $($rest:tt)*]
        $($expr:tt)*
    ) => {
        $crate::__build_counter_metric! { $vis $name, $meter [$($rest)*]
            $meter.u64_counter($metric_name) }
    };

    // `description:` attaches a human-readable description to the instrument.
    ($vis:vis $name:ident, $meter:ident
        [description: $v:literal $($rest:tt)*]
        $($expr:tt)*
    ) => {
        $crate::__build_counter_metric! { $vis $name, $meter [$($rest)*]
            $($expr)* .with_description($v) }
    };

    // `unit:` attaches the unit of measurement to the instrument.
    ($vis:vis $name:ident, $meter:ident
        [unit: $v:literal $($rest:tt)*]
        $($expr:tt)*
    ) => {
        $crate::__build_counter_metric! { $vis $name, $meter [$($rest)*]
            $($expr)* .with_unit($v) }
    };

    // Comma skimmer: strips a leading comma so the next keyword can match.
    ($vis:vis $name:ident, $meter:ident
        [, $($rest:tt)*]
        $($expr:tt)*
    ) => {
        $crate::__build_counter_metric! { $vis $name, $meter [$($rest)*]
            $($expr)* }
    };

    // Token stream exhausted, emit the final `static` counter binding.
    ($vis:vis $name:ident, $meter:ident [] $($expr:tt)*) => {
        $vis static $name: ::std::sync::LazyLock<
            $crate::opentelemetry::Counter<u64>
        > = ::std::sync::LazyLock::new(|| { $($expr)* .build() });
    };
}

/// Create an [`UpDownCounter<i64>`][opentelemetry::metrics::UpDownCounter] with the given name.
///
/// # Example
///
/// ```
/// use metrics_utils::{up_down_counter_metric, global_meter};
///
/// global_meter!(pub MY_METER, "my_service");
///
/// up_down_counter_metric!(
///     pub ACTIVE_CONNECTIONS, MY_METER,
///     name: "active.connections",
///     description: "Number of active connections",
/// );
/// ```
#[macro_export]
macro_rules! up_down_counter_metric {
    // Entry arm: capture all keyword arguments and start the builder chain.
    // Remaining tokens (`$rest`) are passed to the recursive builder which processes
    // `name:` (must appear first if provided), then `description:`, `unit:` in any order,
    // each transforming the builder expression, until no tokens remain and the final `static`
    // binding is emitted.
    ($vis:vis $name:ident, $meter:ident $($rest:tt)*) => {
        $crate::__build_up_down_counter_metric! { $vis $name, $meter [$($rest)*]
            $meter.i64_up_down_counter(::std::stringify!($name)) }
    };
}

/// Internal build helper for [`up_down_counter_metric!`].
///
/// Walks the keyword token stream (`name:`, `description:`, `unit:`, commas) one token at a time,
/// threading the instrument builder expression through recursion, and emits the final `static`
/// binding in the base case.
#[doc(hidden)]
#[macro_export]
macro_rules! __build_up_down_counter_metric {
    // `name:` overrides the default metric name derived from the identifier.
    ($vis:vis $name:ident, $meter:ident
        [name: $metric_name:literal $($rest:tt)*]
        $($expr:tt)*
    ) => {
        $crate::__build_up_down_counter_metric! { $vis $name, $meter [$($rest)*]
            $meter.i64_up_down_counter($metric_name) }
    };

    // `description:` attaches a human-readable description to the instrument.
    ($vis:vis $name:ident, $meter:ident
        [description: $v:literal $($rest:tt)*]
        $($expr:tt)*
    ) => {
        $crate::__build_up_down_counter_metric! { $vis $name, $meter [$($rest)*]
            $($expr)* .with_description($v) }
    };

    // `unit:` attaches the unit of measurement to the instrument.
    ($vis:vis $name:ident, $meter:ident
        [unit: $v:literal $($rest:tt)*]
        $($expr:tt)*
    ) => {
        $crate::__build_up_down_counter_metric! { $vis $name, $meter [$($rest)*]
            $($expr)* .with_unit($v) }
    };

    // Comma skimmer: strips a leading comma so the next keyword can match.
    ($vis:vis $name:ident, $meter:ident
        [, $($rest:tt)*]
        $($expr:tt)*
    ) => {
        $crate::__build_up_down_counter_metric! { $vis $name, $meter [$($rest)*]
            $($expr)* }
    };

    // Token stream exhausted, emit the final `static` up-down counter binding.
    ($vis:vis $name:ident, $meter:ident [] $($expr:tt)*) => {
        $vis static $name: ::std::sync::LazyLock<
            $crate::opentelemetry::UpDownCounter<i64>
        > = ::std::sync::LazyLock::new(|| { $($expr)* .build() });
    };
}

/// Create a [`Gauge<u64>`][opentelemetry::metrics::Gauge] with the given name.
///
/// # Example
///
/// ```
/// use metrics_utils::{gauge_metric, global_meter};
///
/// global_meter!(pub MY_METER, "my_service");
///
/// gauge_metric!(
///     pub CPU_TEMPERATURE, MY_METER,
///     name: "cpu.temperature",
///     description: "Current CPU temperature",
///     unit: "Cel",
/// );
/// ```
#[macro_export]
macro_rules! gauge_metric {
    // Entry arm: capture all keyword arguments and start the builder chain.
    // The metric name defaults to the Rust identifier (stringified).
    // Remaining tokens (`$rest`) are passed to the recursive builder which processes
    // `name:` (must appear first if provided), then `description:`, `unit:` in any order,
    // each transforming the builder expression, until no tokens remain and the final `static`
    // binding is emitted.
    ($vis:vis $name:ident, $meter:ident $($rest:tt)*) => {
        $crate::__build_gauge_metric! { $vis $name, $meter [$($rest)*]
            $meter.u64_gauge(::std::stringify!($name)) }
    };
}

/// Internal build helper for [`gauge_metric!`].
///
/// Walks the keyword token stream (`name:`, `description:`, `unit:`, commas) one token at a time,
/// threading the instrument builder expression through recursion, and emits the final `static`
/// binding in the base case.
#[doc(hidden)]
#[macro_export]
macro_rules! __build_gauge_metric {
    // `name:` overrides the default metric name derived from the identifier.
    ($vis:vis $name:ident, $meter:ident
        [name: $metric_name:literal $($rest:tt)*]
        $($expr:tt)*
    ) => {
        $crate::__build_gauge_metric! { $vis $name, $meter [$($rest)*]
            $meter.u64_gauge($metric_name) }
    };

    // `description:` attaches a human-readable description to the instrument.
    ($vis:vis $name:ident, $meter:ident
        [description: $v:literal $($rest:tt)*]
        $($expr:tt)*
    ) => {
        $crate::__build_gauge_metric! { $vis $name, $meter [$($rest)*]
            $($expr)* .with_description($v) }
    };

    // `unit:` attaches the unit of measurement to the instrument.
    ($vis:vis $name:ident, $meter:ident
        [unit: $v:literal $($rest:tt)*]
        $($expr:tt)*
    ) => {
        $crate::__build_gauge_metric! { $vis $name, $meter [$($rest)*]
            $($expr)* .with_unit($v) }
    };

    // Comma skimmer: strips a leading comma so the next keyword can match.
    ($vis:vis $name:ident, $meter:ident
        [, $($rest:tt)*]
        $($expr:tt)*
    ) => {
        $crate::__build_gauge_metric! { $vis $name, $meter [$($rest)*]
            $($expr)* }
    };

    // Token stream exhausted, emit the final `static` gauge binding.
    ($vis:vis $name:ident, $meter:ident [] $($expr:tt)*) => {
        $vis static $name: ::std::sync::LazyLock<
            $crate::opentelemetry::Gauge<u64>
        > = ::std::sync::LazyLock::new(|| { $($expr)* .build() });
    };
}

/// Create a [`Histogram<f64>`][opentelemetry::metrics::Histogram] with the given name.
///
/// # Example
///
/// ```
/// use metrics_utils::{histogram_metric_f64, global_meter, f64_histogram_buckets};
///
/// global_meter!(pub MY_METER, "my_service");
///
/// histogram_metric_f64!(
///     pub REQUEST_DURATION, MY_METER,
///     name: "request.duration",
///     description: "Request duration in seconds",
///     unit: "s",
///     buckets: f64_histogram_buckets().to_vec(),
/// );
/// ```
#[macro_export]
macro_rules! histogram_metric_f64 {
    // Entry arm: capture all keyword arguments and start the builder chain.
    // The metric name defaults to the Rust identifier (stringified).
    // Remaining tokens (`$rest`) are passed to the recursive builder which processes
    // `name:` (must appear first if provided), then `description:`, `unit:`, `buckets:` in any
    // order, each transforming the builder expression, until no tokens remain and the final
    // `static` binding is emitted.
    ($vis:vis $name:ident, $meter:ident $($rest:tt)*) => {
        $crate::__build_histogram_metric_f64! { $vis $name, $meter [$($rest)*]
            $meter.f64_histogram(::std::stringify!($name)) }
    };
}

/// Internal build helper for [`histogram_metric_f64!`].
///
/// Walks the keyword token stream (`name:`, `description:`, `unit:`, `buckets:`, commas) one token
/// at a time, threading the instrument builder expression through recursion, and emits the final
/// `static` binding in the base case.
#[doc(hidden)]
#[macro_export]
macro_rules! __build_histogram_metric_f64 {
    // `name:` overrides the default metric name derived from the identifier.
    ($vis:vis $name:ident, $meter:ident
        [name: $metric_name:literal $($rest:tt)*]
        $($expr:tt)*
    ) => {
        $crate::__build_histogram_metric_f64! { $vis $name, $meter [$($rest)*]
            $meter.f64_histogram($metric_name) }
    };

    // `description:` attaches a human-readable description to the instrument.
    ($vis:vis $name:ident, $meter:ident
        [description: $v:literal $($rest:tt)*]
        $($expr:tt)*
    ) => {
        $crate::__build_histogram_metric_f64! { $vis $name, $meter [$($rest)*]
            $($expr)* .with_description($v) }
    };

    // `unit:` attaches the unit of measurement to the instrument.
    ($vis:vis $name:ident, $meter:ident
        [unit: $v:literal $($rest:tt)*]
        $($expr:tt)*
    ) => {
        $crate::__build_histogram_metric_f64! { $vis $name, $meter [$($rest)*]
            $($expr)* .with_unit($v) }
    };

    // `buckets:` as the last keyword: no trailing comma, so drain the token stream.
    ($vis:vis $name:ident, $meter:ident
        [buckets: $v:expr]
        $($expr:tt)*
    ) => {
        $crate::__build_histogram_metric_f64! { $vis $name, $meter []
            $($expr)* .with_boundaries($v) }
    };

    // `buckets:` with more keywords following: consume the buckets value and continue.
    ($vis:vis $name:ident, $meter:ident
        [buckets: $v:expr, $($rest:tt)*]
        $($expr:tt)*
    ) => {
        $crate::__build_histogram_metric_f64! { $vis $name, $meter [$($rest)*]
            $($expr)* .with_boundaries($v) }
    };

    // Comma skimmer: strips a leading comma so the next keyword can match.
    ($vis:vis $name:ident, $meter:ident
        [, $($rest:tt)*]
        $($expr:tt)*
    ) => {
        $crate::__build_histogram_metric_f64! { $vis $name, $meter [$($rest)*]
            $($expr)* }
    };

    // Token stream exhausted, emit the final `static` histogram binding.
    ($vis:vis $name:ident, $meter:ident [] $($expr:tt)*) => {
        $vis static $name: ::std::sync::LazyLock<
            $crate::opentelemetry::Histogram<f64>
        > = ::std::sync::LazyLock::new(|| { $($expr)* .build() });
    };
}

/// Create a [`&[KeyValue]`][opentelemetry::KeyValue] array from key-value pairs.
///
/// # Example
///
/// ```
/// use metrics_utils::metric_attributes;
///
/// let attrs = metric_attributes!(("method", "GET"), ("status", "200"));
/// assert_eq!(attrs.len(), 2);
/// ```
#[macro_export]
macro_rules! metric_attributes {
    ($(($key:expr, $value:expr $(,)?)),+ $(,)?) => {
        &[$(
            $crate::opentelemetry::KeyValue::new($key, $value)
        ),+]
    };
}

/// Builds an owned [`Vec<KeyValue>`][opentelemetry::KeyValue] for cases where attributes need to
/// be extended dynamically.
///
/// # Example
///
/// ```
/// use metrics_utils::metric_attributes_vec;
///
/// let attrs = metric_attributes_vec!(("method", "GET"));
/// assert_eq!(attrs.len(), 1);
/// ```
///
/// Combine with `metric_attributes!` using `extend_from_slice`:
///
/// ```
/// use metrics_utils::{metric_attributes, metric_attributes_vec};
///
/// let mut attrs = metric_attributes_vec!(("method", "GET"));
/// attrs.extend_from_slice(metric_attributes!(("status", "200")));
/// assert_eq!(attrs.len(), 2);
/// ```
#[macro_export]
macro_rules! metric_attributes_vec {
    ($(($key:expr, $value:expr $(,)?)),+ $(,)?) => {
        vec![
            $(
                $crate::opentelemetry::KeyValue::new($key, $value)
            ),+
        ]
    };
}
