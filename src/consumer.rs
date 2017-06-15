//! The consumer module contains a type that can connect to an `nsqd` instance
//! to receive messages from and handle accordingly.
#![deny(missing_docs)]

use std::net::ToSocketAddrs;

use byteorder::{BigEndian, ByteOrder};
use futures::{Future, Stream};
use tokio_core::net::TcpStream as TokioTcp;
use tokio_io::AsyncRead;
use tokio_io::codec::{FramedWrite, length_delimited};
use tokio_io::io::write_all;

use config::NsqConfig;
use errors::{NsqResult, NsqError};
use message::{Handler, MessageBuilder, MessageReply, NsqResponder};
use nsq_conn::NsqConn;

/// `Consumer` represents a long-lived connection to an nsqd instance that can read messages
/// and reply depending on a handler function.
pub struct Consumer {
    channel: String,
    config: NsqConfig,
    conn: Option<NsqConn>,
    handler: Option<Box<Handler>>,
    topic: String,
}

// Public methods.
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
    pub fn add_handler<H>(&mut self, handler: H)
    where
        H: Handler + 'static,
    {
        self.handler = Some(Box::new(handler));
    }

    /// Start consuming from nsqd by initiating an event loop. This function moves the consumer
    /// to take ownership of the internal connection.
    pub fn begin_consuming(self) -> NsqResult<()> {
        match self.conn {
            Some(_) => {
                self.read_loop();
                Ok(())
            }
            None => Err(NsqError::InvalidConn),
        }
    }
}

// Private methods.
impl Consumer {
    // TODO: I want to split this into more functions.
    fn read_loop(self) {
        // This only gets called if the connection is valid.
        let mut conn = self.conn.unwrap();
        let sock_clone = conn.socket.try_clone().expect("cloning TCP socket");

        // The socket used to stream events in.
        let stream_sock = TokioTcp::from_stream(conn.socket, &conn.event_loop.handle()).unwrap();
        let (stream_read, stream_write) = stream_sock.split();

        // The socket for our framed writer that will handle message finishing or re-queuing.
        let framed_sock = TokioTcp::from_stream(sock_clone, &conn.event_loop.handle()).unwrap();

        // Write out the config if there are any values and subscribe to a channel and topic.
        let subscribe = format!("SUB {} {}\n", self.topic, self.channel);
        let ready_count = format!("RDY {}\n", self.config.max_in_flight());

        let prelude = write_all(stream_write, b"  V2")
            .and_then(|(stream, _)| write_all(stream, subscribe.as_bytes()))
            .and_then(|(stream, _)| write_all(stream, ready_count.as_bytes()));

        let framed_writer = FramedWrite::new(framed_sock, NsqResponder::default());
        let framed_read = length_delimited::Builder::new()
            .length_field_length(4)
            .new_read(stream_read);
        let handler = self.handler;
        let reader = framed_read
            .map(|mut buf| {
                let frame_type = BigEndian::read_i32(buf.as_ref());
                // Ditch the frame type.
                buf.split_to(4);
                let mut response = MessageReply::Nop;

                // TODO: Handle other frame types and consider constants for reading the bytes.
                if frame_type == 2 {
                    let time_bytes = buf.split_to(8);
                    let time = BigEndian::read_i64(time_bytes.as_ref());

                    let attempt_bytes = buf.split_to(2);
                    let attempts = BigEndian::read_u16(attempt_bytes.as_ref());

                    let id = buf.split_to(16);
                    let message = MessageBuilder::default()
                        .timestamp(time)
                        .attempts(attempts)
                        .id(id)
                        .body(buf)
                        .build()
                        .unwrap();
                    match handler {
                        Some(ref h) => response = h.handle_message(&message),
                        None => {}
                    }
                }

                response
            })
            .forward(framed_writer);

        conn.event_loop.run(prelude.join(reader)).unwrap();
    }
}
