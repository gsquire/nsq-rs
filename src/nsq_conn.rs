//! The `nsq_conn` module helps maintain a connection to an `nsqd` instance handling
//! metadata internally.

use std::net::TcpStream;

use tokio_core::reactor::Handle;

/// `NsqConn` represents a connection to an nsqd instance with additional metadata
/// handling.
#[derive(Debug)]
pub struct NsqConn {
    socket: TcpStream,
    el_handle: Handle,
}
