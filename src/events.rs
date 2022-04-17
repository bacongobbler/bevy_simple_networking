use std::{io, net::SocketAddr};

use bytes::Bytes;

use super::message::Message;

pub enum NetworkEvent {
    // A message was received from a client
    Message(SocketAddr, Bytes),
    // Some invisible stuff: server Pong, etc
    Background(SocketAddr),
    // An client side event
    CliEvent(SocketAddr, Bytes),
    // Uses only on client side.
    GetId(SocketAddr, Bytes),
    // For chatting
    Chat(SocketAddr, Bytes),
    // A new client has connected to us
    Connected(SocketAddr),
    // A client has disconnected from us
    Disconnected(SocketAddr),
    // An error occurred while receiving a message
    RecvError(io::Error),
    // An error occurred while sending a message
    SendError(io::Error, Message),
}
