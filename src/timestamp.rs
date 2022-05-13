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
    #[inline]
    pub fn as_secs(&self, precision: &Precision) -> u64 {
        self.0 / precision.frequency
    }

    /// Returns the duration as a number of seconds (floating-point)
    #[inline]
    pub fn as_secs_f64(&self, precision: &Precision) -> f64 {
        self.0 as f64 / precision.frequency as f64
    }

    /// Returns the duration as milliseconds
    #[inline]
    pub fn as_millis(&self, precision: &Precision) -> u64 {
        self.0 * 1_000 / precision.frequency
    }

    /// Returns the duration as nanoseconds
    #[inline]
    pub fn as_ns(&self, precision: &Precision) -> u64 {
        (((self.0 as f64 * 1_000.0) / precision.frequency as f64) * 1_000_000.0) as u64
    }
}
