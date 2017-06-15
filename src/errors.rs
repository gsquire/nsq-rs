//! This module contains the custom error types.
//! TODO: Investigate better error generation using error_chain.
#![deny(missing_docs)]

use std::io;

/// `NsqError` is a variant of errors that can happen when interacting with an
/// nsqd instance.
#[derive(Debug)]
pub enum NsqError {
    /// `Io` wraps an `std::io::Error`.
    Io(io::Error),

    /// `InvalidConn` is used when there is no connection to an nsqd instance.
    InvalidConn,
}

/// `NsqResult` wraps an `NsqError` with another type T.
pub type NsqResult<T> = Result<T, NsqError>;
