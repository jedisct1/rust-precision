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
    pub fn setup_duration(mut self, setup_duration: Duration) -> Self {
        self.setup_duration = setup_duration;
        self
    }
}
