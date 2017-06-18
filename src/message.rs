//! The message module contains a type that an nsqd instance writes on the wire for
//! a single message.
use std::io;

use bytes::{BufMut, BytesMut};
use tokio_io::codec::Encoder;

const BUFFER_GROW: usize = 1024;
const SPACE_NEEDED: usize = 32;

/// `Message` represents a message on the wire.
#[derive(Builder, Debug)]
pub struct Message {
    id: BytesMut,
    body: BytesMut,
    timestamp: i64,
    attempts: u16,
}

impl Message {
    /// The unique message ID which is used to reply back to nsqd.
    pub fn id(&self) -> BytesMut {
        self.id.clone()
    }

    /// The body of the message.
    pub fn body(&self) -> BytesMut {
        self.body.clone()
    }

    /// This is the timestamp of the message.
    pub fn timestamp(&self) -> i64 {
        self.timestamp
    }

    /// Return the number of attempts that nsqd made to deliver the message.
    pub fn attempts(&self) -> u16 {
        self.attempts
    }
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

fn put_nop(dst: &mut BytesMut) {
    dst.put("NOP\n");
}

fn put_fin(dst: &mut BytesMut, id: &BytesMut) {
    dst.put("FIN ");
    dst.put(id);
    dst.put("\n");
}

fn put_req(dst: &mut BytesMut, id: &BytesMut) {
    dst.put("REQ ");
    dst.put(id);
    dst.put("\n");
}

fn put_touch(dst: &mut BytesMut, id: &BytesMut) {
    dst.put("TOUCH ");
    dst.put(id);
    dst.put("\n");
}

/// NsqResponder is used to write back to nsqd.
#[derive(Default)]
pub struct NsqResponder;

impl Encoder for NsqResponder {
    type Item = MessageReply;
    type Error = io::Error;

    fn encode(&mut self, item: Self::Item, dst: &mut BytesMut) -> Result<(), Self::Error> {
        if dst.remaining_mut() < SPACE_NEEDED {
            dst.reserve(BUFFER_GROW);
        }
        match item {
            MessageReply::Nop => put_nop(dst),
            MessageReply::Fin(ref id) => put_fin(dst, id),
            MessageReply::Req(ref id) => put_req(dst, id),
            MessageReply::Touch(ref id) => put_touch(dst, id),
        }
        Ok(())
    }
}
