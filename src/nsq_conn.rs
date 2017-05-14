//! The `nsq_conn` module helps maintain a connection to an `nsqd` instance handling
//! metadata internally.

use std::net::{TcpStream, ToSocketAddrs};

use tokio_core::reactor::{Core, Handle};

use errors::{NsqError, NsqResult};

/// `NsqConn` represents a connection to an nsqd instance with additional metadata
/// handling.
#[derive(Debug)]
pub struct NsqConn {
    socket: TcpStream,
    el_handle: Handle,
}

impl NsqConn {
    /// Create a new connection to an nsqd instance.
    pub fn new<A: ToSocketAddrs>(addr: A) -> NsqResult<NsqConn> {
        let core = Core::new().unwrap();
        let handle = core.handle();

        let tcp = TcpStream::connect(addr);
        match tcp {
            Ok(t) => { Ok(NsqConn { socket: t, el_handle: handle }) },
            Err(e) => { Err(NsqError::Io(e)) },
        }
    }
}
