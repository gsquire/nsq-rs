//! The configuration module represents the various options that an NSQ consumer
//! can contain.
#![deny(missing_docs)]

/// NsqConfig contains values supported by `nsqd` for consumers.
#[derive(Debug)]
pub struct NsqConfig {
    max_in_flight: usize,
}

impl NsqConfig {
    /// Return a default NsqConfig with sensible defaults.
    pub fn new() -> NsqConfig {
        NsqConfig {
            max_in_flight: 1,
        }
    }

    /// Set the maximum number of messages a consumer can handle before `nsqd`
    /// expects a response.
    pub fn max_in_fligh<'a>(&'a mut self, mif: usize) -> &'a mut NsqConfig {
        self.max_in_flight = mif;
        self
    }
}
