//! The consumer module contains a type that can connect to an `nsqd` instance
//! to receive messages from and handle accordingly.

use std::net::ToSocketAddrs;

use config::NsqConfig;
use nsq_conn::NsqConn;

/// `Consumer` represents a long-lived connection to an nsqd instance that can read messages
/// and reply depending on a handler function.
#[derive(Debug)]
pub struct Consumer {
    conn: Option<NsqConn>,
    config: NsqConfig,
    channel: String,
    topic: String,
}

impl Consumer {
    /// Given a topic, channel, and config, set up a new Consumer instance.
    pub fn new(topic: &str, channel: &str, config: NsqConfig) -> Consumer {
        Consumer {
            topic: topic.to_owned(),
            channel: channel.to_owned(),
            config: config,
            conn: None, 
        }
    }

    /// Connect to a single nsqd instance supplied with a host and port.
    pub fn connect_to_nsqd<A: ToSocketAddrs>(&mut self, addr: A) {
        // TODO: Return custom error type here and set up connection.
    }
}
