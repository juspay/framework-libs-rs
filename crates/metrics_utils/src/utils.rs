/// Returns a sensible set of default histogram bucket boundaries for f64 histograms.
///
/// Buckets start at `0.000_001` and double 29 times, yielding 30 boundaries.
///
/// # Example
///
/// ```
/// use metrics_utils::f64_histogram_buckets;
///
/// let buckets = f64_histogram_buckets();
/// assert_eq!(buckets[0], 0.000_001);
/// assert_eq!(buckets.len(), 30);
/// ```
///
/// Consumers may convert to owned buckets if needed:
///
/// ```
/// use metrics_utils::f64_histogram_buckets;
///
/// let owned: Vec<f64> = f64_histogram_buckets().to_vec();
/// assert_eq!(owned.len(), 30);
/// ```
#[inline]
pub const fn f64_histogram_buckets() -> &'static [f64] {
    const NUM_BOUNDARIES: usize = 30;

    const BUCKETS: [f64; NUM_BOUNDARIES] = {
        let mut buckets = [0.0f64; NUM_BOUNDARIES];
        let mut index = 0;
        let mut value = 0.000_001;

        #[expect(
            clippy::indexing_slicing,
            reason = "Array index guarded by loop condition `index < NUM_BOUNDARIES`; \
                array length equals the same const"
        )]
        while index < NUM_BOUNDARIES {
            buckets[index] = value;
            value = value * 2.0;
            index += 1;
        }

        buckets
    };

    &BUCKETS
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn buckets_are_monotonically_increasing() {
        let buckets = f64_histogram_buckets();
        for i in 1..buckets.len() {
            assert!(
                buckets[i] > buckets[i - 1],
                "buckets[{i}] = {} <= buckets[{}] = {}",
                buckets[i],
                i - 1,
                buckets[i - 1]
            );
        }
    }
}
