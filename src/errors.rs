//! This module contains the custom error types.

use std::io;

/// `NsqError` is a variant of errors that can happen when interacting with an
/// nsqd instance.
pub enum NsqError {
    Io(io::Error),
}

/// `NsqResult` wraps an `NsqError` with another type T.
pub type NsqResult<T> = Result<T, NsqError>;
