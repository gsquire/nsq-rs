//! The consumer module contains a type that can connect to an `nsqd` instance
//! to receive messages from and handle accordingly.

use std::net::ToSocketAddrs;

use config::NsqConfig;
use errors::NsqResult;
use message::Handler;
use nsq_conn::NsqConn;

/// `Consumer` represents a long-lived connection to an nsqd instance that can read messages
/// and reply depending on a handler function.
pub struct Consumer {
    channel: String,
    config: NsqConfig,
    conn: Option<NsqConn>,
    handler: Option<Box<Handler<'static>>>,
    topic: String,
}

impl Consumer {
    /// Given a topic, channel, and config, set up a new Consumer instance.
    pub fn new(topic: &str, channel: &str, config: NsqConfig) -> Consumer {
        Consumer {
            channel: channel.to_owned(),
            config: config,
            conn: None,
            handler: None,
            topic: topic.to_owned(),
        }
    }

    /// Connect to a single nsqd instance supplied with a host and port.
    pub fn connect_to_nsqd<A: ToSocketAddrs>(&mut self, addr: A) -> NsqResult<()> {
        let conn = NsqConn::new(addr)?;
        self.conn = Some(conn);
        Ok(())
    }

    /// Add a handler for messages that are consumed.
    pub fn add_handler<H>(&mut self, handler: H) where H: Handler<'static> + 'static {
        self.handler = Some(Box::new(handler));
    }
}
