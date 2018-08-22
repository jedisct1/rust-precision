use std::time::Duration;

#[derive(Clone)]
pub struct Config {
    pub(crate) setup_duration: Duration,
}

impl Default for Config {
    fn default() -> Self {
        Config {
            setup_duration: Duration::from_secs(5),
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
}
