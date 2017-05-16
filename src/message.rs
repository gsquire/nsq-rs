//! The message module contains a type that an nsqd instance writes on the wire for
//! a single message.
#![deny(missing_docs)]

const ID_LENGTH: usize = 16;

/// `Message` represents a message on the wire.
#[derive(Debug, Default)]
pub struct Message<'a> {
    id: [u8; ID_LENGTH],
    body: &'a[u8],
    timestamp: i64,
    attempts: u16,
}

/// `MessageReply` tells nsqd what to do with the message that was previously consumed.
pub enum MessageReply {
    /// `Fin` tells the consumer to finish a message.
    Fin,
    /// `Req` tells the consumer to requeue a message.
    Req,
    /// `Touch` tells the consumer to touch the message updating the TTL.
    Touch,
}

/// Handler is a trait that a type must implement to handle messages from a consumer.
pub trait Handler<'a> {
    /// This function is invoked when a `Consumer` receives a message.
    fn handle_message(&self, message: &Message<'a>) -> MessageReply;
}
