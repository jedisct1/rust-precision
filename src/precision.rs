use super::config::*;
use super::cpucounter::*;
use super::timestamp::*;
#[cfg(not(any(target_arch = "wasm32", target_arch = "wasm64")))]
use std::thread;
#[cfg(not(any(target_arch = "wasm32", target_arch = "wasm64")))]
use std::time::Duration;

#[derive(Clone)]
pub struct Precision {
    pub(crate) frequency: u64,
}

impl Precision {
    /// Initialize the crate. Note that on Linux system, this will
    /// perform calibration before returning. You may want to do this
    /// only twice. The `Precision` value can then be cloned if needed.
    pub fn new(config: Config) -> Result<Self, &'static str> {
        let frequency = Precision::guess_frequency(&config)?;
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

    #[cfg(target_os = "wasi")]
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
        use libc::{c_long, size_t};
        use std::ffi::CString;
        use std::mem;
        use std::ptr;

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
        let start = CPUCounter::current();
        thread::sleep(setup_duration);
        let stop = CPUCounter::current();
        let elapsed = stop - start;
        Ok(elapsed.ticks() / setup_duration.as_secs())
    }
}
