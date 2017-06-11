//! The message module contains a type that an nsqd instance writes on the wire for
//! a single message.
use std::io;

use bytes::{BufMut, BytesMut};
use tokio_io::codec::Encoder;

/// `Message` represents a message on the wire.
#[derive(Builder, Debug)]
pub struct Message {
    id: BytesMut,
    body: BytesMut,
    timestamp: i64,
    attempts: u16,
}

/// `MessageReply` tells nsqd what to do with the message that was previously consumed.
pub enum MessageReply {
    /// `Fin` tells the consumer to finish a message.
    Fin(BytesMut),
    /// `Req` tells the consumer to requeue a message.
    Req(BytesMut),
    /// `Touch` tells the consumer to touch the message updating the TTL.
    Touch(BytesMut),
    /// `Nop` is typically used only for heartbeats.
    Nop,
}

/// Handler is a trait that a type must implement to handle messages from a consumer.
pub trait Handler {
    /// This function is invoked when a `Consumer` receives a message.
    fn handle_message(&self, message: &Message) -> MessageReply;
}

/// NsqResponder is used to write back to nsqd.
#[derive(Default)]
pub struct NsqResponder;

impl Encoder for NsqResponder {
    type Item = MessageReply;
    type Error = io::Error;

    fn encode(&mut self, item: Self::Item, dst: &mut BytesMut) -> Result<(), Self::Error> {
        match item {
            MessageReply::Nop => dst.put("NOP\n"),
            _ => {}
        }
        Ok(())
    }
}
