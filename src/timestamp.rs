use std::ops::*;

use super::precision::*;

/// A timestamp. Note that this is an opaque structure.
#[derive(Clone, Copy, Eq, PartialEq)]
pub struct Timestamp(pub(crate) u64);

/// The difference between two timestamps.
#[derive(Clone, Copy, Default, Eq, PartialEq, Ord, PartialOrd)]
pub struct Elapsed(u64);

impl Sub for Timestamp {
    type Output = Elapsed;

    #[inline]
    fn sub(self, ts: Timestamp) -> Self::Output {
        if self.0 >= ts.0 {
            Elapsed(self.0 - ts.0)
        } else {
            Elapsed(self.0 + (!ts.0) + 1)
        }
    }
}

impl Add for Elapsed {
    type Output = Elapsed;

    #[inline]
    fn add(self, other: Elapsed) -> Self::Output {
        Elapsed(self.0 + other.0)
    }
}

impl AddAssign for Elapsed {
    #[inline]
    fn add_assign(&mut self, other: Elapsed) {
        self.0 += other.0;
    }
}

impl Elapsed {
    /// Returns a nul duration
    #[inline]
    pub fn new() -> Self {
        Elapsed::default()
    }

    /// Builds a `Duration` from a number of ticks
    #[inline]
    pub fn from_ticks(ticks: u64) -> Self {
        Elapsed(ticks)
    }

    /// Returns the number of ticks for the given duration
    #[inline]
    pub fn ticks(&self) -> u64 {
        self.0
    }

    /// Returns the duration as a number of seconds
    ///
    /// # Panics
    ///
    /// Panics if the given `Precision` was not configured to measure wall time.
    #[inline]
    pub fn as_secs(&self, precision: &Precision) -> u64 {
        self.0
            / precision
                .frequency
                .expect("`Precision` must have been configured to measure wall time")
    }

    /// Returns the duration as a number of seconds (floating-point)
    ///
    /// # Panics
    ///
    /// Panics if the given `Precision` was not configured to measure wall time.
    #[inline]
    pub fn as_secs_f64(&self, precision: &Precision) -> f64 {
        self.0 as f64
            / precision
                .frequency
                .expect("`Precision` must have been configured to measure wall time")
                as f64
    }

    /// Returns the duration as milliseconds
    ///
    /// Saturates at `u64::MAX` if the duration cannot be represented.
    ///
    /// # Panics
    ///
    /// Panics if the given `Precision` was not configured to measure wall time.
    #[inline]
    pub fn as_millis(&self, precision: &Precision) -> u64 {
        let millis = self.0 as u128 * 1_000
            / precision
                .frequency
                .expect("`Precision` must have been configured to measure wall time")
                as u128;
        millis.min(u64::MAX as u128) as u64
    }

    /// Returns the duration as nanoseconds
    ///
    /// Saturates at `u64::MAX` if the duration cannot be represented.
    ///
    /// # Panics
    ///
    /// Panics if the given `Precision` was not configured to measure wall time.
    #[inline]
    pub fn as_ns(&self, precision: &Precision) -> u64 {
        let nanos = self.0 as u128 * 1_000_000_000
            / precision
                .frequency
                .expect("`Precision` must have been configured to measure wall time")
                as u128;
        nanos.min(u64::MAX as u128) as u64
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn milliseconds_do_not_overflow_intermediate_product() {
        let precision = Precision {
            frequency: Some(1_000_000_000),
        };
        assert_eq!(
            Elapsed::from_ticks(u64::MAX).as_millis(&precision),
            u64::MAX / 1_000_000
        );
    }

    #[test]
    fn nanoseconds_preserve_integer_precision() {
        let precision = Precision {
            frequency: Some(1_000_000_000),
        };
        let ticks = (1_u64 << 53) + 1;
        assert_eq!(Elapsed::from_ticks(ticks).as_ns(&precision), ticks);
    }

    #[test]
    fn conversions_truncate_fractional_units() {
        let precision = Precision { frequency: Some(3) };
        let elapsed = Elapsed::from_ticks(2);
        assert_eq!(elapsed.as_millis(&precision), 666);
        assert_eq!(elapsed.as_ns(&precision), 666_666_666);
        assert_eq!(Elapsed::new().as_millis(&precision), 0);
        assert_eq!(Elapsed::new().as_ns(&precision), 0);
    }

    #[test]
    fn conversions_saturate_when_result_does_not_fit() {
        let precision = Precision { frequency: Some(1) };
        let elapsed = Elapsed::from_ticks(u64::MAX);
        assert_eq!(elapsed.as_millis(&precision), u64::MAX);
        assert_eq!(elapsed.as_ns(&precision), u64::MAX);
    }

    #[test]
    #[should_panic(expected = "configured to measure wall time")]
    fn milliseconds_require_wall_time() {
        Elapsed::new().as_millis(&Precision { frequency: None });
    }

    #[test]
    #[should_panic(expected = "configured to measure wall time")]
    fn nanoseconds_require_wall_time() {
        Elapsed::new().as_ns(&Precision { frequency: None });
    }
}
