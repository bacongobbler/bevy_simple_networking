use std::net::SocketAddr;

use bytes::Bytes;

use super::urgency::Urgency;

pub struct Message {
    /// The destination to send the message.
    pub destination: SocketAddr,
    /// The serialized payload itself.
    pub payload: Bytes,
    /// The requirement around when this message should be sent.
    pub urgency: Urgency,
}

impl Message {
    /// Creates and returns a new Message.
    pub(crate) fn new(destination: SocketAddr, payload: &[u8], urgency: Urgency) -> Self {
        Self {
            destination,
            payload: Bytes::copy_from_slice(payload),
            urgency,
        }
    }
}
