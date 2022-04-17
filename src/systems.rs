use std::borrow::Borrow;
use std::{
    io,
    io::{ Error, ErrorKind},
    net::{SocketAddr, UdpSocket},
};

use bevy::prelude::*;

use bytes::Bytes;

use bytes::Buf;

use super::HeartbeatTimer;

use super::{events::NetworkEvent, transport::Transport, NetworkResource};

// Uses for unpacking payload. Accoring to the first byte returns NetworkEvent enum.
fn uncover(address: SocketAddr, mut payload: Bytes) -> NetworkEvent {
    if payload.len() < 1 {
        return NetworkEvent::RecvError(Error::new(ErrorKind::Other, "leng is not satisfied message type"));
    }
    //println!("payload: {:?}", payload);
    let key = payload.split_to(1).get_i8();

    match key {
        109 => NetworkEvent::CliEvent(address, payload), // Eq to "m" or "main" / soon in tables file.
        105 => NetworkEvent::GetId(address, payload),
        _ => {
            let err_msg = format!( "unknown message type: {}", key);
            NetworkEvent::RecvError(Error::new(ErrorKind::Other, err_msg))
        }
    }
}

pub fn client_recv_packet_system(socket: Res<UdpSocket>, mut events: EventWriter<NetworkEvent>) {
    loop {
        let mut buf = [0; 512];
        match socket.recv_from(&mut buf) {
            Ok((recv_len, address)) => {
                let mut payload = Bytes::copy_from_slice(&buf[..recv_len]);
                if payload.len() == 0 {
                    debug!("{}: received heartbeat packet", address);
                    // discard without sending a NetworkEvent
                    continue;
                }
                debug!("received payload {:?} from {}", payload, address);
                events.send(uncover(address, payload));
            }
            Err(e) => {
                if e.kind() != io::ErrorKind::WouldBlock {
                    events.send(NetworkEvent::RecvError(e));
                }
                // break loop when no messages are left to read this frame
                break;
            }
        }
    }
}

pub fn server_recv_packet_system(
    time: Res<Time>,
    socket: Res<UdpSocket>,
    mut events: EventWriter<NetworkEvent>,
    mut net: ResMut<NetworkResource>,
) {
    loop {
        let mut buf = [0; 512];
        match socket.recv_from(&mut buf) {
            Ok((recv_len, address)) => {
                let mut payload = Bytes::copy_from_slice(&buf[..recv_len]);
                if net
                    .connections
                    .insert(address, time.time_since_startup())
                    .is_none()
                {
                    // connection established
                    events.send(NetworkEvent::Connected(address));
                }
                if payload.len() == 0 {
                    debug!("{}: received heartbeat packet", address);
                    // discard without sending a NetworkEvent
                    continue;
                } else {
                
                //println!("received payload {:?} from {}", payload, address);
                //println!("payloaaaaad: {:?}", String::from_utf8_lossy(payload));
                events.send(uncover(address, payload))
                }
                //println!("sliced: {:?}", payload.split_off(1));
            }
            Err(e) => {
                if e.kind() != io::ErrorKind::WouldBlock {
                    events.send(NetworkEvent::RecvError(e));
                }
                // break loop when no messages are left to read this frame
                break;
            }
        }
    }
}

pub fn send_packet_system(
    socket: Res<UdpSocket>,
    mut events: EventWriter<NetworkEvent>,
    mut transport: ResMut<Transport>,
) {
    let messages = transport.drain_messages_to_send(|_| true);
    for message in messages {
        if let Err(e) = socket.send_to(&message.payload, message.destination) {
            events.send(NetworkEvent::SendError(e, message))
        }
    }
}

pub fn idle_timeout_system(
    time: Res<Time>,
    mut net: ResMut<NetworkResource>,
    mut events: EventWriter<NetworkEvent>,
) {
    let idle_timeout = net.idle_timeout.clone();
    net.connections.retain(|addr, last_update| {
        let reached_idle_timeout = time.time_since_startup() - *last_update > idle_timeout;
        if reached_idle_timeout {
            events.send(NetworkEvent::Disconnected(*addr));
        }
        !reached_idle_timeout
    });
}

pub fn auto_heartbeat_system(
    time: Res<Time>,
    mut timer: ResMut<HeartbeatTimer>,
    remote_addr: Res<SocketAddr>,
    mut transport: ResMut<Transport>,
) {
    if timer.0.tick(time.delta()).just_finished() {
        transport.send("heartbeat", *remote_addr, Default::default());
    }
}
