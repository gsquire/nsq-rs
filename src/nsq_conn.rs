//! The `nsq_conn` module helps maintain a connection to an `nsqd` instance handling
//! metadata internally.
#![deny(missing_docs)]

use std::net::{TcpStream, ToSocketAddrs};

use tokio_core::reactor::Core;

use errors::{NsqError, NsqResult};

/// `NsqConn` represents a connection to an nsqd instance with additional metadata
/// handling.
#[derive(Debug)]
pub struct NsqConn {
    /// The TCP connection to nsqd.
    pub socket: TcpStream,
    /// The event loop created from tokio.
    pub event_loop: Core,
}

impl NsqConn {
    /// Create a new connection to an nsqd instance.
    pub fn new<A: ToSocketAddrs>(addr: A) -> NsqResult<NsqConn> {
        let core = Core::new().unwrap();

        let tcp = TcpStream::connect(addr);
        match tcp {
            Ok(t) => { Ok(NsqConn { socket: t, event_loop: core }) },
            Err(e) => { Err(NsqError::Io(e)) },
        }
    }
}
