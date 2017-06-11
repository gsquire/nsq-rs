/// This crate offers a high-level API for the `nsq` daemon.
#[macro_use]
extern crate derive_builder;

extern crate byteorder;
extern crate bytes;
extern crate futures;
extern crate tokio_core;
extern crate tokio_io;

pub mod config;
pub mod consumer;
pub mod errors;
pub mod message;
pub mod nsq_conn;
