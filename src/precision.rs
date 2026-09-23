#[cfg(not(any(target_arch = "wasm32", target_arch = "wasm64")))]
use std::thread;
#[cfg(not(any(target_arch = "wasm32", target_arch = "wasm64")))]
use std::time::{Duration, Instant};

use super::config::*;
use super::cpucounter::*;
use super::timestamp::*;

#[derive(Clone)]
pub struct Precision {
    pub(crate) frequency: Option<u64>,
}

impl Precision {
    /// Initialize the crate. Note that on Linux system, this will
    /// perform calibration before returning. You may want to do this
    /// only twice. The `Precision` value can then be cloned if needed.
    pub fn new(config: Config) -> Result<Self, &'static str> {
        let frequency = if config.wall_time {
            Some(Precision::guess_frequency(&config)?)
        } else {
            None
        };
        Ok(Precision { frequency })
    }

    /// Returns the current timestamp
    #[inline]
    pub fn now(&self) -> Timestamp {
        CPUCounter::current()
    }

    #[cfg(target_os = "macos")]
    fn guess_frequency(config: &Config) -> Result<u64, &'static str> {
        Self::guess_frequency_using_sysctl("machdep.tsc.frequency")
            .or_else(|_| Self::guess_frequency_with_wall_clock(config.setup_duration))
    }

    #[cfg(target_os = "freebsd")]
    fn guess_frequency(config: &Config) -> Result<u64, &'static str> {
        Self::guess_frequency_using_sysctl("machdep.tsc_freq")
            .or_else(|_| Self::guess_frequency_with_wall_clock(config.setup_duration))
    }

    #[cfg(any(target_os = "wasi", target_os = "wasix"))]
    fn guess_frequency(_config: &Config) -> Result<u64, &'static str> {
        Ok(1_000_000_000)
    }

    #[cfg(all(
        any(target_arch = "wasm32", target_arch = "wasm64"),
        target_os = "unknown"
    ))]
    fn guess_frequency(_config: &Config) -> Result<u64, &'static str> {
        Ok(1_000_000_000)
    }

    #[cfg(not(any(
        target_os = "macos",
        target_os = "freebsd",
        any(target_arch = "wasm32", target_arch = "wasm64")
    )))]
    fn guess_frequency(config: &Config) -> Result<u64, &'static str> {
        Self::guess_frequency_with_wall_clock(config.setup_duration)
    }

    #[cfg(any(target_os = "macos", target_os = "freebsd"))]
    fn guess_frequency_using_sysctl(name: &str) -> Result<u64, &'static str> {
        use std::ffi::CString;
        use std::mem;
        use std::ptr;

        use libc::{c_long, size_t};

        let sysctl_name = CString::new(name).map_err(|_| "invalid sysctl name")?;
        let mut result: c_long = 0;
        let mut result_len: size_t = mem::size_of::<c_long>() as _;
        if unsafe {
            libc::sysctlbyname(
                sysctl_name.as_ptr(),
                &mut result as *mut _ as _,
                &mut result_len as *mut _,
                ptr::null_mut(),
                0,
            )
        } != 0
            || result_len != mem::size_of::<c_long>()
            || result <= 0
        {
            return Err("sysctl() failed");
        }
        Ok(result as u64)
    }

    #[cfg(not(any(target_arch = "wasm32", target_arch = "wasm64")))]
    fn guess_frequency_with_wall_clock(setup_duration: Duration) -> Result<u64, &'static str> {
        let setup_duration = std::cmp::max(Duration::from_secs(1), setup_duration);
        let wall_start = Instant::now();
        let start = CPUCounter::current();
        thread::sleep(setup_duration);
        let stop = CPUCounter::current();
        let wall_elapsed = wall_start.elapsed();
        let elapsed = stop - start;
        let frequency = (elapsed.ticks() as u128 * 1_000_000_000) / wall_elapsed.as_nanos();
        if frequency == 0 || frequency > u64::MAX as u128 {
            return Err("invalid hardware counter frequency");
        }
        Ok(frequency as u64)
    }
}

#[cfg(all(test, not(any(target_arch = "wasm32", target_arch = "wasm64"))))]
mod tests {
    use super::*;
    use std::time::Instant;

    #[test]
    fn calibration_accounts_for_fractional_seconds() {
        let frequency =
            Precision::guess_frequency_with_wall_clock(Duration::from_millis(1_500)).unwrap();
        let precision = Precision {
            frequency: Some(frequency),
        };
        let wall_start = Instant::now();
        let start = precision.now();
        thread::sleep(Duration::from_millis(200));
        let elapsed = precision.now() - start;
        let wall_elapsed = wall_start.elapsed().as_secs_f64();
        let ratio = elapsed.as_secs_f64(&precision) / wall_elapsed;
        assert!((0.8..1.2).contains(&ratio), "measured/wall time: {}", ratio);
    }
}
