use std::{
    io,
    net::{SocketAddr, UdpSocket},
};

use bevy::prelude::*;
use bytes::Bytes;

use super::{events::NetworkEvent, time::NetworkTime, transport::Transport, NetworkResource};

/// This system is used exclusively to update the state of the `NetworkTime` resource.
pub fn network_time_system(time: Res<Time>, mut net_time: ResMut<NetworkTime>) {
    net_time.update_elapsed(time.delta());
    net_time.reset_frame_lag();
    while net_time.elapsed_duration() > net_time.per_frame_duration() {
        net_time.increment_frame_number();
    }
}

pub fn network_recv_system(
    time: Res<Time>,
    net_time: Res<NetworkTime>,
    socket: Res<UdpSocket>,
    mut events: EventWriter<NetworkEvent>,
    mut net: ResMut<NetworkResource>,
) {
    for frame in net_time.network_frames_to_run() {
        loop {
            let mut buf = [0; 32];
            match socket.recv_from(&mut buf) {
                Ok((recv_len, address)) => {
                    let payload = Bytes::copy_from_slice(&buf[..recv_len]);
                    if net
                        .connections
                        .insert(address, time.time_since_startup())
                        .is_none()
                    {
                        // connection established
                        events.send(NetworkEvent::Connected(address));
                    }
                    if payload.len() == 0 {
                        debug!("Received heartbeat packet");
                        // discard without sending a NetworkEvent
                        continue;
                    }
                    let event = NetworkEvent::Message(address, payload);
                    debug!("frame {}: sending event {:?}", frame, buf);
                    events.send(event);
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
}

pub fn network_send_system(
    net_time: Res<NetworkTime>,
    socket: Res<UdpSocket>,
    mut events: EventWriter<NetworkEvent>,
    mut transport: ResMut<Transport>,
) {
    let messages = transport.drain_messages_to_send(|_| net_time.should_send_message_now());
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
    net_time: Res<NetworkTime>,
    remote_addr: Res<SocketAddr>,
    mut transport: ResMut<Transport>,
) {
    for _frame in net_time.network_frames_to_run() {
        transport.send(*remote_addr, Default::default());
    }
}
