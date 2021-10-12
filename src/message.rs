use std::net::SocketAddr;

use bytes::Bytes;

pub struct Message {
    /// The destination to send the message.
    pub destination: SocketAddr,
    /// The serialized payload itself.
    pub payload: Bytes,
}

impl Message {
    /// Creates and returns a new Message.
    pub(crate) fn new(destination: SocketAddr, payload: &[u8]) -> Self {
        Self {
            destination,
            payload: Bytes::copy_from_slice(payload),
        }
    }
}
