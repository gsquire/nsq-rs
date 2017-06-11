//! The configuration module represents the various options that an NSQ consumer
//! can contain.

/// NsqConfig contains values supported by `nsqd` for consumers.
#[derive(Builder, Debug)]
pub struct NsqConfig {
    max_in_flight: usize,
}

impl Default for NsqConfig {
    fn default() -> NsqConfig {
        NsqConfig { max_in_flight: 1 }
    }
}

// Get style functions.
impl NsqConfig {
    /// Return the current max in flight value.
    pub fn max_in_flight(&self) -> usize {
        self.max_in_flight
    }
}
