use std::time::Duration;

#[derive(Clone)]
pub struct Config {
    pub(crate) setup_duration: Duration,
    pub(crate) wall_time: bool,
}

impl Default for Config {
    fn default() -> Self {
        Config {
            setup_duration: Duration::from_secs(5),
            wall_time: true,
        }
    }
}

impl Config {
    /// Sets the duration of the calibration, required to estimate the frequency
    /// of the hardware counter on Linux. Default is 5 seconds.
    pub fn setup_duration(mut self, setup_duration: Duration) -> Self {
        self.setup_duration = setup_duration;
        self
    }

    /// Enable or disable measuring wall time.
    ///
    /// Default is `true`.
    ///
    /// If this is disabled, then you can only measure cycles, not
    /// seconds/milliseconds/nanoseconds/etc.
    pub fn wall_time(mut self, enable: bool) -> Self {
        self.wall_time = enable;
        self
    }
}
